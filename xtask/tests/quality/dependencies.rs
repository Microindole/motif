use xtask::quality::dependencies_parse::{
    extract_added_cargo_dependencies, extract_added_json_dependencies,
};

#[test]
fn extracts_added_cargo_dependencies_from_diff() {
    let diff = vec![
        "@@ -1,2 +1,4 @@".to_string(),
        " [dependencies]".to_string(),
        "+serde = \"1.0\"".to_string(),
        "+serde_json = \"1.0\"".to_string(),
    ];
    let names = extract_added_cargo_dependencies(&diff);
    assert_eq!(names, vec!["serde", "serde_json"]);
}

#[test]
fn extracts_added_json_dependencies_only_from_dependency_sections() {
    let diff = vec![
        "@@ -1,6 +1,9 @@".to_string(),
        " {".to_string(),
        "   \"name\": \"demo\",".to_string(),
        "   \"dependencies\": {".to_string(),
        "+    \"react\": \"^19.0.0\",".to_string(),
        "+    \"vite\": \"^7.0.0\"".to_string(),
        "   },".to_string(),
        "+  \"note\": \"not a dependency\"".to_string(),
        " }".to_string(),
    ];
    let names = extract_added_json_dependencies(&diff);
    assert_eq!(names, vec!["react", "vite"]);
}
