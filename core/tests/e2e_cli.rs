#[path = "e2e/assertions.rs"]
mod assertions;
#[path = "e2e/support.rs"]
mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use assertions::{
    assert_case_selectors, assert_demo_selectors, assert_theme_selectors, assert_variant_selectors,
    assert_workspace_selectors,
};
use support::{cleanup_file, repo_root, temp_output_path};

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
    for demo_dir in demo_dirs(&repo_root) {
        assert_demo_output(&demo_dir);
    }
}

fn demo_dirs(repo_root: &Path) -> Vec<PathBuf> {
    vec![
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
    ]
}

fn assert_demo_output(demo_dir: &Path) {
    let label = demo_label(demo_dir);
    let output = temp_output_path(&label);
    let result = Command::new(env!("CARGO_BIN_EXE_motif"))
        .arg(demo_dir)
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

fn demo_label(demo_dir: &Path) -> String {
    demo_dir
        .components()
        .rev()
        .take(2)
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("-")
}
