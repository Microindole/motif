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
            "demo={} stdout={} stderr= {}",
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
    assert!(css.contains(".focus\\:f-ring:focus {") || css.contains(".focus\\:m-ring:focus {"));
    assert!(
        css.contains(".hover\\:f-bg-hover-primary:hover {")
            || css.contains(".hover\\:m-bg-hover-primary:hover {")
    );
    assert!(css.contains(".m-surface {"));
}

fn assert_demo_selectors(css: &str) {
    assert_case_selectors(css);
    assert!(css.contains(".f-text-primary {"));
}

fn assert_variant_selectors(css: &str) {
    assert!(css.contains(".hover\\:m-bg-hover-primary:hover {"));
    assert!(css.contains("background-color: #1765cc;"));
    assert!(css.contains(".active\\:m-shadow-press:active {"));
    assert!(css.contains(
        "box-shadow: 0 1px 2px rgba(60, 64, 67, 0.24), inset 0 1px 0 rgba(255, 255, 255, 0.18);"
    ));
    assert!(css.contains("@media (prefers-color-scheme: dark) {"));
    assert!(css.contains(".dark\\:m-elevation-1 {"));
    assert!(css.contains("box-shadow: 0 1px 2px rgba(60, 64, 67, 0.24);"));
    assert!(css.contains(".focus\\:m-ring:focus {"));
    assert!(css.contains(".ui-control-lg {"));
    assert!(css.contains("min-height: 3.25rem;"));
    assert!(css.contains(".ui-radius-pill {"));
    assert!(css.contains("border-radius: 999px;"));
    assert!(css.contains(".f-checkbox {"));
    assert!(css.contains(".m-radio {"));
    assert!(css.contains(".m-switch {"));
    assert!(css.contains("inline-size: 2.5rem;"));
}

fn assert_theme_selectors(css: &str) {
    assert!(css.contains(".f-surface {"));
    assert!(css.contains("background-image: linear-gradient(135deg, rgba(255, 255, 255, 0.34), rgba(255, 255, 255, 0.08));"));
    assert!(css.contains("backdrop-filter: blur(18px) saturate(1.15);"));
    assert!(css.contains(".f-surface-alt {"));
    assert!(css.contains("backdrop-filter: blur(28px) saturate(1.25);"));
    assert!(css.contains(".m-surface-variant {"));
    assert!(css.contains("background-image: linear-gradient(180deg, rgba(255, 255, 255, 0.42), rgba(211, 227, 253, 0.24));"));
    assert!(css.contains("background-color: #e8eef9;"));
    assert!(css.contains(".m-bg-primary-container {"));
    assert!(css.contains("background-color: #d3e3fd;"));
    assert!(css.contains("color: #041e49;"));
}

