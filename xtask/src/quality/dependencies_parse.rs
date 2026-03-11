use std::collections::{BTreeMap, BTreeSet};

pub fn count_cargo_dependencies(lines: &[String]) -> usize {
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

pub fn count_json_object_entries(content: &str, key: &str) -> usize {
    extract_json_pairs(content, key).len()
}

pub fn extract_json_pairs(content: &str, key: &str) -> Vec<(String, String)> {
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

pub fn collect_version_map(
    versions: &mut BTreeMap<String, BTreeSet<String>>,
    content: &str,
    sections: &[&str],
) {
    for section in sections {
        for (name, version) in extract_json_pairs(content, section) {
            versions.entry(name).or_default().insert(version);
        }
    }
}

pub fn extract_added_cargo_dependencies(diff_lines: &[String]) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_dependencies = false;

    for line in diff_lines {
        let Some((prefix, body)) = diff_body(line) else {
            continue;
        };
        let trimmed = body.trim();
        if trimmed.starts_with('[') {
            in_dependencies = trimmed == "[dependencies]";
            continue;
        }
        if prefix != '+' || !in_dependencies || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((name, _)) = trimmed.split_once('=') {
            names.push(name.trim().to_string());
        }
    }

    finalize_names(names)
}

pub fn extract_added_json_dependencies(diff_lines: &[String]) -> Vec<String> {
    let mut names = Vec::new();
    let mut current_section: Option<&str> = None;
    let mut section_depth = 0usize;

    for line in diff_lines {
        let Some((prefix, body)) = diff_body(line) else {
            continue;
        };
        let trimmed = body.trim();

        if trimmed.starts_with("\"dependencies\"") {
            current_section = Some("dependencies");
            section_depth = brace_delta(trimmed).max(0) as usize;
            continue;
        }
        if trimmed.starts_with("\"devDependencies\"") {
            current_section = Some("devDependencies");
            section_depth = brace_delta(trimmed).max(0) as usize;
            continue;
        }

        if current_section.is_some() {
            section_depth = section_depth.saturating_add_signed(brace_delta(trimmed));
            if prefix == '+' && trimmed.starts_with('"') {
                if let Some((name, _)) = trimmed.trim_end_matches(',').split_once(':') {
                    names.push(name.trim().trim_matches('"').to_string());
                }
            }
            if section_depth == 0 {
                current_section = None;
            }
        }
    }

    let names = names
        .into_iter()
        .filter(|name| name != "dependencies" && name != "devDependencies")
        .collect();
    finalize_names(names)
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

fn diff_body(line: &str) -> Option<(char, &str)> {
    let prefix = line.chars().next()?;
    if !matches!(prefix, ' ' | '+' | '-') || line.starts_with("+++") || line.starts_with("---") {
        return None;
    }
    Some((prefix, &line[1..]))
}

fn brace_delta(line: &str) -> isize {
    line.chars().fold(0, |acc, ch| match ch {
        '{' => acc + 1,
        '}' => acc - 1,
        _ => acc,
    })
}

fn finalize_names(mut names: Vec<String>) -> Vec<String> {
    names.sort();
    names.dedup();
    names
}
