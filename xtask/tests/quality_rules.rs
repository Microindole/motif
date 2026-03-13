use xtask::quality::commit::{
    evaluate_commit_message, evaluate_commit_message_for_commit, parse_pr_head_sha_from_event,
};
use xtask::quality::dependencies_parse::{
    extract_added_cargo_dependencies, extract_added_json_dependencies,
};
use xtask::quality::pr::{parse_pr_body_from_event, validate_pr_body};

#[test]
fn accepts_well_formed_commit_message() {
    let result = evaluate_commit_message("ci: tighten quality gates\n\nExplain the new guardrail.");
    assert!(result.failures.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn rejects_bad_commit_subject_shape() {
    let result = evaluate_commit_message("Update files.");
    assert!(result
        .failures
        .iter()
        .any(|item| item.contains("must match `type: description`")));
    assert!(result
        .failures
        .iter()
        .any(|item| item.contains("must not end with a period")));
}

#[test]
fn warns_when_commit_body_is_missing_for_non_trivial_change() {
    let result = evaluate_commit_message_for_commit("ci: add message checks", true);
    assert!(result.failures.is_empty());
    assert!(result
        .warnings
        .iter()
        .any(|item| item.contains("has no body")));
}

#[test]
fn skips_commit_body_warning_for_trivial_change() {
    let result = evaluate_commit_message_for_commit("docs: tweak wording", false);
    assert!(result.failures.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn parses_pr_head_sha_from_github_event_payload() {
    let payload = r###"{
  "pull_request": {
    "head": {
      "sha": "abc123def456"
    },
    "body": "## Summary\nA real summary.\n\n## Hard checks\n- [x] ok\n\n## Structure review\n- [x] ok\n\n## AI-specific review\n- [x] ok"
  }
}"###;
    let sha = parse_pr_head_sha_from_event(payload).expect("head sha should parse");
    assert_eq!(sha, "abc123def456");
}

#[test]
fn parses_pr_body_from_github_event_payload() {
    let payload = r###"{
  "pull_request": {
    "body": "## Summary\nA real summary.\n\n## Hard checks\n- [x] ok\n\n## Structure review\n- [x] ok\n\n## AI-specific review\n- [x] ok"
  }
}"###;
    let body = parse_pr_body_from_event(payload).expect("body should parse");
    assert!(body.contains("## Summary"));
    assert!(body.contains("## AI-specific review"));
}

#[test]
fn rejects_incomplete_pr_template() {
    let failures = validate_pr_body(
        "## Summary\n- [ ] scoped\n\n## Hard checks\n- [x] ok\n\n## Structure review\n- [x] ok",
    );
    assert!(failures
        .iter()
        .any(|item| item.contains("missing required section")));
    assert!(failures
        .iter()
        .any(|item| item.contains("unchecked template item")));
}

#[test]
fn rejects_pr_summary_that_only_keeps_template_checkboxes() {
    let failures = validate_pr_body(
        "## Summary\n- [x] scoped\n- [x] docs\n\n## Hard checks\n- [x] ok\n\n## Structure review\n- [x] ok\n\n## AI-specific review\n- [x] ok",
    );
    assert!(failures
        .iter()
        .any(|item| item.contains("PR Summary must include at least one non-checkbox line")));
}

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