fn assert_workspace_selectors(css: &str) {
    assert!(css.contains(".f-label {"));
    assert!(css.contains(".f-divider {"));
    assert!(css.contains(".f-field {"));
    assert!(css.contains("caret-color: #0f6cbd;"));
    assert!(css.contains(".f-panel {"));
    assert!(css.contains(".f-action-primary {"));
    assert!(
        css.contains("transition-property: background-color, border-color, box-shadow, transform;")
    );
    assert!(css.contains(".f-action-subtle {"));
    assert!(css.contains(".hover\\:f-bg-hover-subtle:hover {"));
    assert!(css.contains(".focus\\:f-border-focus:focus {"));
    assert!(css.contains(".m-label {"));
    assert!(css.contains(".m-divider {"));
    assert!(css.contains(".m-field {"));
    assert!(css.contains("caret-color: #1a73e8;"));
    assert!(css.contains(".m-surface-container {"));
    assert!(css.contains("background-image: linear-gradient(180deg, rgba(255, 255, 255, 0.72), rgba(232, 240, 254, 0.24));"));
    assert!(css.contains(".m-action-primary {"));
    assert!(css.contains(".m-action-tonal {"));
    assert!(css.contains(".m-action-outlined {"));
    assert!(css.contains(".f-textarea {"));
    assert!(css.contains(".m-textarea {"));
    assert!(css.contains(".f-select {"));
    assert!(css.contains(".m-select {"));
    assert!(css.contains(".f-tab {"));
    assert!(css.contains(".m-tab {"));
    assert!(css.contains(".f-dialog {"));
    assert!(css.contains(".m-dialog {"));
    assert!(css.contains(".f-list-item {"));
    assert!(css.contains(".m-list-item {"));
    assert!(css.contains(".f-menu-item {"));
    assert!(css.contains(".m-menu-item {"));
    assert!(css.contains(".f-icon-button {"));
    assert!(css.contains(".m-icon-button {"));
    assert!(css.contains(".f-nav-item {"));
    assert!(css.contains(".m-nav-item {"));
    assert!(css.contains(".f-badge {"));
    assert!(css.contains(".m-badge {"));
    assert!(css.contains(".f-tooltip {"));
    assert!(css.contains(".m-tooltip {"));
    assert!(css.contains(".f-banner {"));
    assert!(css.contains(".m-banner {"));
    assert!(css.contains(".f-drawer {"));
    assert!(css.contains(".m-drawer {"));
    assert!(css.contains(".f-toast {"));
    assert!(css.contains(".m-toast {"));
    assert!(css.contains(".f-segmented-button {"));
    assert!(css.contains(".m-segmented-button {"));
    assert!(css.contains(".f-search-field {"));
    assert!(css.contains(".m-search-field {"));
    assert!(css.contains(".f-breadcrumb-item {"));
    assert!(css.contains(".m-breadcrumb-item {"));
    assert!(css.contains(".f-breadcrumb {"));
    assert!(css.contains(".m-breadcrumb {"));
    assert!(css.contains(".f-avatar {"));
    assert!(css.contains(".m-avatar {"));
    assert!(css.contains(".f-persona {"));
    assert!(css.contains(".m-persona {"));
    assert!(css.contains(".f-progress {"));
    assert!(css.contains(".m-progress {"));
    assert!(css.contains(".f-spinner {"));
    assert!(css.contains(".m-spinner {"));
    assert!(css.contains(".f-skeleton {"));
    assert!(css.contains(".m-skeleton {"));
    assert!(css.contains(".f-empty-state {"));
    assert!(css.contains(".m-empty-state {"));
    assert!(css.contains(".f-sheet {"));
    assert!(css.contains(".m-sheet {"));
    assert!(css.contains(".f-accordion-item-open {"));
    assert!(css.contains(".m-accordion-item-open {"));
    assert!(css.contains(".f-table-row-selected {"));
    assert!(css.contains(".m-table-row-selected {"));
    assert!(css.contains("min-width: 16rem;"));
    assert!(css.contains("padding-left: 2.5rem;"));
    assert!(css.contains("max-width: 32rem;"));
    assert!(css.contains("max-width: 28rem;"));
    assert!(css.contains("resize: vertical;"));
    assert!(css.contains("appearance: none;"));
    assert!(css.contains("animation: motif-spin 0.9s linear infinite;"));
    assert!(css.contains("animation: motif-shimmer 1.4s ease-in-out infinite;"));
    assert!(css.contains("grid-template-columns: minmax(0, 2fr) minmax(0, 1fr) auto;"));
    assert!(css.contains("flex-wrap: wrap;"));
    assert!(css.contains("border-color: #1a73e8;"));
    assert!(css.contains(".hover\\:m-bg-hover-primary:hover {"));
    assert!(css.contains(".hover\\:m-bg-hover-container:hover {"));
    assert!(css.contains(".hover\\:m-bg-hover-surface:hover {"));
    assert!(css.contains(".active\\:m-shadow-press:active {"));
    assert!(css.contains(".focus\\:m-border-focus:focus {"));
    assert!(css.contains(".focus\\:m-ring:focus {"));
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
