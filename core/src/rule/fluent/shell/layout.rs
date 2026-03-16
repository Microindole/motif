use super::super::super::shared::{append_search_field_adornment, append_transition};
use super::super::super::{declaration, token_declaration, Declaration};
use super::super::typography_from_tokens;
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
        declaration("max-width", "32rem"),
        declaration("width", "100%"),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn drawer(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "flex"),
        declaration("flex-direction", "column"),
        declaration("gap", "1rem"),
        declaration("min-width", "16rem"),
        declaration("width", "100%"),
        declaration("align-self", "stretch"),
        declaration("overflow", "hidden"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("panel")?),
        token_declaration("background-image", tokens.fluent.effect.get("panel-tint")?),
        declaration("background-blend-mode", "screen"),
        token_declaration("border", tokens.fluent.border.get("panel")?),
        token_declaration("border-radius", tokens.fluent.radius.get("lg")?),
        token_declaration("padding", tokens.fluent.space.get("panel-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("panel")?),
        declaration("backdrop-filter", "blur(24px) saturate(1.18)"),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
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
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("action-pad")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn search_field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "body-size", "body-weight", None)?;
    declarations.extend([
        declaration("min-height", "2.75rem"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("caret-color", tokens.fluent.color.get("primary")?),
        token_declaration("background-color", tokens.fluent.color.get("field")?),
        token_declaration("background-image", tokens.fluent.effect.get("field-tint")?),
        token_declaration("border", tokens.fluent.border.get("field")?),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
        token_declaration("padding", tokens.fluent.space.get("field-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("field")?),
        declaration("backdrop-filter", "blur(14px) saturate(1.05)"),
    ]);
    append_search_field_adornment(&mut declarations);
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn breadcrumb(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("flex-wrap", "wrap"),
        declaration("gap", "0.5rem"),
        token_declaration("color", tokens.fluent.color.get("muted")?),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn breadcrumb_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        token_declaration("color", tokens.fluent.color.get("muted")?),
    ]);
    Some(declarations)
}
fn avatar(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "inline-grid"),
        declaration("place-items", "center"),
        declaration("inline-size", "2.5rem"),
        declaration("block-size", "2.5rem"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        declaration("border-radius", "999px"),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn persona(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("gap", "0.75rem"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
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
fn sheet(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = dialog(tokens)?;
    declarations.retain(|decl| decl.property != "max-width");
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
