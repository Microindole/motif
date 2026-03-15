use super::super::super::shared::append_transition;
use super::super::super::{declaration, token_declaration, Declaration};
use super::super::typography_from_tokens;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("badge", None) => badge(tokens),
        ("chip", None) => chip(tokens),
        ("tag", None) => tag(tokens),
        ("progress", None) => progress(tokens),
        ("spinner", None) => spinner(tokens),
        ("skeleton", None) => skeleton(tokens),
        ("empty", Some("state")) => empty_state(tokens),
        _ => None,
    }
}

fn badge(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("min-height", "1.75rem"),
        token_declaration("color", tokens.material.color.get("on-primary-container")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("primary-container")?,
        ),
        declaration("border", "0"),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        declaration("padding", "0.35rem 0.75rem"),
    ]);
    Some(declarations)
}

fn chip(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("gap", "0.5rem"),
        declaration("min-height", "2rem"),
        token_declaration("color", tokens.material.color.get("primary")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        declaration("padding", "0.55rem 0.85rem"),
    ]);
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn tag(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("min-height", "1.5rem"),
        token_declaration("color", tokens.material.color.get("primary")?),
        declaration("background-color", "transparent"),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("md")?),
        declaration("padding", "0.2rem 0.55rem"),
    ]);
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn progress(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "block"),
        declaration("inline-size", "100%"),
        declaration("block-size", "0.5rem"),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration(
            "background-image",
            tokens.material.effect.get("container-tint")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn spinner(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "inline-block"),
        declaration("inline-size", "1.25rem"),
        declaration("block-size", "1.25rem"),
        declaration("border-width", "2px"),
        declaration("border-style", "solid"),
        token_declaration(
            "border-color",
            tokens.material.color.get("surface-variant")?,
        ),
        token_declaration("border-top-color", tokens.material.color.get("primary")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        declaration("animation", "motif-spin 0.9s linear infinite"),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn skeleton(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![declaration("display", "block"), declaration("min-height", "1rem"), token_declaration("background-color", tokens.material.color.get("surface-container")?), declaration("background-image", "linear-gradient(90deg, rgba(255, 255, 255, 0.12), rgba(255, 255, 255, 0.56), rgba(211, 227, 253, 0.2))"), declaration("background-size", "200% 100%"), token_declaration("border-radius", tokens.material.radius.get("lg")?), declaration("animation", "motif-shimmer 1.4s ease-in-out infinite")];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn empty_state(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "grid"),
        declaration("justify-items", "start"),
        declaration("gap", "0.75rem"),
        token_declaration("color", tokens.material.color.get("muted")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1rem"),
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
