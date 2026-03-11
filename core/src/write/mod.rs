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

#[cfg(test)]
mod tests {
    use super::{resolve_output_path, write_stylesheet};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn resolves_default_output_beside_scan_root() {
        let root = std::env::temp_dir().join(unique_name());
        fs::create_dir_all(root.join("cases")).unwrap();

        let output = resolve_output_path(&root, None);
        assert_eq!(output, root.join("motif.css"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn writes_stylesheet_to_disk() {
        let root = std::env::temp_dir().join(unique_name());
        let output = root.join("nested").join("motif.css");

        write_stylesheet(&output, ".f-stack {}\n").unwrap();

        let written = fs::read_to_string(&output).unwrap();
        assert_eq!(written, ".f-stack {}\n");

        fs::remove_dir_all(root).unwrap();
    }

    fn unique_name() -> String {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("motif-write-test-{stamp}")
    }
}
