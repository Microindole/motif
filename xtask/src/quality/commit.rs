use crate::quality::github_event::parse_pull_request_nested_string_field;
use crate::utils::command_output;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const MAX_SUBJECT_LEN: usize = 72;
const NON_TRIVIAL_FILE_THRESHOLD: usize = 3;
const NON_TRIVIAL_LINE_THRESHOLD: usize = 40;
const ALLOWED_TYPES: &[&str] = &[
    "feat", "fix", "refactor", "docs", "test", "build", "ci", "chore", "perf", "style",
];

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MessageCheckResult {
    pub failures: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn test_commit_message(root: &Path, failures: &mut Vec<String>, warnings: &mut Vec<String>) {
    let revision = pr_head_sha_from_env().unwrap_or_else(|| "HEAD".to_string());
    let Ok(message) = command_output("git", &["log", "-1", "--pretty=%s%n%b", &revision], root)
    else {
        warnings.push(format!(
            "commit-message check skipped: unable to read commit message for {revision}"
        ));
        return;
    };

    let is_non_trivial = commit_looks_non_trivial(root, &revision).unwrap_or(true);
    let result = evaluate_commit_message_for_commit(&message, is_non_trivial);
    failures.extend(result.failures);
    warnings.extend(result.warnings);
}

pub fn evaluate_commit_message(message: &str) -> MessageCheckResult {
    evaluate_commit_message_for_commit(message, true)
}

pub fn evaluate_commit_message_for_commit(
    message: &str,
    is_non_trivial: bool,
) -> MessageCheckResult {
    let mut result = MessageCheckResult::default();
    let mut lines = message.lines();
    let subject = lines.next().unwrap_or("").trim();
    let body = lines.collect::<Vec<_>>().join("\n").trim().to_string();

    if subject.is_empty() {
        result
            .failures
            .push("HEAD commit message subject is empty".to_string());
        return result;
    }
    if subject.len() > MAX_SUBJECT_LEN {
        result.failures.push(format!(
            "HEAD commit subject is {} chars; limit is {MAX_SUBJECT_LEN}",
            subject.len()
        ));
    }
    if subject.ends_with('.') {
        result
            .failures
            .push("HEAD commit subject must not end with a period".to_string());
    }

    if let Some((commit_type, description)) = split_subject(subject) {
        if !ALLOWED_TYPES.contains(&commit_type) {
            result.failures.push(format!(
                "HEAD commit subject uses unsupported type `{commit_type}`"
            ));
        }
        if description.is_empty() {
            result
                .failures
                .push("HEAD commit subject must include a description after `: `".to_string());
        } else if description
            .chars()
            .next()
            .is_some_and(|ch| ch.is_uppercase())
        {
            result.warnings.push(
                "HEAD commit subject description starts with uppercase; prefer sentence fragment style"
                    .to_string(),
            );
        }
    } else {
        result.failures.push(
            "HEAD commit subject must match `type: description` or `type(scope): description`"
                .to_string(),
        );
    }

    if body.is_empty() && is_non_trivial {
        result
            .warnings
            .push("HEAD commit has no body; add one when the change is non-trivial".to_string());
    }

    result
}

pub fn parse_pr_head_sha_from_event(content: &str) -> Option<String> {
    parse_pull_request_nested_string_field(content, "head", "sha")
}

fn commit_looks_non_trivial(root: &Path, revision: &str) -> Option<bool> {
    let output = command_output(
        "git",
        &["show", "--format=", "--shortstat", "--no-renames", revision],
        root,
    )
    .ok()?;
    let stats = parse_shortstat(&output);
    Some(
        stats.files >= NON_TRIVIAL_FILE_THRESHOLD
            || stats.changed_lines >= NON_TRIVIAL_LINE_THRESHOLD,
    )
}

#[derive(Default)]
struct CommitChangeStats {
    files: usize,
    changed_lines: usize,
}

fn parse_shortstat(output: &str) -> CommitChangeStats {
    let mut stats = CommitChangeStats::default();
    let line = output.lines().find(|line| line.contains("changed"));
    let Some(line) = line else {
        return stats;
    };

    for part in line.split(',') {
        let trimmed = part.trim();
        if let Some(value) = trimmed.split_whitespace().next() {
            let count = value.parse::<usize>().unwrap_or_default();
            if trimmed.contains("file") {
                stats.files = count;
            } else if trimmed.contains("insertion") || trimmed.contains("deletion") {
                stats.changed_lines += count;
            }
        }
    }

    stats
}

fn pr_head_sha_from_env() -> Option<String> {
    let event_name = env::var("GITHUB_EVENT_NAME").ok()?;
    if event_name != "pull_request" {
        return None;
    }

    let event_path = PathBuf::from(env::var("GITHUB_EVENT_PATH").ok()?);
    let content = fs::read_to_string(event_path).ok()?;
    parse_pr_head_sha_from_event(&content)
}

fn split_subject(subject: &str) -> Option<(&str, &str)> {
    let (prefix, description) = subject.split_once(": ")?;
    let commit_type = prefix.split('(').next()?.trim();
    Some((commit_type, description.trim()))
}
