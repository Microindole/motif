use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum WriteError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "{}: {}", path.display(), source),
        }
    }
}

impl std::error::Error for WriteError {}

pub fn resolve_output_path(root: &Path, explicit_output: Option<&Path>) -> PathBuf {
    if let Some(path) = explicit_output {
        return path.to_path_buf();
    }

    if root.is_dir() {
        root.join("motif.css")
    } else {
        root.parent()
            .unwrap_or_else(|| Path::new("."))
            .join("motif.css")
    }
}

pub fn write_stylesheet(path: &Path, content: &str) -> Result<(), WriteError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| WriteError::Io {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    fs::write(path, content).map_err(|source| WriteError::Io {
        path: path.to_path_buf(),
        source,
    })
}
