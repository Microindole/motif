use motif_core::parse::parse_class_name;
use motif_core::rule::resolve_rule;
use motif_core::token::{load_registry, TokenRegistry};

pub(crate) fn tokens() -> TokenRegistry {
    load_registry().unwrap()
}

pub(crate) fn has_exact(
    tokens: &TokenRegistry,
    class_name: &str,
    property: &str,
    value: &str,
) -> bool {
    let parsed = parse_class_name(class_name).unwrap();
    let rule = resolve_rule(&parsed, tokens).unwrap();
    rule.declarations
        .iter()
        .any(|decl| decl.property == property && decl.value == value)
}

pub(crate) fn has_contains(
    tokens: &TokenRegistry,
    class_name: &str,
    property: &str,
    needle: &str,
) -> bool {
    let parsed = parse_class_name(class_name).unwrap();
    let rule = resolve_rule(&parsed, tokens).unwrap();
    rule.declarations
        .iter()
        .any(|decl| decl.property == property && decl.value.contains(needle))
}

#[test]
fn resolves_whitelisted_fluent_rule() {
    let tokens = tokens();
    let parsed = parse_class_name("hover:f-bg-hover-primary").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert_eq!(rule.raw_class_name, "hover:f-bg-hover-primary");
    assert_eq!(rule.variants, parsed.variants);
    assert_eq!(rule.utility, "bg");
    assert_eq!(rule.value.as_deref(), Some("hover-primary"));
    assert_eq!(rule.declarations[0].property, "background-color");
    assert_eq!(rule.declarations[0].value, "#115ea3");
}

#[test]
fn resolves_material_on_primary_text_rule() {
    let tokens = tokens();
    let parsed = parse_class_name("m-text-on-primary").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert_eq!(rule.declarations[0].property, "color");
    assert_eq!(rule.declarations[0].value, "#ffffff");
}

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
    let parsed = parse_class_name("f-surface").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();
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
fn resolves_material_primary_container() {
    let tokens = tokens();

    assert!(has_exact(
        &tokens,
        "m-bg-primary-container",
        "background-color",
        "#d3e3fd"
    ));
    assert!(has_exact(
        &tokens,
        "m-bg-primary-container",
        "color",
        "#041e49"
    ));
}

#[test]
fn resolves_workspace_fluent_utilities() {
    let tokens = tokens();

    assert!(has_exact(
        &tokens,
        "f-panel",
        "background-color",
        "rgba(255, 255, 255, 0.74)"
    ));
    assert!(has_contains(&tokens, "f-panel", "background-image", "0.44"));
    assert!(has_contains(&tokens, "f-panel", "backdrop-filter", "24px"));
    assert!(has_contains(
        &tokens,
        "f-action-subtle",
        "border",
        "rgba(255, 255, 255, 0.98)"
    ));
    assert!(has_contains(
        &tokens,
        "f-action-subtle",
        "box-shadow",
        "20px"
    ));
    assert!(has_exact(
        &tokens,
        "f-action-subtle",
        "transition-property",
        "background-color, border-color, box-shadow, transform"
    ));
    assert!(has_contains(
        &tokens,
        "f-shadow-press",
        "box-shadow",
        "inset 0 1px 1px"
    ));
    assert!(has_exact(
        &tokens,
        "hover:f-bg-hover-subtle",
        "background-color",
        "rgba(255, 255, 255, 0.82)"
    ));
    assert!(has_exact(
        &tokens,
        "focus:f-border-focus",
        "border",
        "rgba(15, 108, 189, 0.72)"
    ));
}

#[test]
fn resolves_workspace_material_utilities() {
    let tokens = tokens();

    assert!(has_exact(
        &tokens,
        "hover:m-bg-hover-primary",
        "background-color",
        "#1765cc"
    ));
    assert!(has_contains(
        &tokens,
        "m-shadow-press",
        "box-shadow",
        "inset 0 1px 0"
    ));
    assert!(has_contains(&tokens, "m-ring", "box-shadow", "0 0 0 4px"));
    assert!(has_exact(
        &tokens,
        "m-ring",
        "transition-property",
        "background-color, color, box-shadow, border-color"
    ));
    assert!(has_exact(
        &tokens,
        "m-surface-container",
        "background-color",
        "#f5f7fb"
    ));
    assert!(has_contains(
        &tokens,
        "m-surface-container",
        "background-image",
        "0.72"
    ));
    assert!(has_exact(
        &tokens,
        "m-surface-container",
        "border",
        "1px solid #dbe3f0"
    ));
    assert!(has_exact(
        &tokens,
        "m-action-outlined",
        "border",
        "1px solid #b6c3d6"
    ));
    assert!(has_exact(
        &tokens,
        "m-action-outlined",
        "min-height",
        "2.5rem"
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

#[test]
fn leaves_unknown_rule_unresolved() {
    let tokens = tokens();
    let parsed = parse_class_name("f-bg-secondary").unwrap();
    assert!(resolve_rule(&parsed, &tokens).is_none());
}

#[test]
fn resolves_universal_size_utilities() {
    let tokens = tokens();

    assert!(has_exact(&tokens, "ui-control-lg", "min-height", "3.25rem"));
    assert!(has_exact(
        &tokens,
        "ui-control-lg",
        "padding",
        "0.95rem 1.2rem"
    ));
    let parsed = parse_class_name("ui-radius-pill").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();
    assert_eq!(rule.declarations[0].property, "border-radius");
    assert_eq!(rule.declarations[0].value, "999px");
    assert!(has_exact(&tokens, "ui-text-sm", "font-size", "0.875rem"));
}

#[test]
fn resolves_selection_controls_for_both_presets() {
    let tokens = tokens();

    assert!(has_exact(&tokens, "f-checkbox", "appearance", "none"));
    assert!(has_exact(&tokens, "f-checkbox", "inline-size", "1.125rem"));
    assert!(has_exact(&tokens, "f-checkbox", "border-radius", "6px"));
    assert!(has_exact(&tokens, "m-radio", "border-radius", "999px"));
    assert!(has_exact(&tokens, "m-switch", "inline-size", "2.5rem"));
    assert!(has_exact(
        &tokens,
        "m-switch",
        "background-color",
        "#f5f7fb"
    ));
}
