use super::super::super::shared::append_transition;
use super::super::super::{declaration, token_declaration, Declaration};
use super::super::typography_from_tokens;
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
    let mut declarations = vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "space-between"),
        declaration("gap", "0.75rem"),
        declaration("min-height", "3rem"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration(
            "background-image",
            tokens.fluent.effect.get("surface-alt-tint")?,
        ),
        declaration("background-blend-mode", "screen"),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("surface-alt")?),
        declaration("backdrop-filter", "blur(18px) saturate(1.08)"),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn menu_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = list_item(tokens)?;
    declarations.push(declaration("min-height", "2.5rem"));
    declarations.push(declaration("cursor", "pointer"));
    Some(declarations)
}

fn icon_button(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "inline-grid"),
        declaration("place-items", "center"),
        declaration("inline-size", "2.5rem"),
        declaration("block-size", "2.5rem"),
        token_declaration("color", tokens.fluent.color.get("primary")?),
        token_declaration(
            "background-color",
            tokens.fluent.color.get("action-subtle")?,
        ),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("action-subtle")?),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn nav_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("gap", "0.5rem"),
        declaration("min-height", "2.5rem"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration(
            "background-color",
            tokens.fluent.color.get("action-subtle")?,
        ),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("action-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("action-subtle")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn tooltip(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("surface-alt")?),
    ]);
    Some(declarations)
}

fn banner(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "space-between"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("panel")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn toast(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "space-between"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("surface-alt")?),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
