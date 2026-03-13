use crate::quality::changes::diff_spec;
use crate::utils::{command_output, path_from_repo, read_lines};
use std::collections::HashMap;
use std::path::Path;

const DUPLICATE_WINDOW: usize = 12;
const HARD_FAIL_DUPLICATE_GROUPS: usize = 1;
const ALLOWED_MIRRORED_RULE_PAIR: [&str; 2] =
    ["core/src/rule/fluent.rs", "core/src/rule/material.rs"];

// Duplicate detection stays exact and local: only repeated normalized windows count, and
// hard failures only trigger when the repeated block touches files changed in the current diff.
pub fn test_duplicate_blocks(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let mut windows: HashMap<String, Vec<String>> = HashMap::new();
    let changed_files = changed_files(root);

    for file in tracked {
        let path = path_from_repo(root, file);
        if !path.is_file() || !is_duplicate_scan_target(file) {
            continue;
        }

        let lines = read_lines(&path)?;
        let normalized: Vec<String> = lines
            .into_iter()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty() && !is_comment_only(line))
            .collect();

        if normalized.len() < DUPLICATE_WINDOW {
            continue;
        }

        for window in normalized.windows(DUPLICATE_WINDOW) {
            let fingerprint = window.join("\n");
            let entry = windows.entry(fingerprint).or_default();
            if !entry.iter().any(|existing| existing == file) {
                entry.push(file.clone());
            }
        }
    }

    let mut reports = Vec::new();
    for files in windows.into_values() {
        if files.len() >= 2 && !is_allowed_mirrored_rule_report(&files) {
            reports.push(files);
        }
    }
    reports.sort();
    reports.dedup();

    let changed_reports = reports
        .iter()
        .filter(|files| {
            files
                .iter()
                .any(|file| changed_files.iter().any(|changed| changed == file))
        })
        .count();
    if changed_reports >= HARD_FAIL_DUPLICATE_GROUPS {
        failures.push(format!(
            "change set introduces {changed_reports} duplicate block group(s) touching changed files"
        ));
    }

    for files in reports.into_iter().take(5) {
        warnings.push(format!(
            "possible duplicate block across {}",
            files.join(", ")
        ));
    }

    Ok(())
}

fn changed_files(root: &Path) -> Vec<String> {
    let Some(spec) = diff_spec(root) else {
        return Vec::new();
    };
    let Ok(output) = command_output("git", &["diff", "--name-only", &spec.range], root) else {
        return Vec::new();
    };
    output
        .lines()
        .map(|line| line.trim().replace('\\', "/"))
        .filter(|line| !line.is_empty() && is_duplicate_scan_target(line))
        .collect()
}

fn is_duplicate_scan_target(file: &str) -> bool {
    (file.starts_with("core/src/")
        || file.starts_with("core/tests/")
        || file.starts_with("xtask/src/")
        || file.starts_with("scripts/"))
        && [".rs", ".ps1"].iter().any(|ext| file.ends_with(ext))
}

fn is_allowed_mirrored_rule_report(files: &[String]) -> bool {
    if files.len() != ALLOWED_MIRRORED_RULE_PAIR.len() {
        return false;
    }

    ALLOWED_MIRRORED_RULE_PAIR
        .iter()
        .all(|allowed| files.iter().any(|file| file == allowed))
}

fn is_comment_only(line: &str) -> bool {
    line.starts_with("//")
        || line.starts_with("/*")
        || line.starts_with('*')
        || line.starts_with('#')
}
