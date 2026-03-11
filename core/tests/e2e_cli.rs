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

    assert!(
        result.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );

    let css = fs::read_to_string(&output).expect("failed to read generated css for cases");
    assert_case_selectors(&css);
    assert_variant_selectors(&css);
    assert_theme_selectors(&css);
    assert_workspace_selectors(&css);

    cleanup_file(&output);
}

#[test]
fn generates_expected_css_for_all_demo_scenarios() {
    let repo_root = repo_root();
    let demos = [
        repo_root.join("demo").join("native").join("basic"),
        repo_root.join("demo").join("native").join("variants"),
        repo_root.join("demo").join("native").join("theme"),
        repo_root.join("demo").join("native").join("workspace"),
        repo_root.join("demo").join("ts").join("basic"),
        repo_root.join("demo").join("ts").join("variants"),
        repo_root.join("demo").join("ts").join("theme"),
        repo_root.join("demo").join("ts").join("workspace"),
        repo_root.join("demo").join("react").join("basic"),
        repo_root.join("demo").join("react").join("variants"),
        repo_root.join("demo").join("react").join("theme"),
        repo_root.join("demo").join("react").join("workspace"),
        repo_root.join("demo").join("vue").join("basic"),
        repo_root.join("demo").join("vue").join("variants"),
        repo_root.join("demo").join("vue").join("theme"),
        repo_root.join("demo").join("vue").join("workspace"),
    ];

    for demo_dir in demos {
        let label = demo_dir
            .components()
            .rev()
            .take(2)
            .filter_map(|component| component.as_os_str().to_str())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("-");
        let output = temp_output_path(&label);
        let result = Command::new(env!("CARGO_BIN_EXE_motif"))
            .arg(&demo_dir)
            .arg(&output)
            .output()
            .unwrap_or_else(|error| {
                panic!(
                    "failed to run motif binary for {}: {}",
                    demo_dir.display(),
                    error
                )
            });

        assert!(
            result.status.success(),
            "demo={} stdout={} stderr={}",
            demo_dir.display(),
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr)
        );

        let css = fs::read_to_string(&output).unwrap_or_else(|error| {
            panic!(
                "failed to read generated css for {}: {}",
                demo_dir.display(),
                error
            )
        });
        assert_demo_selectors(&css);
        if demo_dir.ends_with("variants") {
            assert_variant_selectors(&css);
        }
        if demo_dir.ends_with("theme") {
            assert_theme_selectors(&css);
        }
        if demo_dir.ends_with("workspace") {
            assert_workspace_selectors(&css);
        }

        cleanup_file(&output);
    }
}

fn assert_case_selectors(css: &str) {
    assert!(css.contains(".f-stack {"));
    assert!(css.contains(".focus\\:f-ring:focus {"));
    assert!(css.contains(".hover\\:f-bg-primary:hover {"));
    assert!(css.contains(".m-surface {"));
}

fn assert_demo_selectors(css: &str) {
    assert_case_selectors(css);
    assert!(css.contains(".f-text-primary {"));
}

fn assert_variant_selectors(css: &str) {
    assert!(css.contains(".active\\:m-shadow-2:active {"));
    assert!(css.contains("box-shadow: 0 2px 6px rgba(60, 64, 67, 0.30);"));
    assert!(css.contains("@media (prefers-color-scheme: dark) {"));
    assert!(css.contains(".dark\\:m-elevation-1 {"));
    assert!(css.contains("box-shadow: 0 1px 2px rgba(60, 64, 67, 0.24);"));
}

fn assert_theme_selectors(css: &str) {
    assert!(css.contains(".f-surface {"));
    assert!(css.contains("backdrop-filter: blur(18px) saturate(1.15);"));
    assert!(css.contains(".f-surface-alt {"));
    assert!(css.contains("backdrop-filter: blur(28px) saturate(1.25);"));
    assert!(css.contains(".m-surface-variant {"));
    assert!(css.contains("background-color: #eef3fd;"));
    assert!(css.contains(".m-bg-primary-container {"));
    assert!(css.contains("background-color: #d3e3fd;"));
    assert!(css.contains("color: #041e49;"));
}

fn assert_workspace_selectors(css: &str) {
    assert!(css.contains(".f-label {"));
    assert!(css.contains(".f-divider {"));
    assert!(css.contains(".f-field {"));
    assert!(css.contains(".f-action-primary {"));
    assert!(css.contains(".m-label {"));
    assert!(css.contains(".m-divider {"));
    assert!(css.contains(".m-field {"));
    assert!(css.contains(".m-action-primary {"));
    assert!(css.contains(".m-action-tonal {"));
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
