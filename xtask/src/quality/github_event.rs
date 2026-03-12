// Read a few string fields from the GitHub pull_request event payload without adding serde.
pub fn parse_pull_request_string_field(content: &str, key: &str) -> Option<String> {
    let pr_index = content.find("\"pull_request\"")?;
    let key_index = content[pr_index..].find(&format!("\"{key}\""))? + pr_index;
    let after_key = &content[key_index + key.len() + 2..];
    let colon = after_key.find(':')?;
    let after_colon = after_key[colon + 1..].trim_start();
    if after_colon.starts_with("null") {
        return Some(String::new());
    }
    parse_json_string(after_colon)
}

pub fn parse_pull_request_nested_string_field(
    content: &str,
    parent: &str,
    key: &str,
) -> Option<String> {
    let pr_index = content.find("\"pull_request\"")?;
    let parent_index = content[pr_index..].find(&format!("\"{parent}\""))? + pr_index;
    let key_index = content[parent_index..].find(&format!("\"{key}\""))? + parent_index;
    let after_key = &content[key_index + key.len() + 2..];
    let colon = after_key.find(':')?;
    let after_colon = after_key[colon + 1..].trim_start();
    parse_json_string(after_colon)
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
