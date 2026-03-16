use crate::basics::{has_contains, has_exact, tokens};

#[test]
fn resolves_fluent_surface_with_mica_like_traits() {
    let tokens = tokens();

    assert!(has_contains(
        &tokens,
        "f-surface",
        "background-image",
        "0.34"
    ));
    assert!(has_exact(
        &tokens,
        "f-surface",
        "backdrop-filter",
        "blur(18px) saturate(1.15)"
    ));
    let parsed = motif_core::parse::parse_class_name("f-surface").unwrap();
    let rule = motif_core::rule::resolve_rule(&parsed, &tokens).unwrap();
    assert!(rule
        .declarations
        .iter()
        .any(|decl| decl.property == "border-color"));
}

#[test]
fn resolves_fluent_surface_alt_as_acrylic_panel() {
    let tokens = tokens();

    assert!(has_contains(
        &tokens,
        "f-surface-alt",
        "backdrop-filter",
        "28px"
    ));
    assert!(has_contains(&tokens, "f-surface-alt", "box-shadow", "34px"));
    assert!(has_exact(
        &tokens,
        "f-surface-alt",
        "transition-property",
        "background-color, border-color, box-shadow, backdrop-filter"
    ));
}

#[test]
fn resolves_fluent_field_and_material_action_rules() {
    let tokens = tokens();

    assert!(has_contains(
        &tokens,
        "f-field",
        "border",
        "rgba(255, 255, 255, 0.96)"
    ));
    assert!(has_exact(
        &tokens,
        "f-field",
        "background-color",
        "rgba(255, 255, 255, 0.82)"
    ));
    assert!(has_contains(&tokens, "f-field", "background-image", "0.38"));
    assert!(has_contains(&tokens, "f-field", "backdrop-filter", "14px"));
    assert!(has_exact(&tokens, "f-field", "caret-color", "#0f6cbd"));
    assert!(has_exact(
        &tokens,
        "m-action-tonal",
        "background-color",
        "#d3e3fd"
    ));
    assert!(has_contains(
        &tokens,
        "m-action-tonal",
        "box-shadow",
        "rgba(26, 115, 232, 0.16)"
    ));
    assert!(has_contains(&tokens, "m-field", "background-image", "0.86"));
    assert!(has_exact(&tokens, "m-field", "caret-color", "#1a73e8"));
}
