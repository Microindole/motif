use super::super::super::{declaration, token_declaration, Declaration};
use super::super::typography_from_tokens;
use super::super::with_effect_transition;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("list", Some("item")) => list_item(tokens),
        ("menu", Some("item")) => menu_item(tokens),
        ("icon", Some("button")) => icon_button(tokens),
        ("nav", Some("item")) => nav_item(tokens),
        ("tooltip", None) => tooltip(tokens),
        ("banner", None) => banner(tokens),
        ("toast", None) => toast(tokens),
        _ => None,
    }
}

fn list_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let declarations = vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "space-between"),
        declaration("gap", "0.75rem"),
        declaration("min-height", "3rem"),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration(
            "background-image",
            tokens.material.effect.get("container-tint")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "0.85rem 1rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("container")?),
    ];
    with_effect_transition(tokens, declarations, "state-transition")
}

fn menu_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = list_item(tokens)?;
    declarations.push(declaration("min-height", "2.5rem"));
    declarations.push(declaration("cursor", "pointer"));
    Some(declarations)
}

fn icon_button(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let declarations = vec![
        declaration("display", "inline-grid"),
        declaration("place-items", "center"),
        declaration("inline-size", "2.5rem"),
        declaration("block-size", "2.5rem"),
        token_declaration("color", tokens.material.color.get("primary")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration("box-shadow", tokens.material.shadow.get("outlined-action")?),
    ];
    with_effect_transition(tokens, declarations, "state-transition")
}

fn nav_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("gap", "0.5rem"),
        declaration("min-height", "2.5rem"),
        token_declaration("color", tokens.material.color.get("primary")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration(
            "background-image",
            tokens.material.effect.get("container-tint")?,
        ),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration("padding", tokens.material.space.get("action-pad")?),
        token_declaration("box-shadow", tokens.material.shadow.get("outlined-action")?),
    ]);
    with_effect_transition(tokens, declarations, "state-transition")
}

fn tooltip(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "0.5rem 0.75rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("container")?),
    ]);
    Some(declarations)
}

fn banner(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let declarations = vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "space-between"),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "0.85rem 1rem"),
    ];
    with_effect_transition(tokens, declarations, "state-transition")
}

fn toast(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let declarations = vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "space-between"),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "0.85rem 1rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("container")?),
    ];
    with_effect_transition(tokens, declarations, "state-transition")
}
