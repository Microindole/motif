use crate::basics::{has_contains, has_exact, tokens};

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
fn resolves_universal_size_utilities() {
    let tokens = tokens();

    assert!(has_exact(&tokens, "ui-control-lg", "min-height", "3.25rem"));
    assert!(has_exact(
        &tokens,
        "ui-control-lg",
        "padding",
        "0.95rem 1.2rem"
    ));
    let parsed = motif_core::parse::parse_class_name("ui-radius-pill").unwrap();
    let rule = motif_core::rule::resolve_rule(&parsed, &tokens).unwrap();
    assert_eq!(rule.declarations[0].property, "border-radius");
    assert_eq!(rule.declarations[0].value, "999px");
    assert!(has_exact(&tokens, "ui-text-sm", "font-size", "0.875rem"));
    assert!(has_exact(&tokens, "ui-gap-lg", "gap", "1rem"));
}

#[test]
fn resolves_list_and_navigation_shell_traits() {
    let tokens = tokens();

    assert!(has_exact(&tokens, "f-drawer", "gap", "1rem"));
    assert!(has_contains(&tokens, "f-drawer", "backdrop-filter", "24px"));
    assert!(has_contains(
        &tokens,
        "f-list-item",
        "background-image",
        "0.48"
    ));
    assert!(has_exact(
        &tokens,
        "f-nav-item",
        "background-color",
        "rgba(255, 255, 255, 0.7)"
    ));
    assert!(has_exact(&tokens, "m-drawer", "gap", "1rem"));
    assert!(has_contains(
        &tokens,
        "m-list-item",
        "background-image",
        "0.72"
    ));
    assert!(has_exact(
        &tokens,
        "m-nav-item",
        "border",
        "1px solid #b6c3d6"
    ));
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
