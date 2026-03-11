use super::{declaration, token_declaration, Declaration};
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("stack", None) => Some(vec![
            declaration("display", "flex"),
            declaration("flex-direction", "column"),
            token_declaration("gap", tokens.fluent.space.get("md")?),
        ]),
        ("ring", None) => Some(vec![
            token_declaration("outline-width", tokens.fluent.outline.get("focus-width")?),
            declaration("outline-style", "solid"),
            token_declaration("outline-color", tokens.fluent.color.get("primary")?),
            token_declaration("outline-offset", tokens.fluent.space.get("focus-offset")?),
            token_declaration("transition-duration", tokens.fluent.motion.get("duration")?),
            token_declaration(
                "transition-timing-function",
                tokens.fluent.motion.get("easing")?,
            ),
        ]),
        ("bg", Some("primary")) => Some(vec![
            token_declaration("background-color", tokens.fluent.color.get("primary")?),
            token_declaration("color", tokens.fluent.color.get("action-foreground")?),
        ]),
        ("text", Some("primary")) => Some(vec![token_declaration(
            "color",
            tokens.fluent.color.get("primary")?,
        )]),
        ("text", Some("muted")) => Some(vec![token_declaration(
            "color",
            tokens.fluent.color.get("muted")?,
        )]),
        ("surface", None) => Some(vec![
            token_declaration("background-color", tokens.fluent.color.get("surface")?),
            token_declaration("color", tokens.fluent.color.get("text")?),
            token_declaration("border-radius", tokens.fluent.radius.get("md")?),
            token_declaration("padding", tokens.fluent.space.get("surface-pad")?),
            token_declaration("border-width", tokens.fluent.border.get("subtle-width")?),
            declaration("border-style", "solid"),
            token_declaration("border-color", tokens.fluent.color.get("border")?),
            token_declaration("box-shadow", tokens.fluent.shadow.get("surface")?),
            declaration("backdrop-filter", "blur(18px) saturate(1.15)"),
        ]),
        ("surface", Some("alt")) => Some(vec![
            token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
            token_declaration("color", tokens.fluent.color.get("text")?),
            token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
            token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
            token_declaration("border-width", tokens.fluent.border.get("subtle-width")?),
            declaration("border-style", "solid"),
            token_declaration("border-color", tokens.fluent.color.get("border-strong")?),
            token_declaration("box-shadow", tokens.fluent.shadow.get("surface-alt")?),
            declaration("backdrop-filter", "blur(28px) saturate(1.25)"),
        ]),
        ("title", None) => Some(vec![
            token_declaration("font-family", tokens.fluent.typography.get("font-family")?),
            token_declaration("font-size", tokens.fluent.typography.get("title-size")?),
            token_declaration("font-weight", tokens.fluent.typography.get("title-weight")?),
            token_declaration("line-height", tokens.fluent.typography.get("line-height")?),
            token_declaration("letter-spacing", tokens.fluent.typography.get("tracking")?),
        ]),
        ("body", None) => Some(vec![
            token_declaration("font-family", tokens.fluent.typography.get("font-family")?),
            token_declaration("font-size", tokens.fluent.typography.get("body-size")?),
            token_declaration("font-weight", tokens.fluent.typography.get("body-weight")?),
            token_declaration("line-height", tokens.fluent.typography.get("line-height")?),
            token_declaration("letter-spacing", tokens.fluent.typography.get("tracking")?),
        ]),
        ("label", None) => Some(vec![
            token_declaration("font-family", tokens.fluent.typography.get("font-family")?),
            token_declaration("font-size", tokens.fluent.typography.get("label-size")?),
            token_declaration("font-weight", tokens.fluent.typography.get("label-weight")?),
            token_declaration("line-height", tokens.fluent.typography.get("line-height")?),
            token_declaration("letter-spacing", tokens.fluent.typography.get("tracking")?),
            token_declaration("color", tokens.fluent.color.get("muted")?),
        ]),
        ("divider", None) => Some(vec![
            declaration("display", "block"),
            token_declaration("border-bottom", tokens.fluent.border.get("divider")?),
        ]),
        ("field", None) => Some(vec![
            token_declaration("font-family", tokens.fluent.typography.get("font-family")?),
            token_declaration("font-size", tokens.fluent.typography.get("body-size")?),
            token_declaration("font-weight", tokens.fluent.typography.get("body-weight")?),
            token_declaration("line-height", tokens.fluent.typography.get("line-height")?),
            token_declaration("color", tokens.fluent.color.get("text")?),
            token_declaration("background-color", tokens.fluent.color.get("field")?),
            token_declaration("border", tokens.fluent.border.get("field")?),
            token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
            token_declaration("padding", tokens.fluent.space.get("field-pad")?),
            token_declaration("box-shadow", tokens.fluent.shadow.get("field")?),
            token_declaration("transition-duration", tokens.fluent.motion.get("duration")?),
            token_declaration(
                "transition-timing-function",
                tokens.fluent.motion.get("easing")?,
            ),
        ]),
        ("action", Some("primary")) => Some(vec![
            token_declaration("font-family", tokens.fluent.typography.get("font-family")?),
            token_declaration("font-size", tokens.fluent.typography.get("label-size")?),
            token_declaration("font-weight", tokens.fluent.typography.get("label-weight")?),
            declaration("line-height", "1"),
            token_declaration("color", tokens.fluent.color.get("action-foreground")?),
            token_declaration("background-color", tokens.fluent.color.get("primary")?),
            declaration("border", "0"),
            token_declaration("border-radius", tokens.fluent.radius.get("md")?),
            token_declaration("padding", tokens.fluent.space.get("action-pad")?),
            token_declaration("box-shadow", tokens.fluent.shadow.get("action")?),
            token_declaration("transition-duration", tokens.fluent.motion.get("duration")?),
            token_declaration(
                "transition-timing-function",
                tokens.fluent.motion.get("easing")?,
            ),
        ]),
        _ => None,
    }
}
