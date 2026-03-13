use motif_core::token::load_registry;

#[test]
fn loads_embedded_token_registry() {
    let registry = load_registry().expect("token registry should load from embedded json");

    assert_eq!(
        registry.fluent.color.get("surface-alt").map(String::as_str),
        Some("rgba(255, 255, 255, 0.58)")
    );
    assert_eq!(
        registry.material.color.get("primary").map(String::as_str),
        Some("#1a73e8")
    );
    assert_eq!(
        registry.material.space.get("field-pad").map(String::as_str),
        Some("0.9rem 1rem")
    );
}
