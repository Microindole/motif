// Keep dependency growth visible before AI changes turn small demos or tooling into dependency sinks.
use crate::utils::{path_from_repo, read_lines};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const CORE_WARN_DEPS: usize = 4;
const CORE_FAIL_DEPS: usize = 8;
const XTASK_WARN_DEPS: usize = 3;
const XTASK_FAIL_DEPS: usize = 6;
const DEMO_WARN_DEPS: usize = 8;
const DEMO_FAIL_DEPS: usize = 10;

pub fn test_dependency_hygiene(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    test_cargo_manifest(
        root,
        "core/Cargo.toml",
        CORE_WARN_DEPS,
        CORE_FAIL_DEPS,
        failures,
        warnings,
    )?;
    test_cargo_manifest(
        root,
        "xtask/Cargo.toml",
        XTASK_WARN_DEPS,
        XTASK_FAIL_DEPS,
        failures,
        warnings,
    )?;
    test_package_manifests(root, tracked, failures, warnings)?;
    Ok(())
}

fn test_cargo_manifest(
    root: &Path,
    manifest: &str,
    warn_limit: usize,
    fail_limit: usize,
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let path = path_from_repo(root, manifest);
    let lines = read_lines(&path)?;
    let count = count_cargo_dependencies(&lines);
    if count > fail_limit {
        failures.push(format!(
            "{manifest} declares {count} direct dependencies; hard limit is {fail_limit}"
        ));
    } else if count > warn_limit {
        warnings.push(format!(
            "{manifest} declares {count} direct dependencies; review whether they all pay for themselves"
        ));
    }
    Ok(())
}

fn test_package_manifests(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let manifests: Vec<&String> = tracked
        .iter()
        .filter(|file| file.ends_with("package.json"))
        .collect();

    let mut versions: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for manifest in manifests {
        let path = path_from_repo(root, manifest);
        let content = std::fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {manifest}: {error}"))?;
        let dependency_count = count_json_object_entries(&content, "dependencies")
            + count_json_object_entries(&content, "devDependencies");

        if dependency_count > DEMO_FAIL_DEPS {
            failures.push(format!(
                "{manifest} declares {dependency_count} npm dependencies; hard limit is {DEMO_FAIL_DEPS}"
            ));
        } else if dependency_count > DEMO_WARN_DEPS {
            warnings.push(format!(
                "{manifest} declares {dependency_count} npm dependencies; review whether the demo is pulling too much"
            ));
        }

        for section in ["dependencies", "devDependencies"] {
            for (name, version) in extract_json_pairs(&content, section) {
                versions.entry(name).or_default().insert(version);
            }
        }
    }

    for (name, version_set) in versions {
        if version_set.len() >= 2 {
            warnings.push(format!(
                "npm dependency `{name}` uses multiple versions across demos: {}",
                version_set.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
    }

    Ok(())
}

fn count_cargo_dependencies(lines: &[String]) -> usize {
    let mut count = 0usize;
    let mut in_dependencies = false;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_dependencies = trimmed == "[dependencies]";
            continue;
        }
        if !in_dependencies || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if trimmed.contains('=') {
            count += 1;
        }
    }

    count
}

fn count_json_object_entries(content: &str, key: &str) -> usize {
    extract_json_pairs(content, key).len()
}

fn extract_json_pairs(content: &str, key: &str) -> Vec<(String, String)> {
    let Some(body) = extract_json_object_body(content, key) else {
        return Vec::new();
    };

    body.lines()
        .filter_map(|line| {
            let trimmed = line.trim().trim_end_matches(',').trim();
            if trimmed.is_empty() || !trimmed.starts_with('"') {
                return None;
            }
            let mut parts = trimmed.splitn(2, ':');
            let name = parts.next()?.trim().trim_matches('"').to_string();
            let version = parts.next()?.trim().trim_matches('"').to_string();
            Some((name, version))
        })
        .collect()
}

fn extract_json_object_body<'a>(content: &'a str, key: &str) -> Option<&'a str> {
    let marker = format!("\"{key}\"");
    let start = content.find(&marker)?;
    let after_marker = &content[start + marker.len()..];
    let open_offset = after_marker.find('{')?;
    let body_start = start + marker.len() + open_offset + 1;

    let mut depth = 1usize;
    let mut end = body_start;
    for (index, ch) in content[body_start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    end = body_start + index;
                    break;
                }
            }
            _ => {}
        }
    }

    if end == body_start {
        None
    } else {
        Some(&content[body_start..end])
    }
}
