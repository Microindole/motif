use super::super::super::{declaration, token_declaration, Declaration};
use super::super::foundation::{field, surface_container};
use super::super::typography_from_tokens;
use super::super::with_effect_transition;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("dialog", None) => dialog(tokens),
        ("drawer", None) => drawer(tokens),
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

fn drawer(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let declarations = vec![
        declaration("display", "flex"),
        declaration("flex-direction", "column"),
        declaration("gap", "1rem"),
        declaration("min-width", "16rem"),
        declaration("width", "100%"),
        declaration("align-self", "stretch"),
        declaration("overflow", "hidden"),
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
    with_effect_transition(tokens, declarations, "state-transition")
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
    with_effect_transition(tokens, declarations, "state-transition")
}

fn search_field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = field(tokens)?;
    declarations.push(declaration("padding-left", "2.5rem"));
    Some(declarations)
}

fn breadcrumb(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let declarations = vec![
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("flex-wrap", "wrap"),
        declaration("gap", "0.5rem"),
        token_declaration("color", tokens.material.color.get("muted")?),
    ];
    with_effect_transition(tokens, declarations, "transition")
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
    let declarations = vec![
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
    with_effect_transition(tokens, declarations, "state-transition")
}

fn persona(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let declarations = vec![
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
    with_effect_transition(tokens, declarations, "state-transition")
}

fn sheet(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = surface_container(tokens)?;
    declarations.push(declaration("max-width", "28rem"));
    declarations.push(declaration("width", "100%"));
    declarations.push(declaration("align-self", "stretch"));
    declarations.push(declaration("margin-inline", "auto"));
    declarations.push(declaration("overflow", "hidden"));
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
    declarations.push(declaration("width", "100%"));
    declarations.push(declaration("border-bottom-left-radius", "0"));
    declarations.push(declaration("border-bottom-right-radius", "0"));
    Some(declarations)
}
