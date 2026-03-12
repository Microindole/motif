// Validate the pull request body against the repository template so review context does not degrade.
use crate::quality::github_event::parse_pull_request_string_field;
use crate::quality::warning::{candidate, info};
use std::env;
use std::fs;
use std::path::Path;

const REQUIRED_SECTIONS: &[&str] = &[
    "## Summary",
    "## Hard checks",
    "## Structure review",
    "## AI-specific review",
];
const CLICHE_SUMMARY_LINES: &[&str] = &[
    "adjust stuff",
    "fix issues",
    "misc changes",
    "minor updates",
    "small fix",
    "small fixes",
    "update files",
    "update code",
    "various changes",
];

pub fn test_pr_description(failures: &mut Vec<String>, warnings: &mut Vec<String>) {
    let Ok(event_name) = env::var("GITHUB_EVENT_NAME") else {
        return;
    };
    if event_name != "pull_request" {
        return;
    }

    let Ok(event_path) = env::var("GITHUB_EVENT_PATH") else {
        failures.push("pr-description check failed: GITHUB_EVENT_PATH is missing".to_string());
        return;
    };

    let path = Path::new(&event_path);
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            failures.push(format!(
                "pr-description check failed: unable to read {}: {error}",
                path.display()
            ));
            return;
        }
    };

    let Some(body) = parse_pr_body_from_event(&content) else {
        failures.push("pr-description check failed: pull_request.body not found".to_string());
        return;
    };

    if body.trim().is_empty() {
        warnings.push(info(
            "PR description is empty; add one when the change is non-trivial".to_string(),
        ));
        return;
    }

    failures.extend(validate_pr_body(&body));
}

pub fn validate_pr_body(body: &str) -> Vec<String> {
    let body = body.trim();
    let mut failures = Vec::new();

    if body.is_empty() {
        failures.push("PR description must not be empty".to_string());
        return failures;
    }

    for section in REQUIRED_SECTIONS {
        if !body.contains(section) {
            failures.push(format!(
                "PR description is missing required section `{section}`"
            ));
        }
    }

    if let Some(summary) = extract_section(body, "## Summary", "## Hard checks") {
        let summary_lines: Vec<&str> = summary
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect();

        if summary_lines.iter().copied().all(is_template_checkbox) {
            failures.push(
                "PR Summary must include at least one non-checkbox line explaining the change"
                    .to_string(),
            );
        }

        let prose_lines: Vec<&str> = summary_lines
            .iter()
            .copied()
            .filter(|line| !is_template_checkbox(line))
            .collect();
        if !prose_lines.is_empty() && prose_lines.iter().all(|line| is_cliche_summary_line(line)) {
            failures.push(format!(
                "PR Summary is too vague: {}",
                candidate(
                    "replace generic lines like `update files` with concrete scope and intent"
                )
            ));
        }
    }

    let unchecked = body
        .lines()
        .filter(|line| line.trim_start().starts_with("- [ ]"))
        .count();
    if unchecked > 0 {
        failures.push(format!(
            "PR description still contains {unchecked} unchecked template item(s); complete them or delete them"
        ));
    }

    failures
}

pub fn parse_pr_body_from_event(content: &str) -> Option<String> {
    parse_pull_request_string_field(content, "body")
}

fn extract_section<'a>(body: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let start_index = body.find(start)? + start.len();
    let tail = &body[start_index..];
    let end_index = tail.find(end)?;
    Some(&tail[..end_index])
}

fn is_template_checkbox(line: &str) -> bool {
    line.starts_with("- [ ]") || line.starts_with("- [x]") || line.starts_with("- [X]")
}

fn is_cliche_summary_line(line: &str) -> bool {
    let normalized = line
        .trim_start_matches(['-', '*', ' '])
        .trim()
        .trim_end_matches('.')
        .to_ascii_lowercase();
    CLICHE_SUMMARY_LINES.contains(&normalized.as_str())
}
