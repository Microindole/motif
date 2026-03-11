use crate::utils::{path_from_repo, read_lines};
use std::collections::HashMap;
use std::path::Path;

const DUPLICATE_WINDOW: usize = 12;

pub fn test_duplicate_blocks(
    root: &Path,
    tracked: &[String],
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let mut windows: HashMap<String, Vec<String>> = HashMap::new();

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
        if files.len() >= 2 {
            reports.push(files);
        }
    }
    reports.sort();
    reports.dedup();

    for files in reports.into_iter().take(5) {
        warnings.push(format!(
            "possible duplicate block across {}",
            files.join(", ")
        ));
    }

    Ok(())
}

fn is_duplicate_scan_target(file: &str) -> bool {
    (file.starts_with("core/src/")
        || file.starts_with("core/tests/")
        || file.starts_with("xtask/src/")
        || file.starts_with("scripts/"))
        && [".rs", ".ps1"].iter().any(|ext| file.ends_with(ext))
}

fn is_comment_only(line: &str) -> bool {
    line.starts_with("//")
        || line.starts_with("/*")
        || line.starts_with('*')
        || line.starts_with('#')
}
