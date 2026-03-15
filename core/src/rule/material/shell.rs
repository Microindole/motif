use super::super::shared::append_transition;
use super::super::{declaration, token_declaration, Declaration};
use super::foundation::{field, surface_container};
use super::typography_from_tokens;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("dialog", None) => dialog(tokens),
        ("list", Some("item")) => list_item(tokens),
        ("menu", Some("item")) => menu_item(tokens),
        ("icon", Some("button")) => icon_button(tokens),
        ("nav", Some("item")) => nav_item(tokens),
        ("tooltip", None) => tooltip(tokens),
        ("banner", None) => banner(tokens),
        ("drawer", None) => drawer(tokens),
        ("toast", None) => toast(tokens),
        ("segmented", Some("button")) => segmented_button(tokens),
        ("search", Some("field")) => search_field(tokens),
        ("breadcrumb", None) => breadcrumb(tokens),
        ("breadcrumb", Some("item")) => breadcrumb_item(tokens),
        ("avatar", None) => avatar(tokens),
        ("persona", None) => persona(tokens),
        ("sheet", None) => sheet(tokens),
        ("sheet", Some("side")) => sheet_side(tokens),
        ("sheet", Some("bottom")) => sheet_bottom(tokens),
        _ => None,
    }
}

fn dialog(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = surface_container(tokens)?;
    declarations.push(declaration("max-width", "32rem"));
    declarations.push(declaration("width", "100%"));
    Some(declarations)
}

fn list_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "space-between"),
        declaration("min-height", "3rem"),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "0.85rem 1rem"),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
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
        token_declaration("color", tokens.material.color.get("primary")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration("box-shadow", tokens.material.shadow.get("outlined-action")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn nav_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("min-height", "2.5rem"),
        token_declaration("color", tokens.material.color.get("primary")?),
        declaration("background-color", "transparent"),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration("padding", tokens.material.space.get("action-pad")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
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
    let mut declarations = vec![
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
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn drawer(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "flex"),
        declaration("flex-direction", "column"),
        declaration("min-width", "16rem"),
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

fn toast(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
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
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn segmented_button(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "center"),
        declaration("min-height", "2.5rem"),
        token_declaration("color", tokens.material.color.get("primary")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration("padding", tokens.material.space.get("action-pad")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn search_field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = field(tokens)?;
    declarations.push(declaration("padding-left", "2.5rem"));
    Some(declarations)
}

fn breadcrumb(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("flex-wrap", "wrap"),
        declaration("gap", "0.5rem"),
        token_declaration("color", tokens.material.color.get("muted")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn breadcrumb_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        token_declaration("color", tokens.material.color.get("muted")?),
    ]);
    Some(declarations)
}

fn avatar(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "inline-grid"),
        declaration("place-items", "center"),
        declaration("inline-size", "2.5rem"),
        declaration("block-size", "2.5rem"),
        token_declaration("color", tokens.material.color.get("on-primary-container")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("primary-container")?,
        ),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        declaration("border", "0"),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn persona(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("gap", "0.75rem"),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "0.85rem 1rem"),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn sheet(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = surface_container(tokens)?;
    declarations.push(declaration("max-width", "28rem"));
    declarations.push(declaration("width", "100%"));
    declarations.push(declaration("margin-inline", "auto"));
    Some(declarations)
}

fn sheet_side(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = sheet(tokens)?;
    declarations.push(declaration("min-height", "100%"));
    declarations.push(declaration("margin-right", "0"));
    declarations.push(declaration("border-top-right-radius", "0"));
    declarations.push(declaration("border-bottom-right-radius", "0"));
    Some(declarations)
}

fn sheet_bottom(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = sheet(tokens)?;
    declarations.push(declaration("max-width", "36rem"));
    declarations.push(declaration("margin-bottom", "0"));
    declarations.push(declaration("border-bottom-left-radius", "0"));
    declarations.push(declaration("border-bottom-right-radius", "0"));
    Some(declarations)
}
