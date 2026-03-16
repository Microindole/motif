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
fn leaves_unknown_rule_unresolved() {
    let tokens = tokens();
    let parsed = parse_class_name("f-bg-secondary").unwrap();
    assert!(resolve_rule(&parsed, &tokens).is_none());
}
