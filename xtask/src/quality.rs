use crate::demo_builds;
use crate::utils::{path_from_repo, read_lines, repo_root, run_step, tracked_files};
use std::collections::BTreeMap;
use std::path::Path;

pub fn run() -> Result<(), String> {
    let root = repo_root();
    let tracked = tracked_files(&root)?;

    let mut failures = Vec::new();
    let mut warnings = Vec::new();

    test_generated_artifacts(&tracked, &mut failures);
    test_file_line_limits(&root, &tracked, &mut failures, &mut warnings)?;
    test_directory_flatness(&tracked, &mut failures);
    test_forbidden_patterns(&root, &tracked, &mut failures)?;
    test_context_docs(&root, &mut failures)?;
    test_naming_patterns(&tracked, &mut warnings);
    test_comment_heuristics(&root, &tracked, &mut warnings)?;

    if let Err(error) = run_step(
        "cargo fmt --all --check",
        "cargo",
        &["fmt", "--all", "--check"],
        &root,
    ) {
        failures.push(error);
    }
    if let Err(error) = run_step(
        "cargo clippy --workspace --all-targets --all-features -- -D warnings",
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
        &root,
    ) {
        failures.push(error);
    }
    if let Err(error) = run_step(
        "cargo test -p motif-core",
        "cargo",
        &["test", "-p", "motif-core"],
        &root,
    ) {
        failures.push(error);
    }
    if let Err(error) = demo_builds::run() {
        failures.push(error);
    }

    if !warnings.is_empty() {
        println!("\nSoft warnings:");
        for warning in warnings {
            println!("- {warning}");
        }
    }

    if failures.is_empty() {
        println!("\nquality checks passed");
        Ok(())
    } else {
        println!("\nHard gate failures:");
        for failure in failures {
            println!("- {failure}");
        }
        Err("quality checks failed".to_string())
    }
}

fn test_generated_artifacts(tracked: &[String], failures: &mut Vec<String>) {
    for file in tracked {
        if file.contains("/target/")
            || file.contains("/node_modules/")
            || file.contains("/dist/")
            || file.contains("/coverage/")
            || file.contains("/.vite/")
            || file.ends_with("/motif.css")
            || file.ends_with(".tsbuildinfo")
        {
            failures.push(format!("tracked generated artifact detected: {file}"));
        }
    }
}

fn test_file_line_limits(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    for file in tracked {
        let path = path_from_repo(root, file);
        if !path.is_file() {
            continue;
        }

        let limit = match file.as_str() {
            value if value.starts_with("core/src/") && value.ends_with(".rs") => Some(320),
            value if value.starts_with("core/tests/") && value.ends_with(".rs") => Some(420),
            value if value.starts_with("xtask/src/") && value.ends_with(".rs") => Some(320),
            value if value.starts_with("scripts/") && value.ends_with(".ps1") => Some(320),
            value if value.starts_with("agent/") && value.ends_with(".md") => Some(400),
            value if value.starts_with("tokens/") && value.ends_with(".json") => Some(220),
            value
                if value.starts_with(".github/")
                    && (value.ends_with(".yml")
                        || value.ends_with(".yaml")
                        || value.ends_with(".md")) =>
            {
                Some(260)
            }
            value
                if (value.starts_with("demo/") || value.starts_with("cases/"))
                    && (value.ends_with(".html")
                        || value.ends_with(".ts")
                        || value.ends_with(".tsx")
                        || value.ends_with(".vue")
                        || value.ends_with(".md")
                        || value.ends_with(".json")) =>
            {
                Some(260)
            }
            _ => None,
        };

        if let Some(limit) = limit {
            let lines = read_lines(&path)?.len();
            if lines > limit {
                failures.push(format!("{file} is {lines} lines, exceeds limit {limit}"));
            } else if file.starts_with("core/src/") && lines > 240 {
                warnings.push(format!("{file} is already large at {lines} lines"));
            }
        }
    }
    Ok(())
}

fn test_directory_flatness(tracked: &[String], failures: &mut Vec<String>) {
    let mut counts = BTreeMap::<String, usize>::new();
    for file in tracked {
        if file.contains("/target/")
            || file.contains("/node_modules/")
            || file.contains("/dist/")
            || file.contains("/coverage/")
            || file.contains("/.vite/")
        {
            continue;
        }
        let dir = file.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("");
        *counts.entry(dir.to_string()).or_default() += 1;
    }
    for (dir, count) in counts {
        if !dir.is_empty() && count > 12 {
            failures.push(format!(
                "directory {dir} has {count} tracked files; limit is 12"
            ));
        }
    }
}

