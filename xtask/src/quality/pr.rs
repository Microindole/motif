// Validate the pull request body against the repository template so review context does not degrade.
use crate::quality::github_event::parse_pull_request_string_field;
use std::env;
use std::fs;
use std::path::Path;

const REQUIRED_SECTIONS: &[&str] = &[
    "## Summary",
    "## Hard checks",
    "## Structure review",
    "## AI-specific review",
    "## Large change override",
];
const LARGE_CHANGE_HEADING: &str = "## Large change override";
const LARGE_CHANGE_CHECKBOX: &str =
    "- [x] This PR intentionally exceeds the change-size gate and requires explicit human review.";
const LARGE_CHANGE_CHECKBOX_ALT: &str =
    "- [X] This PR intentionally exceeds the change-size gate and requires explicit human review.";

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
        warnings
            .push("PR description is empty; add one when the change is non-trivial".to_string());
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
        if summary
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .all(is_template_checkbox)
        {
            failures.push(
                "PR Summary must include at least one non-checkbox line explaining the change"
                    .to_string(),
            );
        }
    }

    if body.contains(LARGE_CHANGE_CHECKBOX) || body.contains(LARGE_CHANGE_CHECKBOX_ALT) {
        match large_change_override_rationale(body) {
            Some(rationale) if !rationale.is_empty() => {}
            _ => failures.push(
                "Large change override is checked but missing rationale under `## Large change override`"
                    .to_string(),
            ),
        }
    }

    let unchecked = count_unchecked_template_items(body);
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

pub fn active_large_change_override_from_env() -> Option<String> {
    let Ok(event_name) = env::var("GITHUB_EVENT_NAME") else {
        return None;
    };
    if event_name != "pull_request" {
        return None;
    }
    let Ok(event_path) = env::var("GITHUB_EVENT_PATH") else {
        return None;
    };
    let content = fs::read_to_string(Path::new(&event_path)).ok()?;
    let body = parse_pr_body_from_event(&content)?;
    large_change_override_rationale(&body)
}

pub fn large_change_override_rationale(body: &str) -> Option<String> {
    if !(body.contains(LARGE_CHANGE_CHECKBOX) || body.contains(LARGE_CHANGE_CHECKBOX_ALT)) {
        return None;
    }
    let section = extract_section_to_end(body, LARGE_CHANGE_HEADING)?;
    let rationale = section
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("- ["))
        .collect::<Vec<_>>()
        .join(" ");
    if rationale.is_empty() {
        None
    } else {
        Some(rationale)
    }
}

fn extract_section<'a>(body: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let start_index = body.find(start)? + start.len();
    let tail = &body[start_index..];
    let end_index = tail.find(end)?;
    Some(&tail[..end_index])
}

fn extract_section_to_end<'a>(body: &'a str, start: &str) -> Option<&'a str> {
    let start_index = body.find(start)? + start.len();
    Some(&body[start_index..])
}

fn count_unchecked_template_items(body: &str) -> usize {
    let mut in_large_change_section = false;
    let mut count = 0;
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            in_large_change_section = trimmed == LARGE_CHANGE_HEADING;
        }
        if trimmed.starts_with("- [ ]") && !in_large_change_section {
            count += 1;
        }
    }
    count
}

fn is_template_checkbox(line: &str) -> bool {
    line.starts_with("- [ ]") || line.starts_with("- [x]") || line.starts_with("- [X]")
}
