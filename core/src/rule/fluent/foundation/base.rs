use super::super::super::shared::append_transition;
use super::super::super::{declaration, token_declaration, Declaration};
use super::super::typography_from_tokens;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("stack", None) => stack(tokens),
        ("ring", None) => ring(tokens),
        ("bg", Some("primary")) => bg_primary(tokens),
        ("bg", Some("hover-primary")) => bg_hover_primary(tokens),
        ("bg", Some("hover-subtle")) => bg_hover_subtle(tokens),
        ("text", Some("primary")) => text_primary(tokens),
        ("text", Some("muted")) => text_muted(tokens),
        ("surface", None) => surface(tokens),
        ("surface", Some("alt")) => surface_alt(tokens),
        ("panel", None) => panel(tokens),
        ("shadow", Some("press")) => shadow_press(tokens),
        ("title", None) => title(tokens),
        ("body", None) => body(tokens),
        ("label", None) => label(tokens),
        ("border", Some("focus")) => border_focus(tokens),
        ("divider", None) => divider(tokens),
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
    let mut declarations = vec![
        token_declaration("outline-width", tokens.fluent.outline.get("focus-width")?),
        declaration("outline-style", "solid"),
        token_declaration("outline-color", tokens.fluent.color.get("primary")?),
        token_declaration("outline-offset", tokens.fluent.space.get("focus-offset")?),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
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

fn bg_hover_subtle(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "background-color",
        tokens.fluent.color.get("hover-subtle")?,
    )])
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
    let mut declarations = vec![
        token_declaration("background-color", tokens.fluent.color.get("surface")?),
        token_declaration(
            "background-image",
            tokens.fluent.effect.get("surface-tint")?,
        ),
        declaration("background-blend-mode", "screen"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad")?),
        token_declaration("border-width", tokens.fluent.border.get("subtle-width")?),
        declaration("border-style", "solid"),
        token_declaration("border-color", tokens.fluent.color.get("border")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("surface")?),
        declaration("backdrop-filter", "blur(18px) saturate(1.15)"),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn surface_alt(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration(
            "background-image",
            tokens.fluent.effect.get("surface-alt-tint")?,
        ),
        declaration("background-blend-mode", "screen"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
        token_declaration("border-width", tokens.fluent.border.get("subtle-width")?),
        declaration("border-style", "solid"),
        token_declaration("border-color", tokens.fluent.color.get("border-strong")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("surface-alt")?),
        declaration("backdrop-filter", "blur(28px) saturate(1.25)"),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn panel(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("background-color", tokens.fluent.color.get("panel")?),
        token_declaration("background-image", tokens.fluent.effect.get("panel-tint")?),
        declaration("background-blend-mode", "screen"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("border", tokens.fluent.border.get("panel")?),
        token_declaration("border-radius", tokens.fluent.radius.get("lg")?),
        token_declaration("padding", tokens.fluent.space.get("panel-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("panel")?),
        declaration("backdrop-filter", "blur(24px) saturate(1.2)"),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn shadow_press(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "box-shadow",
        tokens.fluent.shadow.get("press")?,
    )])
}

fn title(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography_from_tokens(
        tokens,
        "title-size",
        "title-weight",
        Some(tokens.fluent.color.get("text")?),
    )
}

fn body(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography_from_tokens(
        tokens,
        "body-size",
        "body-weight",
        Some(tokens.fluent.color.get("text")?),
    )
}

fn label(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography_from_tokens(
        tokens,
        "label-size",
        "label-weight",
        Some(tokens.fluent.color.get("muted")?),
    )
}

fn border_focus(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "border",
        tokens.fluent.color.get("border-focus")?,
    )])
}

fn divider(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        declaration("display", "block"),
        token_declaration("border-bottom", tokens.fluent.border.get("divider")?),
    ])
}
