use super::{declaration, token_declaration, Declaration};
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("stack", None) => stack(tokens),
        ("ring", None) => ring(tokens),
        ("bg", Some("primary")) => bg_primary(tokens),
        ("bg", Some("hover-primary")) => bg_hover_primary(tokens),
        ("text", Some("primary")) => text_primary(tokens),
        ("text", Some("muted")) => text_muted(tokens),
        ("surface", None) => surface(tokens),
        ("surface", Some("alt")) => surface_alt(tokens),
        ("panel", None) => panel(tokens),
        ("shadow", Some("press")) => shadow_press(tokens),
        ("title", None) => title(tokens),
        ("body", None) => body(tokens),
        ("label", None) => label(tokens),
        ("divider", None) => divider(tokens),
        ("field", None) => field(tokens),
        ("action", Some("primary")) => action_primary(tokens),
        ("action", Some("subtle")) => action_subtle(tokens),
        _ => None,
    }
}

fn stack(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        declaration("display", "flex"),
        declaration("flex-direction", "column"),
        token_declaration("gap", tokens.fluent.space.get("md")?),
    ])
}

fn ring(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration("outline-width", tokens.fluent.outline.get("focus-width")?),
        declaration("outline-style", "solid"),
        token_declaration("outline-color", tokens.fluent.color.get("primary")?),
        token_declaration("outline-offset", tokens.fluent.space.get("focus-offset")?),
        token_declaration("transition-duration", tokens.fluent.motion.get("duration")?),
        token_declaration(
            "transition-timing-function",
            tokens.fluent.motion.get("easing")?,
        ),
    ])
}

fn bg_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration("background-color", tokens.fluent.color.get("primary")?),
        token_declaration("color", tokens.fluent.color.get("action-foreground")?),
    ])
}

fn bg_hover_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration(
            "background-color",
            tokens.fluent.color.get("hover-primary")?,
        ),
        token_declaration("color", tokens.fluent.color.get("action-foreground")?),
    ])
}

fn text_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "color",
        tokens.fluent.color.get("primary")?,
    )])
}

fn text_muted(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "color",
        tokens.fluent.color.get("muted")?,
    )])
}

fn surface(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration("background-color", tokens.fluent.color.get("surface")?),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad")?),
        token_declaration("border-width", tokens.fluent.border.get("subtle-width")?),
        declaration("border-style", "solid"),
        token_declaration("border-color", tokens.fluent.color.get("border")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("surface")?),
        declaration("backdrop-filter", "blur(18px) saturate(1.15)"),
    ])
}

fn surface_alt(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
        token_declaration("border-width", tokens.fluent.border.get("subtle-width")?),
        declaration("border-style", "solid"),
        token_declaration("border-color", tokens.fluent.color.get("border-strong")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("surface-alt")?),
        declaration("backdrop-filter", "blur(28px) saturate(1.25)"),
    ])
}

fn panel(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration("background-color", tokens.fluent.color.get("panel")?),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("border", tokens.fluent.border.get("panel")?),
        token_declaration("border-radius", tokens.fluent.radius.get("lg")?),
        token_declaration("padding", tokens.fluent.space.get("panel-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("panel")?),
        declaration("backdrop-filter", "blur(24px) saturate(1.2)"),
    ])
}

fn shadow_press(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "box-shadow",
        tokens.fluent.shadow.get("press")?,
    )])
}

fn title(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography(
        tokens,
        "title-size",
        "title-weight",
        Some(tokens.fluent.color.get("text")?),
    )
}

fn body(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography(
        tokens,
        "body-size",
        "body-weight",
        Some(tokens.fluent.color.get("text")?),
    )
}

fn label(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography(
        tokens,
        "label-size",
        "label-weight",
        Some(tokens.fluent.color.get("muted")?),
    )
}

fn divider(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        declaration("display", "block"),
        token_declaration("border-bottom", tokens.fluent.border.get("divider")?),
    ])
}

fn field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography(tokens, "body-size", "body-weight", None)?;
    declarations.extend([
        declaration("min-height", "2.75rem"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("field")?),
        token_declaration("border", tokens.fluent.border.get("field")?),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
        token_declaration("padding", tokens.fluent.space.get("field-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("field")?),
        declaration("backdrop-filter", "blur(14px) saturate(1.05)"),
        token_declaration("transition-duration", tokens.fluent.motion.get("duration")?),
        token_declaration(
            "transition-timing-function",
            tokens.fluent.motion.get("easing")?,
        ),
    ]);
    Some(declarations)
}

fn action_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "center"),
        declaration("min-height", "2.5rem"),
        declaration("line-height", "1"),
        token_declaration("color", tokens.fluent.color.get("action-foreground")?),
        token_declaration("background-color", tokens.fluent.color.get("primary")?),
        token_declaration("border", tokens.fluent.border.get("action-primary")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("action-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("action")?),
        token_declaration("transition-duration", tokens.fluent.motion.get("duration")?),
        token_declaration(
            "transition-timing-function",
            tokens.fluent.motion.get("easing")?,
        ),
    ]);
    Some(declarations)
}

fn action_subtle(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "center"),
        declaration("min-height", "2.5rem"),
        declaration("line-height", "1"),
        token_declaration("color", tokens.fluent.color.get("primary")?),
        token_declaration(
            "background-color",
            tokens.fluent.color.get("action-subtle")?,
        ),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("action-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("action-subtle")?),
        declaration("backdrop-filter", "blur(18px) saturate(1.08)"),
        token_declaration("transition-duration", tokens.fluent.motion.get("duration")?),
        token_declaration(
            "transition-timing-function",
            tokens.fluent.motion.get("easing")?,
        ),
    ]);
    Some(declarations)
}

fn typography(
    tokens: &TokenRegistry,
    size_key: &str,
    weight_key: &str,
    color: Option<&str>,
) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("font-family", tokens.fluent.typography.get("font-family")?),
        token_declaration("font-size", tokens.fluent.typography.get(size_key)?),
        token_declaration("font-weight", tokens.fluent.typography.get(weight_key)?),
        token_declaration("line-height", tokens.fluent.typography.get("line-height")?),
        token_declaration("letter-spacing", tokens.fluent.typography.get("tracking")?),
    ];
    if let Some(color) = color {
        declarations.push(token_declaration("color", color));
    }
    Some(declarations)
}
