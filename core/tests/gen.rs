use motif_core::gen::render_stylesheet;
use motif_core::parse::parse_class_name;
use motif_core::rule::resolve_rule;

#[test]
fn renders_plain_and_variant_rules() {
    let plain = resolve_rule(&parse_class_name("f-stack").unwrap()).unwrap();
    let hover = resolve_rule(&parse_class_name("hover:f-bg-primary").unwrap()).unwrap();

    let stylesheet = render_stylesheet(&[plain, hover]);

    assert!(stylesheet.contains(".f-stack {"));
    assert!(stylesheet.contains("display: flex;"));
    assert!(stylesheet.contains(".hover\\:f-bg-primary:hover {"));
    assert!(stylesheet.contains("background-color: #0f6cbd;"));
}

#[test]
fn wraps_dark_variant_in_media_query() {
    let dark = resolve_rule(&parse_class_name("dark:m-elevation-1").unwrap()).unwrap();
    let stylesheet = render_stylesheet(&[dark]);

    assert!(stylesheet.contains("@media (prefers-color-scheme: dark) {"));
    assert!(stylesheet.contains(".dark\\:m-elevation-1 {"));
    assert!(stylesheet.contains("box-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);"));
}
