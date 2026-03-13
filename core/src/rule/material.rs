use super::shared::{append_inline_action_layout, append_transition, typography};
use super::{declaration, token_declaration, Declaration};
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("surface", None) => surface(tokens),
        ("surface", Some("container")) => surface_container(tokens),
        ("surface", Some("variant")) => surface_variant(tokens),
        ("bg", Some("primary")) => bg_primary(tokens),
        ("bg", Some("hover-primary")) => bg_color(tokens, "hover-primary"),
        ("bg", Some("hover-container")) => bg_color(tokens, "hover-container"),
        ("bg", Some("hover-surface")) => bg_color(tokens, "hover-surface"),
        ("bg", Some("primary-container")) => bg_primary_container(tokens),
        ("text", Some("primary")) => text_color(tokens, "primary"),
        ("text", Some("on-primary")) => text_color(tokens, "on-primary"),
        ("text", Some("muted")) => text_color(tokens, "muted"),
        ("ring", None) => ring(tokens),
        ("title", None) => typography_from_tokens(
            tokens,
            "title-size",
            "title-weight",
            Some(tokens.material.color.get("on-surface")?),
        ),
        ("body", None) => typography_from_tokens(
            tokens,
            "body-size",
            "body-weight",
            Some(tokens.material.color.get("on-surface")?),
        ),
        ("label", None) => typography_from_tokens(
            tokens,
            "label-size",
            "label-weight",
            Some(tokens.material.color.get("muted")?),
        ),
        ("divider", None) => divider(tokens),
        ("border", Some("focus")) => border_focus(tokens),
        ("field", None) => field(tokens),
        ("action", Some("primary")) => action_primary(tokens),
        ("action", Some("tonal")) => action_tonal(tokens),
        ("action", Some("outlined")) => action_outlined(tokens),
        ("elevation", Some("1")) => shadow(tokens, "1"),
        ("shadow", Some("2")) => shadow(tokens, "2"),
        ("shadow", Some("press")) => shadow(tokens, "press"),
        _ => None,
    }
}
fn surface(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("background-color", tokens.material.color.get("surface")?),
        token_declaration(
            "background-image",
            tokens.material.effect.get("surface-tint")?,
        ),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1.25rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("surface")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn surface_container(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration(
            "background-image",
            tokens.material.effect.get("container-tint")?,
        ),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("container-high")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn surface_variant(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-variant")?,
        ),
        token_declaration(
            "background-image",
            tokens.material.effect.get("variant-tint")?,
        ),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("container")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn bg_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("background-color", tokens.material.color.get("primary")?),
        token_declaration("color", tokens.material.color.get("on-primary")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn bg_color(tokens: &TokenRegistry, key: &str) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "background-color",
        tokens.material.color.get(key)?,
    )])
}
fn bg_primary_container(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration(
            "background-color",
            tokens.material.color.get("primary-container")?,
        ),
        token_declaration("color", tokens.material.color.get("on-primary-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        token_declaration("box-shadow", tokens.material.shadow.get("container")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn text_color(tokens: &TokenRegistry, key: &str) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "color",
        tokens.material.color.get(key)?,
    )])
}
fn ring(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("outline-width", "2px"),
        declaration("outline-style", "solid"),
        token_declaration("outline-color", tokens.material.color.get("primary")?),
        declaration("outline-offset", "2px"),
        declaration("box-shadow", "0 0 0 4px rgba(26, 115, 232, 0.16)"),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn divider(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        declaration("display", "block"),
        token_declaration("border-bottom", tokens.material.border.get("divider")?),
    ])
}
fn border_focus(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "border",
        tokens.material.border.get("focus")?,
    )])
}
fn field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "body-size", "body-weight", None)?;
    declarations.extend([
        declaration("min-height", "2.75rem"),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration("caret-color", tokens.material.color.get("primary")?),
        token_declaration("background-color", tokens.material.color.get("field")?),
        token_declaration(
            "background-image",
            tokens.material.effect.get("field-tint")?,
        ),
        token_declaration("border", tokens.material.border.get("field")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        token_declaration("padding", tokens.material.space.get("field-pad")?),
        token_declaration("box-shadow", tokens.material.shadow.get("field")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn action_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    append_inline_action_layout(&mut declarations);
    declarations.extend([
        token_declaration("color", tokens.material.color.get("on-primary")?),
        token_declaration("background-color", tokens.material.color.get("primary")?),
        declaration("border", "0"),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration("padding", tokens.material.space.get("action-pad")?),
        token_declaration("box-shadow", tokens.material.shadow.get("action")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn action_tonal(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    append_inline_action_layout(&mut declarations);
    declarations.extend([
        token_declaration("color", tokens.material.color.get("on-primary-container")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("primary-container")?,
        ),
        declaration("border", "0"),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        token_declaration("padding", tokens.material.space.get("action-pad")?),
        token_declaration("box-shadow", tokens.material.shadow.get("tonal-action")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn action_outlined(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    append_inline_action_layout(&mut declarations);
    declarations.extend([
        token_declaration("color", tokens.material.color.get("primary")?),
        token_declaration("background-color", tokens.material.color.get("surface")?),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration("padding", tokens.material.space.get("action-pad")?),
        token_declaration("box-shadow", tokens.material.shadow.get("outlined-action")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn shadow(tokens: &TokenRegistry, key: &str) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "box-shadow",
        tokens.material.shadow.get(key)?,
    )])
}
fn typography_from_tokens(
    tokens: &TokenRegistry,
    size_key: &str,
    weight_key: &str,
    color: Option<&str>,
) -> Option<Vec<Declaration>> {
    Some(typography(
        tokens.material.typography.get("font-family")?,
        tokens.material.typography.get(size_key)?,
        tokens.material.typography.get(weight_key)?,
        tokens.material.typography.get("line-height")?,
        tokens.material.typography.get("tracking")?,
        color,
    ))
}