fn test_forbidden_patterns(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
) -> Result<(), String> {
    let patterns = [
        ("unsafe", "unsafe is forbidden outside explicit review"),
        ("transmute", "transmute is forbidden"),
        ("MaybeUninit", "MaybeUninit is forbidden"),
        ("todo!(", "todo! is forbidden"),
        ("unimplemented!(", "unimplemented! is forbidden"),
        ("dbg!(", "dbg! is forbidden"),
        (".unwrap(", "unwrap() is forbidden in library code"),
        (".expect(", "expect() is forbidden in library code"),
        ("println!(", "println! is forbidden in library code"),
        ("eprintln!(", "eprintln! is forbidden in library code"),
    ];

    for file in tracked {
        if !(file.starts_with("core/src/") && file.ends_with(".rs") && file != "core/src/main.rs") {
            continue;
        }
        let content = std::fs::read_to_string(path_from_repo(root, file))
            .map_err(|error| format!("failed to read {file}: {error}"))?;
        for (needle, message) in patterns {
            if content.contains(needle) {
                failures.push(format!("{file}: {message}"));
            }
        }
    }
    Ok(())
}

fn test_context_docs(root: &Path, failures: &mut Vec<String>) -> Result<(), String> {
    let required = [
        "agent/product.md",
        "agent/quality.md",
        "agent/presets.md",
        "agent/scope.md",
        "agent/architecture.md",
        "agent/status.md",
        "agent/rules.md",
    ];
    let context = std::fs::read_to_string(path_from_repo(root, "agent/context.md"))
        .map_err(|error| format!("failed to read agent/context.md: {error}"))?;
    for doc in required {
        if !path_from_repo(root, doc).exists() {
            failures.push(format!("required doc missing: {doc}"));
        } else if !context.contains(doc) {
            failures.push(format!("agent/context.md is missing entry for {doc}"));
        }
    }
    Ok(())
}

fn test_naming_patterns(tracked: &[String], warnings: &mut Vec<String>) {
    let suspicious = [
        "helper",
        "util",
        "utils",
        "manager",
        "service",
        "handler",
        "processor",
        "temp",
        "misc",
        "final",
        "old",
        "new",
    ];
    let mut grouped: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for file in tracked {
        let dir = file.rsplit_once('/').map(|(dir, _)| dir).unwrap_or("");
        grouped
            .entry(dir.to_string())
            .or_default()
            .push(file.clone());
    }

    for (dir, files) in grouped {
        let mut prefixes = BTreeMap::<String, usize>::new();
        let mut suffixes = BTreeMap::<String, usize>::new();
        for file in files {
            let stem = file.rsplit('/').next().unwrap_or(&file);
            let stem = stem.split('.').next().unwrap_or(stem);
            let parts: Vec<&str> = stem.split(['_', '-']).collect();
            if parts.len() >= 2 {
                *prefixes.entry(parts[0].to_string()).or_default() += 1;
                *suffixes
                    .entry(parts[parts.len() - 1].to_string())
                    .or_default() += 1;
            }
        }
        for token in suspicious {
            if prefixes.get(token).copied().unwrap_or_default() >= 4 {
                warnings.push(format!(
                    "directory {dir} has many files starting with suspicious token `{token}`"
                ));
            }
            if suffixes.get(token).copied().unwrap_or_default() >= 4 {
                warnings.push(format!(
                    "directory {dir} has many files ending with suspicious token `{token}`"
                ));
            }
        }
    }
}

fn test_comment_heuristics(
    root: &Path,
    tracked: &[String],
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    for file in tracked {
        let is_source = [".rs", ".ts", ".tsx", ".vue", ".ps1"]
            .iter()
            .any(|ext| file.ends_with(ext));
        if !is_source {
            continue;
        }
        let lines = read_lines(&path_from_repo(root, file))?;
        if lines.len() < 25 {
            continue;
        }

        let comment_lines = lines
            .iter()
            .filter(|line| {
                let trimmed = line.trim_start();
                trimmed.starts_with("//")
                    || trimmed.starts_with("/*")
                    || trimmed.starts_with('*')
                    || trimmed.starts_with("<!--")
                    || trimmed.starts_with('#')
            })
            .count();
        let ratio = comment_lines as f64 / lines.len() as f64;
        if ratio > 0.28 && lines.len() < 140 {
            warnings.push(format!(
                "{file} has unusually high comment density ({:.1}%)",
                ratio * 100.0
            ));
        }

        let branch_signals = lines
            .iter()
            .filter(|line| {
                ["if", "match", "for", "while", "switch"]
                    .iter()
                    .any(|token| line.contains(token))
            })
            .count();
        if lines.len() >= 80 && branch_signals >= 8 && comment_lines == 0 {
            warnings.push(format!(
                "{file} is complex and has no comments explaining constraints or tradeoffs"
            ));
        }
    }
    Ok(())
}
