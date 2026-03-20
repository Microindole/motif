use crate::parse::ParsedClass;

use super::{declaration, token_declaration, Declaration};

pub(super) fn resolve(parsed: &ParsedClass) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("control", Some("sm")) => Some(control("2.25rem", "0.625rem 0.85rem", "0.875rem")),
        ("control", Some("md")) => Some(control("2.75rem", "0.78rem 0.95rem", "0.95rem")),
        ("control", Some("lg")) => Some(control("3.25rem", "0.95rem 1.2rem", "1rem")),
        ("pad", Some("sm")) => Some(vec![declaration("padding", "0.75rem")]),
        ("pad", Some("md")) => Some(vec![declaration("padding", "1rem")]),
        ("pad", Some("lg")) => Some(vec![declaration("padding", "1.25rem")]),
        ("pad", Some("xl")) => Some(vec![declaration("padding", "1.5rem")]),
        ("gap", Some("sm")) => Some(vec![declaration("gap", "0.5rem")]),
        ("gap", Some("md")) => Some(vec![declaration("gap", "0.75rem")]),
        ("gap", Some("lg")) => Some(vec![declaration("gap", "1rem")]),
        ("gap", Some("xl")) => Some(vec![declaration("gap", "1.25rem")]),
        ("density", Some("compact")) => Some(density("0.625rem", "0.5rem")),
        ("density", Some("comfortable")) => Some(density("1.125rem", "1rem")),
        ("stack", Some("inline")) => Some(stack_inline()),
        ("stack", Some("center")) => Some(stack_center()),
        ("radius", Some("sm")) => Some(vec![declaration("border-radius", "6px")]),
        ("radius", Some("md")) => Some(vec![declaration("border-radius", "10px")]),
        ("radius", Some("lg")) => Some(vec![declaration("border-radius", "14px")]),
        ("radius", Some("pill")) => Some(vec![declaration("border-radius", "999px")]),
        ("text", Some("xs")) => Some(text("0.75rem", "1.35")),
        ("text", Some("sm")) => Some(text("0.875rem", "1.4")),
        ("text", Some("md")) => Some(text("1rem", "1.5")),
        ("text", Some("lg")) => Some(text("1.125rem", "1.55")),
        _ => None,
    }
}

pub(super) fn typography(
    font_family: &str,
    size: &str,
    weight: &str,
    line_height: &str,
    tracking: &str,
    color: Option<&str>,
) -> Vec<Declaration> {
    let mut declarations = vec![
        token_declaration("font-family", font_family),
        token_declaration("font-size", size),
        token_declaration("font-weight", weight),
        token_declaration("line-height", line_height),
        token_declaration("letter-spacing", tracking),
    ];
    if let Some(color) = color {
        declarations.push(token_declaration("color", color));
    }
    declarations
}

pub(super) fn append_transition(
    declarations: &mut Vec<Declaration>,
    property: &str,
    duration: &str,
    easing: &str,
) {
    declarations.extend([
        token_declaration("transition-property", property),
        token_declaration("transition-duration", duration),
        token_declaration("transition-timing-function", easing),
    ]);
}

pub(super) fn with_transition(
    mut declarations: Vec<Declaration>,
    property: &str,
    duration: &str,
    easing: &str,
) -> Vec<Declaration> {
    append_transition(&mut declarations, property, duration, easing);
    declarations
}

pub(super) fn append_inline_action_layout(declarations: &mut Vec<Declaration>) {
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "center"),
        declaration("min-height", "2.5rem"),
        declaration("line-height", "1"),
    ]);
}

pub(super) fn append_search_field_adornment(declarations: &mut Vec<Declaration>) {
    declarations.push(declaration("padding-left", "2.5rem"));
}

pub(super) fn table_container(
    color: &str,
    background: &str,
    border: &str,
    radius: &str,
    padding: &str,
) -> Vec<Declaration> {
    vec![
        declaration("display", "grid"),
        declaration("gap", "0.5rem"),
        token_declaration("color", color),
        token_declaration("background-color", background),
        token_declaration("border", border),
        token_declaration("border-radius", radius),
        token_declaration("padding", padding),
    ]
}

pub(super) fn table_header_row(color: &str, padding: &str) -> Vec<Declaration> {
    vec![
        declaration("display", "grid"),
        declaration(
            "grid-template-columns",
            "minmax(0, 2fr) minmax(0, 1fr) auto",
        ),
        declaration("align-items", "center"),
        declaration("gap", "0.75rem"),
        token_declaration("color", color),
        token_declaration("padding", padding),
    ]
}

pub(super) fn table_data_row(
    color: &str,
    background: &str,
    border: &str,
    radius: &str,
    padding: &str,
) -> Vec<Declaration> {
    vec![
        declaration("display", "grid"),
        declaration(
            "grid-template-columns",
            "minmax(0, 2fr) minmax(0, 1fr) auto",
        ),
        declaration("align-items", "center"),
        declaration("gap", "0.75rem"),
        declaration("min-height", "3rem"),
        token_declaration("color", color),
        token_declaration("background-color", background),
        token_declaration("border", border),
        token_declaration("border-radius", radius),
        token_declaration("padding", padding),
    ]
}

pub(super) fn accordion_item_base(
    color: &str,
    background: &str,
    border: &str,
    radius: &str,
    padding: &str,
) -> Vec<Declaration> {
    vec![
        declaration("display", "grid"),
        declaration("gap", "0.75rem"),
        token_declaration("color", color),
        token_declaration("background-color", background),
        token_declaration("border", border),
        token_declaration("border-radius", radius),
        token_declaration("padding", padding),
    ]
}

pub(super) fn accordion_header_base(color: &str) -> Vec<Declaration> {
    vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "space-between"),
        token_declaration("color", color),
    ]
}

pub(super) fn accordion_content_base(color: &str) -> Vec<Declaration> {
    vec![
        token_declaration("color", color),
        declaration("display", "grid"),
        declaration("gap", "0.5rem"),
    ]
}

fn control(
    min_height: &'static str,
    padding: &'static str,
    font_size: &'static str,
) -> Vec<Declaration> {
    vec![
        declaration("min-height", min_height),
        token_declaration("padding", padding),
        declaration("font-size", font_size),
    ]
}

fn text(font_size: &'static str, line_height: &'static str) -> Vec<Declaration> {
    vec![
        declaration("font-size", font_size),
        declaration("line-height", line_height),
    ]
}

fn density(padding: &'static str, gap: &'static str) -> Vec<Declaration> {
    vec![declaration("padding", padding), declaration("gap", gap)]
}

fn stack_inline() -> Vec<Declaration> {
    vec![
        declaration("display", "flex"),
        declaration("flex-direction", "row"),
        declaration("align-items", "center"),
        declaration("flex-wrap", "wrap"),
    ]
}

fn stack_center() -> Vec<Declaration> {
    vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "center"),
    ]
}
