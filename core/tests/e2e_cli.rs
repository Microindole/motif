use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn generates_expected_css_for_cases_directory() {
    let repo_root = repo_root();
    let output = temp_output_path("cases");

    let result = Command::new(env!("CARGO_BIN_EXE_motif"))
        .arg(repo_root.join("cases"))
        .arg(&output)
        .output()
        .expect("failed to run motif binary for cases");

    assert!(result.status.success(), "stdout: {}\nstderr: {}", String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));

    let css = fs::read_to_string(&output).expect("failed to read generated css for cases");
    assert!(css.contains(".f-stack {"));
    assert!(css.contains(".focus\\:f-ring:focus {"));
    assert!(css.contains(".hover\\:f-bg-primary:hover {"));
    assert!(css.contains(".m-surface {"));

    cleanup_file(&output);
}

#[test]
fn generates_expected_css_for_all_demo_variants() {
    let repo_root = repo_root();
    let demos = [
        repo_root.join("demo").join("native").join("basic"),
        repo_root.join("demo").join("ts").join("basic"),
        repo_root.join("demo").join("react").join("basic"),
        repo_root.join("demo").join("vue").join("basic"),
    ];

    for demo_dir in demos {
        let output = temp_output_path(demo_dir.file_name().and_then(|v| v.to_str()).unwrap_or("demo"));
        let result = Command::new(env!("CARGO_BIN_EXE_motif"))
            .arg(&demo_dir)
            .arg(&output)
            .output()
            .unwrap_or_else(|error| panic!("failed to run motif binary for {}: {}", demo_dir.display(), error));

        assert!(
            result.status.success(),
            "demo={} stdout={} stderr={}",
            demo_dir.display(),
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );

        let css = fs::read_to_string(&output)
            .unwrap_or_else(|error| panic!("failed to read generated css for {}: {}", demo_dir.display(), error));
        assert!(css.contains(".f-stack {"), "demo={} css={}", demo_dir.display(), css);
        assert!(css.contains(".f-text-primary {"), "demo={} css={}", demo_dir.display(), css);
        assert!(css.contains(".focus\\:f-ring:focus {"), "demo={} css={}", demo_dir.display(), css);
        assert!(css.contains(".hover\\:f-bg-primary:hover {"), "demo={} css={}", demo_dir.display(), css);
        assert!(css.contains(".m-surface {"), "demo={} css={}", demo_dir.display(), css);

        cleanup_file(&output);
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("core crate should live under repo root")
        .to_path_buf()
}

fn temp_output_path(label: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("motif-e2e-{label}-{stamp}.css"))
}

fn cleanup_file(path: &Path) {
    if path.exists() {
        fs::remove_file(path).expect("failed to remove temporary css output");
    }
}
