use motif_core::scan::{scan_root, scan_sources, SourceInput};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn extracts_prefixed_classes_from_mixed_markup() {
    let root = unique_temp_dir();
    fs::create_dir_all(&root).unwrap();
    fs::write(
        root.join("index.html"),
        r#"
        <div class="f-text-primary hover:f-bg-primary dark:m-elevation-1 plain-class">
          <button className={'focus:f-ring active:m-shadow-2 ui-control-lg'} />
        </div>
        "#,
    )
    .unwrap();

    let result = scan_root(&root).unwrap();
    let actual: Vec<_> = result.class_names.into_iter().collect();

    assert_eq!(
        actual,
        vec![
            "active:m-shadow-2",
            "dark:m-elevation-1",
            "f-text-primary",
            "focus:f-ring",
            "hover:f-bg-primary",
            "ui-control-lg",
        ]
    );

    fs::remove_dir_all(root).unwrap();
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
    assert_eq!(result.entries.len(), 2);
    assert!(result
        .entries
        .iter()
        .any(|entry| entry.path.ends_with("app.tsx") && entry.class_names.contains("f-stack")));
    assert_eq!(
        actual,
        vec!["f-stack", "focus:f-ring", "hover:f-bg-primary", "m-surface"]
    );

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn scans_in_memory_sources_with_file_level_results() {
    let result = scan_sources(&[
        SourceInput {
            path: PathBuf::from("src/app.tsx"),
            content: "<div className=\"f-stack hover:f-bg-primary\"></div>".to_string(),
        },
        SourceInput {
            path: PathBuf::from("src/panel.vue"),
            content: "<section class=\"m-surface focus:m-ring\"></section>".to_string(),
        },
    ]);

    assert_eq!(result.files.len(), 2);
    assert_eq!(result.entries.len(), 2);
    assert!(result
        .entries
        .iter()
        .any(|entry| entry.path == Path::new("src/app.tsx")
            && entry.class_names.contains("hover:f-bg-primary")));
    assert!(result
        .entries
        .iter()
        .any(|entry| entry.path == Path::new("src/panel.vue")
            && entry.class_names.contains("focus:m-ring")));
}

fn unique_temp_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("motif-scan-test-{stamp}"))
}
