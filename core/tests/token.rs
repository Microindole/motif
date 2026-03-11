use motif_core::token::load_registry;

#[test]
fn loads_embedded_token_registry() {
    let registry = load_registry().unwrap();

    assert_eq!(registry.fluent.color.get("primary").map(String::as_str), Some("#0f6cbd"));
    assert_eq!(registry.material.shadow.get("2").map(String::as_str), Some("0 2px 6px rgba(0, 0, 0, 0.24)"));
}
