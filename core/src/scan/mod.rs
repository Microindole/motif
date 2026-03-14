use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const SUPPORTED_EXTENSIONS: &[&str] = &["html", "jsx", "ts", "tsx", "vue", "svelte"];
const SKIP_DIRECTORIES: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "dist",
    "build",
    "coverage",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScannedFile {
    pub path: PathBuf,
    pub class_names: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceInput {
    pub path: PathBuf,
    pub content: String,
}

#[derive(Debug)]
pub struct ScanResult {
    pub files: Vec<PathBuf>,
    pub entries: Vec<ScannedFile>,
    pub class_names: BTreeSet<String>,
}

#[derive(Debug)]
pub enum ScanError {
    Io { path: PathBuf, source: io::Error },
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
    let mut entries = Vec::new();
    visit_path(root, &mut entries)?;
    entries.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(build_scan_result(entries))
}

pub fn scan_sources(sources: &[SourceInput]) -> ScanResult {
    let mut entries = sources
        .iter()
        .filter_map(|source| scanned_file(&source.path, &source.content))
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| left.path.cmp(&right.path));
    build_scan_result(entries)
}

fn build_scan_result(entries: Vec<ScannedFile>) -> ScanResult {
    let files = entries
        .iter()
        .map(|entry| entry.path.clone())
        .collect::<Vec<_>>();
    let class_names = entries
        .iter()
        .flat_map(|entry| entry.class_names.iter().cloned())
        .collect::<BTreeSet<_>>();

    ScanResult {
        files,
        entries,
        class_names,
    }
}

fn visit_path(path: &Path, entries: &mut Vec<ScannedFile>) -> Result<(), ScanError> {
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

            visit_path(&child_path, entries)?;
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
    if let Some(entry) = scanned_file(path, &content) {
        entries.push(entry);
    }
    Ok(())
}

fn scanned_file(path: &Path, content: &str) -> Option<ScannedFile> {
    let class_names = extract_class_names(content);
    if class_names.is_empty() {
        return None;
    }

    Some(ScannedFile {
        path: path.to_path_buf(),
        class_names,
    })
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
