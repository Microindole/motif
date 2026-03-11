use motif_core::write::{resolve_output_path, write_stylesheet};
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
