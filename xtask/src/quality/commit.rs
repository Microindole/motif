use crate::utils::command_output;
use std::path::Path;

const MAX_SUBJECT_LEN: usize = 72;
const ALLOWED_TYPES: &[&str] = &[
    "feat", "fix", "refactor", "docs", "test", "build", "ci", "chore", "perf", "style",
];

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MessageCheckResult {
    pub failures: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn test_commit_message(root: &Path, failures: &mut Vec<String>, warnings: &mut Vec<String>) {
    let Ok(message) = command_output("git", &["log", "-1", "--pretty=%s%n%b"], root) else {
        warnings.push("commit-message check skipped: unable to read HEAD message".to_string());
        return;
    };

    let result = evaluate_commit_message(&message);
    failures.extend(result.failures);
    warnings.extend(result.warnings);
}

pub fn evaluate_commit_message(message: &str) -> MessageCheckResult {
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

    if body.is_empty() {
        result
            .warnings
            .push("HEAD commit has no body; add one when the change is non-trivial".to_string());
    }

    result
}

fn split_subject(subject: &str) -> Option<(&str, &str)> {
    let (prefix, description) = subject.split_once(": ")?;
    let commit_type = prefix.split('(').next()?.trim();
    Some((commit_type, description.trim()))
}
