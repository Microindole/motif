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
        ("radius", Some("sm")) => Some(vec![declaration("border-radius", "6px")]),
        ("radius", Some("md")) => Some(vec![declaration("border-radius", "10px")]),
        ("radius", Some("lg")) => Some(vec![declaration("border-radius", "14px")]),
        ("radius", Some("pill")) => Some(vec![declaration("border-radius", "999px")]),
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

pub(super) fn append_inline_action_layout(declarations: &mut Vec<Declaration>) {
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "center"),
        declaration("min-height", "2.5rem"),
        declaration("line-height", "1"),
    ]);
}

fn control(
    min_height: &'static str,
    padding: &'static str,
    font_size: &'static str,
) -> Vec<Declaration> {
    vec![
        declaration("min-height", min_height),
        declaration("padding", padding),
        declaration("font-size", font_size),
    ]
}

fn text(font_size: &'static str, line_height: &'static str) -> Vec<Declaration> {
    vec![
        declaration("font-size", font_size),
        declaration("line-height", line_height),
    ]
}
