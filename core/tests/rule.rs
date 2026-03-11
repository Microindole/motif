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
    assert_eq!(rule.declarations.len(), 1);
    assert_eq!(rule.declarations[0].property, "background-color");
    assert_eq!(rule.declarations[0].value, "#0f6cbd");
}

#[test]
fn leaves_unknown_rule_unresolved() {
    let tokens = load_registry().unwrap();
    let parsed = parse_class_name("f-bg-secondary").unwrap();
    assert!(resolve_rule(&parsed, &tokens).is_none());
}
