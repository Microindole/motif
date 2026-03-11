use motif_core::parse::parse_class_name;
use motif_core::rule::resolve_rule;
use motif_core::token::load_registry;

#[test]
fn resolves_whitelisted_fluent_rule() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("hover:f-bg-primary").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert_eq!(rule.raw_class_name, "hover:f-bg-primary");
    assert_eq!(rule.variants, parsed.variants);
    assert_eq!(rule.utility, "bg");
    assert_eq!(rule.value.as_deref(), Some("primary"));
    assert_eq!(rule.declarations[0].property, "background-color");
    assert_eq!(rule.declarations[0].value, "#0f6cbd");
}

#[test]
fn resolves_material_on_primary_text_rule() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("m-text-on-primary").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert_eq!(rule.declarations[0].property, "color");
    assert_eq!(rule.declarations[0].value, "#ffffff");
}

#[test]
fn resolves_fluent_surface_with_mica_like_traits() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("f-surface").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert!(rule.declarations.iter().any(|decl| decl.property == "backdrop-filter"));
    assert!(rule.declarations.iter().any(|decl| decl.property == "border-color"));
}

#[test]
fn resolves_fluent_surface_alt_as_acrylic_panel() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("f-surface-alt").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert!(rule.declarations.iter().any(|decl| decl.property == "backdrop-filter" && decl.value.contains("28px")));
    assert!(rule.declarations.iter().any(|decl| decl.property == "box-shadow" && decl.value.contains("32px")));
}

#[test]
fn resolves_material_primary_container() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("m-bg-primary-container").unwrap();
    let rule = resolve_rule(&parsed, &tokens).unwrap();

    assert!(rule.declarations.iter().any(|decl| decl.property == "background-color" && decl.value == "#d3e3fd"));
    assert!(rule.declarations.iter().any(|decl| decl.property == "color" && decl.value == "#041e49"));
}

#[test]
fn resolves_fluent_field_and_material_action_rules() {
    let tokens = load_registry().unwrap();

    let fluent_field = resolve_rule(&parse_class_name("f-field").unwrap(), &tokens).unwrap();
    assert!(fluent_field.declarations.iter().any(|decl| decl.property == "border" && decl.value.contains("rgba(255, 255, 255, 0.9)")));
    assert!(fluent_field.declarations.iter().any(|decl| decl.property == "background-color" && decl.value == "rgba(255, 255, 255, 0.72)"));

    let material_action = resolve_rule(&parse_class_name("m-action-tonal").unwrap(), &tokens).unwrap();
    assert!(material_action.declarations.iter().any(|decl| decl.property == "background-color" && decl.value == "#d3e3fd"));
    assert!(material_action.declarations.iter().any(|decl| decl.property == "color" && decl.value == "#041e49"));
}

#[test]
fn leaves_unknown_rule_unresolved() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("f-bg-secondary").unwrap();
    assert!(resolve_rule(&parsed, &tokens).is_none());
}
