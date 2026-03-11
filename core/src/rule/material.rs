use super::{declaration, token_declaration, Declaration};
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("surface", None) => surface(tokens),
        ("surface", Some("variant")) => surface_variant(tokens),
        ("bg", Some("primary")) => bg_primary(tokens),
        ("bg", Some("primary-container")) => bg_primary_container(tokens),
        ("text", Some("primary")) => text_primary(tokens),
        ("text", Some("on-primary")) => text_on_primary(tokens),
        ("text", Some("muted")) => text_muted(tokens),
        ("title", None) => title(tokens),
        ("body", None) => body(tokens),
        ("label", None) => label(tokens),
        ("divider", None) => divider(tokens),
        ("field", None) => field(tokens),
        ("action", Some("primary")) => action_primary(tokens),
        ("action", Some("tonal")) => action_tonal(tokens),
        ("elevation", Some("1")) => elevation_1(tokens),
        ("shadow", Some("2")) => shadow_2(tokens),
        _ => None,
    }
}

fn surface(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration("background-color", tokens.material.color.get("surface")?),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1.25rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("surface")?),
    ])
}

fn surface_variant(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-variant")?,
        ),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("container")?),
    ])
}

fn bg_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
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
    ])
}

fn bg_primary_container(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration(
            "background-color",
            tokens.material.color.get("primary-container")?,
        ),
        token_declaration("color", tokens.material.color.get("on-primary-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        token_declaration("box-shadow", tokens.material.shadow.get("container")?),
    ])
}

fn text_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "color",
        tokens.material.color.get("primary")?,
    )])
}

fn text_on_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "color",
        tokens.material.color.get("on-primary")?,
    )])
}

fn text_muted(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "color",
        tokens.material.color.get("muted")?,
    )])
}

fn title(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography(
        tokens,
        "title-size",
        "title-weight",
        Some(tokens.material.color.get("on-surface")?),
    )
}

fn body(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography(
        tokens,
        "body-size",
        "body-weight",
        Some(tokens.material.color.get("on-surface")?),
    )
}

fn label(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography(
        tokens,
        "label-size",
        "label-weight",
        Some(tokens.material.color.get("muted")?),
    )
}

fn divider(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        declaration("display", "block"),
        token_declaration("border-bottom", tokens.material.border.get("divider")?),
    ])
}

fn field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography(tokens, "body-size", "body-weight", None)?;
    declarations.extend([
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
    ]);
    Some(declarations)
}

fn action_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
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
    ]);
    Some(declarations)
}

fn action_tonal(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
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
    ]);
    Some(declarations)
}

fn elevation_1(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "box-shadow",
        tokens.material.shadow.get("1")?,
    )])
}

fn shadow_2(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "box-shadow",
        tokens.material.shadow.get("2")?,
    )])
}

fn typography(
    tokens: &TokenRegistry,
    size_key: &str,
    weight_key: &str,
    color: Option<&str>,
) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration(
            "font-family",
            tokens.material.typography.get("font-family")?,
        ),
        token_declaration("font-size", tokens.material.typography.get(size_key)?),
        token_declaration("font-weight", tokens.material.typography.get(weight_key)?),
        token_declaration(
            "line-height",
            tokens.material.typography.get("line-height")?,
        ),
        token_declaration(
            "letter-spacing",
            tokens.material.typography.get("tracking")?,
        ),
    ];
    if let Some(color) = color {
        declarations.push(token_declaration("color", color));
    }
    Some(declarations)
}
