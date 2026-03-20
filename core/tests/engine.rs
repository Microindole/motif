use motif_core::engine::compile_sources;
use motif_core::scan::SourceInput;
use motif_core::token::load_registry;
use std::path::{Path, PathBuf};

#[test]
fn compiles_in_memory_sources_for_plugin_like_entry() {
    let tokens = load_registry().unwrap();
    let result = compile_sources(
        &[
            SourceInput {
                path: PathBuf::from("src/app.tsx"),
                content:
                    "<button className=\"f-action-primary hover:f-bg-hover-primary\">Run</button>"
                        .to_string(),
            },
            SourceInput {
                path: PathBuf::from("src/card.vue"),
                content: "<section class=\"m-surface-container\"></section>".to_string(),
            },
        ],
        &tokens,
    );

    assert_eq!(result.scan_result.files.len(), 2);
    assert!(result
        .scan_result
        .entries
        .iter()
        .any(|entry| entry.path == Path::new("src/app.tsx")));
    assert!(
        result.stylesheet.contains(".f-action-primary {")
            && result
                .stylesheet
                .contains(".hover\\:f-bg-hover-primary:hover {")
            && result.stylesheet.contains(".m-surface-container {")
    );
}
