// Validate the pull request body against the repository template so review context does not degrade.
use std::env;
use std::fs;
use std::path::Path;

const REQUIRED_SECTIONS: &[&str] = &[
    "## Summary",
    "## Hard checks",
    "## Structure review",
    "## AI-specific review",
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

    failures.extend(validate_pr_body(&body));
    let _ = warnings;
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
    let pr_index = content.find("\"pull_request\"")?;
    let body_key = content[pr_index..].find("\"body\"")? + pr_index;
    let after_key = &content[body_key + "\"body\"".len()..];
    let colon = after_key.find(':')?;
    let after_colon = after_key[colon + 1..].trim_start();
    if after_colon.starts_with("null") {
        return Some(String::new());
    }
    parse_json_string(after_colon)
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

fn parse_json_string(input: &str) -> Option<String> {
    let mut chars = input.chars();
    if chars.next()? != '"' {
        return None;
    }

    let mut result = String::new();
    let mut escaped = false;
    for ch in chars {
        if escaped {
            match ch {
                '"' => result.push('"'),
                '\\' => result.push('\\'),
                '/' => result.push('/'),
                'b' => result.push('\u{0008}'),
                'f' => result.push('\u{000C}'),
                'n' => result.push('\n'),
                'r' => result.push('\r'),
                't' => result.push('\t'),
                'u' => return None,
                _ => return None,
            }
            escaped = false;
            continue;
        }

        match ch {
            '\\' => escaped = true,
            '"' => return Some(result),
            _ => result.push(ch),
        }
    }

    None
}
