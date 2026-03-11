use motif_core::token::load_registry;

#[test]
fn loads_embedded_token_registry() {
    let registry = load_registry().unwrap();

    assert_eq!(
        registry.fluent.color.get("primary").map(String::as_str),
        Some("#0f6cbd")
    );
    assert_eq!(
        registry.fluent.color.get("surface-alt").map(String::as_str),
        Some("rgba(255, 255, 255, 0.52)")
    );
    assert_eq!(
        registry
            .fluent
            .typography
            .get("font-family")
            .map(String::as_str),
        Some("'Segoe UI', 'Segoe UI Variable', sans-serif")
    );
    assert_eq!(
        registry.fluent.border.get("divider").map(String::as_str),
        Some("1px solid rgba(255, 255, 255, 0.8)")
    );
    assert_eq!(
        registry.material.color.get("primary").map(String::as_str),
        Some("#1a73e8")
    );
    assert_eq!(
        registry
            .material
            .color
            .get("primary-container")
            .map(String::as_str),
        Some("#d3e3fd")
    );
    assert_eq!(
        registry.material.radius.get("pill").map(String::as_str),
        Some("999px")
    );
    assert_eq!(
        registry.material.space.get("field-pad").map(String::as_str),
        Some("0.875rem 1rem")
    );
    assert_eq!(
        registry
            .material
            .typography
            .get("font-family")
            .map(String::as_str),
        Some("'Google Sans', 'Roboto', sans-serif")
    );
}
