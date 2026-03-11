use motif_core::parse::{parse_class_name, Family, ParseError, Variant};

#[test]
fn parses_variants_family_utility_and_value() {
    let parsed = parse_class_name("dark:hover:f-bg-primary").unwrap();

    assert_eq!(parsed.variants, vec![Variant::Dark, Variant::Hover]);
    assert_eq!(parsed.family, Family::Fluent);
    assert_eq!(parsed.utility, "bg");
    assert_eq!(parsed.value.as_deref(), Some("primary"));
}

#[test]
fn parses_value_less_utility() {
    let parsed = parse_class_name("m-surface").unwrap();

    assert_eq!(parsed.family, Family::Material);
    assert_eq!(parsed.utility, "surface");
    assert_eq!(parsed.value, None);
}

#[test]
fn rejects_unknown_variants() {
    let error = parse_class_name("sm:f-bg-primary").unwrap_err();
    assert_eq!(error, ParseError::InvalidVariant("sm".to_string()));
}
