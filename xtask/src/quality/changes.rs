use crate::utils::command_output;
use std::path::Path;

const WARN_CHANGED_FILES: usize = 12;
const FAIL_CHANGED_FILES: usize = 24;
const WARN_ADDED_LINES: usize = 300;
const FAIL_ADDED_LINES: usize = 700;
const WARN_DELETED_LINES: usize = 200;
const FAIL_DELETED_LINES: usize = 500;

pub fn test_change_size(root: &Path, failures: &mut Vec<String>, warnings: &mut Vec<String>) {
    let Some(range) = diff_range(root) else {
        warnings.push("change-size check skipped: no previous commit available".to_string());
        return;
    };

    let Ok(output) = command_output("git", &["diff", "--numstat", &range], root) else {
        warnings.push(format!(
            "change-size check skipped: failed to diff range {range}"
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

    if files > FAIL_CHANGED_FILES {
        failures.push(format!(
            "change set touches {files} files in {range}; hard limit is {FAIL_CHANGED_FILES}"
        ));
    } else if files > WARN_CHANGED_FILES {
        warnings.push(format!(
            "change set touches {files} files in {range}; consider splitting the work"
        ));
    }

    if added > FAIL_ADDED_LINES {
        failures.push(format!(
            "change set adds {added} lines in {range}; hard limit is {FAIL_ADDED_LINES}"
        ));
    } else if added > WARN_ADDED_LINES {
        warnings.push(format!(
            "change set adds {added} lines in {range}; consider splitting the work"
        ));
    }

    if deleted > FAIL_DELETED_LINES {
        failures.push(format!(
            "change set deletes {deleted} lines in {range}; hard limit is {FAIL_DELETED_LINES}"
        ));
    } else if deleted > WARN_DELETED_LINES {
        warnings.push(format!(
            "change set deletes {deleted} lines in {range}; consider splitting the work"
        ));
    }
}

fn diff_range(root: &Path) -> Option<String> {
    if command_output("git", &["rev-parse", "--verify", "HEAD~1"], root).is_ok() {
        Some("HEAD~1..HEAD".to_string())
    } else {
        None
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
