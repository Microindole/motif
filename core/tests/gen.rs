use motif_core::gen::render_stylesheet;
use motif_core::parse::parse_class_name;
use motif_core::rule::resolve_rule;
use motif_core::token::load_registry;

#[test]
fn renders_plain_and_variant_rules() {
    let tokens = load_registry().unwrap();
    let plain = resolve_rule(&parse_class_name("f-stack").unwrap(), &tokens).unwrap();
    let hover = resolve_rule(
        &parse_class_name("hover:f-bg-hover-primary").unwrap(),
        &tokens,
    )
    .unwrap();

    let stylesheet = render_stylesheet(&[plain, hover]);

    assert!(stylesheet.contains(".f-stack {"));
    assert!(stylesheet.contains("display: flex;"));
    assert!(stylesheet.contains("gap: 1rem;"));
    assert!(stylesheet.contains(".hover\\:f-bg-hover-primary:hover {"));
    assert!(stylesheet.contains("background-color: #115ea3;"));
}

#[test]
fn wraps_dark_variant_in_media_query() {
    let tokens = load_registry().unwrap();
    let dark = resolve_rule(&parse_class_name("dark:m-elevation-1").unwrap(), &tokens).unwrap();
    let hover = resolve_rule(
        &parse_class_name("hover:m-bg-hover-primary").unwrap(),
        &tokens,
    )
    .unwrap();
    let ring = resolve_rule(&parse_class_name("focus:m-ring").unwrap(), &tokens).unwrap();
    let stylesheet = render_stylesheet(&[dark, hover, ring]);

    assert!(stylesheet.contains("@media (prefers-color-scheme: dark) {"));
    assert!(stylesheet.contains(".dark\\:m-elevation-1 {"));
    assert!(stylesheet.contains("box-shadow: 0 1px 2px rgba(60, 64, 67, 0.24);"));
    assert!(stylesheet.contains(".hover\\:m-bg-hover-primary:hover {"));
    assert!(stylesheet.contains("background-color: #1765cc;"));
    assert!(stylesheet.contains(".focus\\:m-ring:focus {"));
    assert!(stylesheet.contains("box-shadow: 0 0 0 4px rgba(26, 115, 232, 0.16);"));
}
