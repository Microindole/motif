use crate::utils::command_output;
use std::env;
use std::path::Path;

const WARN_CHANGED_FILES: usize = 12;
const FAIL_CHANGED_FILES: usize = 24;
const WARN_ADDED_LINES: usize = 300;
const FAIL_ADDED_LINES: usize = 700;
const WARN_DELETED_LINES: usize = 200;
const FAIL_DELETED_LINES: usize = 500;

pub fn test_change_size(root: &Path, failures: &mut Vec<String>, warnings: &mut Vec<String>) {
    let Some(spec) = diff_spec(root) else {
        warnings.push("change-size check skipped: no usable diff base available".to_string());
        return;
    };

    let Ok(output) = command_output("git", &["diff", "--numstat", &spec.range], root) else {
        warnings.push(format!(
            "change-size check skipped: failed to diff {}",
            spec.label
        ));
        return;
    };

    let mut files = 0usize;
    let mut added = 0usize;
    let mut deleted = 0usize;

    for line in output.lines() {
        let mut parts = line.split_whitespace();
        let Some(add) = parts.next() else {
            continue;
        };
        let Some(remove) = parts.next() else {
            continue;
        };
        let Some(path) = parts.next() else {
            continue;
        };
        if is_generated_path(path) {
            continue;
        }
        files += 1;
        added += add.parse::<usize>().unwrap_or_default();
        deleted += remove.parse::<usize>().unwrap_or_default();
    }

    report_metric(
        "files",
        files,
        WARN_CHANGED_FILES,
        FAIL_CHANGED_FILES,
        &spec,
        failures,
        warnings,
    );
    report_metric(
        "adds",
        added,
        WARN_ADDED_LINES,
        FAIL_ADDED_LINES,
        &spec,
        failures,
        warnings,
    );
    report_metric(
        "deletes",
        deleted,
        WARN_DELETED_LINES,
        FAIL_DELETED_LINES,
        &spec,
        failures,
        warnings,
    );
}

pub(crate) struct DiffSpec {
    pub range: String,
    pub label: String,
    pub hard_gate: bool,
}

pub(crate) fn diff_spec(root: &Path) -> Option<DiffSpec> {
    if let Ok(base_ref) = env::var("GITHUB_BASE_REF") {
        let remote_ref = format!("origin/{base_ref}");
        if let Some(range) = merge_base_range(root, &remote_ref) {
            return Some(DiffSpec {
                range,
                label: format!("merge-base with {remote_ref}"),
                hard_gate: true,
            });
        }
    }

    if command_output("git", &["rev-parse", "--verify", "HEAD~1"], root).is_ok() {
        return Some(DiffSpec {
            range: "HEAD~1..HEAD".to_string(),
            label: "HEAD~1..HEAD".to_string(),
            hard_gate: true,
        });
    }

    let current_branch = command_output("git", &["rev-parse", "--abbrev-ref", "HEAD"], root)
        .ok()
        .map(|value| value.trim().to_string())
        .unwrap_or_default();

    if current_branch != "main" && current_branch != "master" {
        for remote_ref in ["origin/main", "origin/master"] {
            if let Some(range) = merge_base_range(root, remote_ref) {
                return Some(DiffSpec {
                    range,
                    label: format!("merge-base with {remote_ref}"),
                    hard_gate: false,
                });
            }
        }
    }

    None
}

fn merge_base_range(root: &Path, target: &str) -> Option<String> {
    command_output("git", &["rev-parse", "--verify", target], root).ok()?;
    let base = command_output("git", &["merge-base", "HEAD", target], root)
        .ok()?
        .trim()
        .to_string();
    Some(format!("{base}..HEAD"))
}

fn report_metric(
    label: &str,
    value: usize,
    warn_limit: usize,
    fail_limit: usize,
    spec: &DiffSpec,
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    let verb = match label {
        "files" => "touches",
        "adds" => "adds",
        "deletes" => "deletes",
        _ => "changes",
    };

    if value > fail_limit {
        let message = format!(
            "change set {verb} {value} {label} in {}; hard limit is {fail_limit}",
            spec.label
        );
        if spec.hard_gate {
            failures.push(message);
        } else {
            warnings.push(format!("{message}; local branch signal only"));
        }
    } else if value > warn_limit {
        warnings.push(format!(
            "change set {verb} {value} {label} in {}; consider splitting the work",
            spec.label
        ));
    }
}

fn is_generated_path(path: &str) -> bool {
    path.contains("/target/")
        || path.contains("/node_modules/")
        || path.contains("/dist/")
        || path.contains("/coverage/")
        || path.contains("/.vite/")
        || path.ends_with("/motif.css")
        || path.ends_with(".tsbuildinfo")
}
