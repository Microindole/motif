use super::{declaration, token_declaration, Declaration};

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
