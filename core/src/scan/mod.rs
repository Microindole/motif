use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const SUPPORTED_EXTENSIONS: &[&str] = &["html", "jsx", "ts", "tsx", "vue", "svelte"];
const SKIP_DIRECTORIES: &[&str] = &[".git", "node_modules", "target", "dist", "build", "coverage"];

#[derive(Debug)]
pub struct ScanResult {
    pub files: Vec<PathBuf>,
    pub class_names: BTreeSet<String>,
}

#[derive(Debug)]
pub enum ScanError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
}

impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {}", path.display(), source),
        }
    }
}

impl std::error::Error for ScanError {}

pub fn scan_root(root: &Path) -> Result<ScanResult, ScanError> {
    let mut files = Vec::new();
    let mut class_names = BTreeSet::new();

    visit_path(root, &mut files, &mut class_names)?;
    files.sort();

    Ok(ScanResult { files, class_names })
}

fn visit_path(
    path: &Path,
    files: &mut Vec<PathBuf>,
    class_names: &mut BTreeSet<String>,
) -> Result<(), ScanError> {
    let metadata = fs::metadata(path).map_err(|source| ScanError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    if metadata.is_dir() {
        for entry in fs::read_dir(path).map_err(|source| ScanError::Io {
            path: path.to_path_buf(),
            source,
        })? {
            let entry = entry.map_err(|source| ScanError::Io {
                path: path.to_path_buf(),
                source,
            })?;
            let child_path = entry.path();

            if should_skip_dir(&child_path) {
                continue;
            }

            visit_path(&child_path, files, class_names)?;
        }

        return Ok(());
    }

    if !is_supported_file(path) {
        return Ok(());
    }

    let content = fs::read_to_string(path).map_err(|source| ScanError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    files.push(path.to_path_buf());
    class_names.extend(extract_class_names(&content));

    Ok(())
}

fn should_skip_dir(path: &Path) -> bool {
    path.is_dir()
        && path
            .file_name()
            .and_then(|value| value.to_str())
            .map(|value| SKIP_DIRECTORIES.contains(&value))
            .unwrap_or(false)
}

fn is_supported_file(path: &Path) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .map(|value| SUPPORTED_EXTENSIONS.contains(&value))
        .unwrap_or(false)
}

fn extract_class_names(content: &str) -> BTreeSet<String> {
    content
        .split(is_token_boundary)
        .filter_map(normalize_candidate)
        .collect()
}

fn is_token_boundary(ch: char) -> bool {
    ch.is_whitespace()
        || matches!(
            ch,
            '"' | '\'' | '`' | '<' | '>' | '=' | '{' | '}' | '(' | ')' | '[' | ']' | ',' | ';'
        )
}

fn normalize_candidate(candidate: &str) -> Option<String> {
    let trimmed = candidate.trim_matches(|ch: char| !is_class_char(ch));
    if trimmed.is_empty() || !trimmed.chars().all(is_class_char) {
        return None;
    }

    let utility = trimmed.rsplit(':').next().unwrap_or(trimmed);
    if utility.starts_with("f-") || utility.starts_with("m-") {
        Some(trimmed.to_string())
    } else {
        None
    }
}

fn is_class_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '-' | ':' | '/' | '_' | '.')
}

#[cfg(test)]
mod tests {
    use super::{extract_class_names, scan_root};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn extracts_prefixed_classes_from_mixed_markup() {
        let classes = extract_class_names(
            r#"
            <div class="f-text-primary hover:f-bg-primary dark:m-elevation-1 plain-class">
              <button className={'focus:f-ring active:m-shadow-2'} />
            </div>
            "#,
        );

        let actual: Vec<_> = classes.into_iter().collect();
        assert_eq!(
            actual,
            vec![
                "active:m-shadow-2",
                "dark:m-elevation-1",
                "f-text-primary",
                "focus:f-ring",
                "hover:f-bg-primary",
            ]
        );
    }

    #[test]
    fn scans_supported_files_and_skips_build_output() {
        let root = unique_temp_dir();
        fs::create_dir_all(root.join("src")).unwrap();
        fs::create_dir_all(root.join("target")).unwrap();

        fs::write(
            root.join("src").join("app.tsx"),
            r#"<main className="f-stack hover:f-bg-primary ignored"></main>"#,
        )
        .unwrap();
        fs::write(
            root.join("src").join("card.vue"),
            r#"<template><section class="m-surface focus:f-ring"></section></template>"#,
        )
        .unwrap();
        fs::write(
            root.join("target").join("noise.html"),
            r#"<div class="f-should-not-appear"></div>"#,
        )
        .unwrap();

        let result = scan_root(&root).unwrap();
        let actual: Vec<_> = result.class_names.into_iter().collect();

        assert_eq!(result.files.len(), 2);
        assert_eq!(
            actual,
            vec!["f-stack", "focus:f-ring", "hover:f-bg-primary", "m-surface"]
        );

        fs::remove_dir_all(root).unwrap();
    }

    fn unique_temp_dir() -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("motif-scan-test-{stamp}"))
    }
}
