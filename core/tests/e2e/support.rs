use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("core crate should live under repo root")
        .to_path_buf()
}

pub(crate) fn temp_output_path(label: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("motif-e2e-{label}-{stamp}.css"))
}

pub(crate) fn cleanup_file(path: &Path) {
    if path.exists() {
        fs::remove_file(path).expect("failed to remove temporary css output");
    }
}
