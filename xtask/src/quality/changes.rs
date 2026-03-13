// Compare the current work against a meaningful base so oversized AI edits are caught before review.
use crate::quality::pr::active_large_change_override_from_env;
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

    let override_rationale = active_large_change_override_from_env();
    let counts = collect_counts(&output);
    for (label, value, thresholds) in [
        (
            "files",
            counts.files,
            Thresholds::new(WARN_CHANGED_FILES, FAIL_CHANGED_FILES),
        ),
        (
            "adds",
            counts.added,
            Thresholds::new(WARN_ADDED_LINES, FAIL_ADDED_LINES),
        ),
        (
            "deletes",
            counts.deleted,
            Thresholds::new(WARN_DELETED_LINES, FAIL_DELETED_LINES),
        ),
    ] {
        if let Some((is_failure, message)) = evaluate_metric(
            label,
            value,
            thresholds,
            &spec,
            override_rationale.as_deref(),
        ) {
            if is_failure {
                failures.push(message);
            } else {
                warnings.push(message);
            }
        }
    }
}

pub(crate) struct DiffSpec {
    pub range: String,
    pub label: String,
    pub hard_gate: bool,
}

struct ChangeCounts {
    files: usize,
    added: usize,
    deleted: usize,
}

#[derive(Clone, Copy)]
struct Thresholds {
    warn: usize,
    fail: usize,
}

impl Thresholds {
    const fn new(warn: usize, fail: usize) -> Self {
        Self { warn, fail }
    }
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

fn collect_counts(output: &str) -> ChangeCounts {
    let mut counts = ChangeCounts {
        files: 0,
        added: 0,
        deleted: 0,
    };

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
        counts.files += 1;
        counts.added += add.parse::<usize>().unwrap_or_default();
        counts.deleted += remove.parse::<usize>().unwrap_or_default();
    }

    counts
}

fn merge_base_range(root: &Path, target: &str) -> Option<String> {
    command_output("git", &["rev-parse", "--verify", target], root).ok()?;
    let base = command_output("git", &["merge-base", "HEAD", target], root)
        .ok()?
        .trim()
        .to_string();
    Some(format!("{base}..HEAD"))
}

fn evaluate_metric(
    label: &str,
    value: usize,
    thresholds: Thresholds,
    spec: &DiffSpec,
    override_rationale: Option<&str>,
) -> Option<(bool, String)> {
    let verb = match label {
        "files" => "touches",
        "adds" => "adds",
        "deletes" => "deletes",
        _ => "changes",
    };

    if value > thresholds.fail {
        let message = format!(
            "change set {verb} {value} {label} in {}; hard limit is {}",
            spec.label, thresholds.fail
        );
        if let Some(rationale) = override_rationale {
            return Some((
                false,
                format!(
                    "{message}; large-change override is active and still requires explicit review: {rationale}"
                ),
            ));
        }
        if spec.hard_gate {
            Some((true, message))
        } else {
            Some((false, format!("{message}; local branch signal only")))
        }
    } else if value > thresholds.warn {
        Some((
            false,
            format!(
                "change set {verb} {value} {label} in {}; consider splitting the work",
                spec.label
            ),
        ))
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
