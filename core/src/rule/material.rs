use super::{declaration, token_declaration, Declaration};
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("surface", None) => Some(vec![
            token_declaration("background-color", tokens.material.color.get("surface")?),
            token_declaration("color", tokens.material.color.get("on-surface")?),
            token_declaration("border-radius", tokens.material.radius.get("lg")?),
            declaration("padding", "1.25rem"),
            token_declaration("box-shadow", tokens.material.shadow.get("surface")?),
        ]),
        ("surface", Some("variant")) => Some(vec![
            token_declaration(
                "background-color",
                tokens.material.color.get("surface-variant")?,
            ),
            token_declaration("color", tokens.material.color.get("on-surface")?),
            token_declaration("border-radius", tokens.material.radius.get("lg")?),
            declaration("padding", "1rem"),
            token_declaration("box-shadow", tokens.material.shadow.get("container")?),
        ]),
        ("bg", Some("primary")) => Some(vec![
            token_declaration("background-color", tokens.material.color.get("primary")?),
            token_declaration("color", tokens.material.color.get("on-primary")?),
            token_declaration("border-radius", tokens.material.radius.get("pill")?),
            token_declaration(
                "transition-duration",
                tokens.material.motion.get("duration")?,
            ),
            token_declaration(
                "transition-timing-function",
                tokens.material.motion.get("easing")?,
            ),
        ]),
        ("bg", Some("primary-container")) => Some(vec![
            token_declaration(
                "background-color",
                tokens.material.color.get("primary-container")?,
            ),
            token_declaration("color", tokens.material.color.get("on-primary-container")?),
            token_declaration("border-radius", tokens.material.radius.get("lg")?),
            token_declaration("box-shadow", tokens.material.shadow.get("container")?),
        ]),
        ("text", Some("primary")) => Some(vec![token_declaration(
            "color",
            tokens.material.color.get("primary")?,
        )]),
        ("text", Some("on-primary")) => Some(vec![token_declaration(
            "color",
            tokens.material.color.get("on-primary")?,
        )]),
        ("text", Some("muted")) => Some(vec![token_declaration(
            "color",
            tokens.material.color.get("muted")?,
        )]),
        ("title", None) => Some(vec![
            token_declaration(
                "font-family",
                tokens.material.typography.get("font-family")?,
            ),
            token_declaration("font-size", tokens.material.typography.get("title-size")?),
            token_declaration(
                "font-weight",
                tokens.material.typography.get("title-weight")?,
            ),
            token_declaration(
                "line-height",
                tokens.material.typography.get("line-height")?,
            ),
            token_declaration(
                "letter-spacing",
                tokens.material.typography.get("tracking")?,
            ),
        ]),
        ("body", None) => Some(vec![
            token_declaration(
                "font-family",
                tokens.material.typography.get("font-family")?,
            ),
            token_declaration("font-size", tokens.material.typography.get("body-size")?),
            token_declaration(
                "font-weight",
                tokens.material.typography.get("body-weight")?,
            ),
            token_declaration(
                "line-height",
                tokens.material.typography.get("line-height")?,
            ),
            token_declaration(
                "letter-spacing",
                tokens.material.typography.get("tracking")?,
            ),
        ]),
        ("label", None) => Some(vec![
            token_declaration(
                "font-family",
                tokens.material.typography.get("font-family")?,
            ),
            token_declaration("font-size", tokens.material.typography.get("label-size")?),
            token_declaration(
                "font-weight",
                tokens.material.typography.get("label-weight")?,
            ),
            token_declaration(
                "line-height",
                tokens.material.typography.get("line-height")?,
            ),
            token_declaration(
                "letter-spacing",
                tokens.material.typography.get("tracking")?,
            ),
            token_declaration("color", tokens.material.color.get("muted")?),
        ]),
        ("divider", None) => Some(vec![
            declaration("display", "block"),
            token_declaration("border-bottom", tokens.material.border.get("divider")?),
        ]),
        ("field", None) => Some(vec![
            token_declaration(
                "font-family",
                tokens.material.typography.get("font-family")?,
            ),
            token_declaration("font-size", tokens.material.typography.get("body-size")?),
            token_declaration(
                "font-weight",
                tokens.material.typography.get("body-weight")?,
            ),
            token_declaration(
                "line-height",
                tokens.material.typography.get("line-height")?,
            ),
            token_declaration("color", tokens.material.color.get("on-surface")?),
            token_declaration("background-color", tokens.material.color.get("field")?),
            token_declaration("border", tokens.material.border.get("field")?),
            token_declaration("border-radius", tokens.material.radius.get("lg")?),
            token_declaration("padding", tokens.material.space.get("field-pad")?),
            token_declaration("box-shadow", tokens.material.shadow.get("field")?),
            token_declaration(
                "transition-duration",
                tokens.material.motion.get("duration")?,
            ),
            token_declaration(
                "transition-timing-function",
                tokens.material.motion.get("easing")?,
            ),
        ]),
        ("action", Some("primary")) => Some(vec![
            token_declaration(
                "font-family",
                tokens.material.typography.get("font-family")?,
            ),
            token_declaration("font-size", tokens.material.typography.get("label-size")?),
            token_declaration(
                "font-weight",
                tokens.material.typography.get("label-weight")?,
            ),
            declaration("line-height", "1"),
            token_declaration("color", tokens.material.color.get("on-primary")?),
            token_declaration("background-color", tokens.material.color.get("primary")?),
            declaration("border", "0"),
            token_declaration("border-radius", tokens.material.radius.get("pill")?),
            token_declaration("padding", tokens.material.space.get("action-pad")?),
            token_declaration("box-shadow", tokens.material.shadow.get("action")?),
            token_declaration(
                "transition-duration",
                tokens.material.motion.get("duration")?,
            ),
            token_declaration(
                "transition-timing-function",
                tokens.material.motion.get("easing")?,
            ),
        ]),
        ("action", Some("tonal")) => Some(vec![
            token_declaration(
                "font-family",
                tokens.material.typography.get("font-family")?,
            ),
            token_declaration("font-size", tokens.material.typography.get("label-size")?),
            token_declaration(
                "font-weight",
                tokens.material.typography.get("label-weight")?,
            ),
            declaration("line-height", "1"),
            token_declaration("color", tokens.material.color.get("on-primary-container")?),
            token_declaration(
                "background-color",
                tokens.material.color.get("primary-container")?,
            ),
            declaration("border", "0"),
            token_declaration("border-radius", tokens.material.radius.get("lg")?),
            token_declaration("padding", tokens.material.space.get("action-pad")?),
            token_declaration("box-shadow", tokens.material.shadow.get("container")?),
            token_declaration(
                "transition-duration",
                tokens.material.motion.get("duration")?,
            ),
            token_declaration(
                "transition-timing-function",
                tokens.material.motion.get("easing")?,
            ),
        ]),
        ("elevation", Some("1")) => Some(vec![token_declaration(
            "box-shadow",
            tokens.material.shadow.get("1")?,
        )]),
        ("shadow", Some("2")) => Some(vec![token_declaration(
            "box-shadow",
            tokens.material.shadow.get("2")?,
        )]),
        _ => None,
    }
}
