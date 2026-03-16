use super::super::super::shared::{
    accordion_content_base, accordion_header_base, accordion_item_base, append_transition,
    table_container, table_data_row, table_header_row,
};
use super::super::super::{declaration, token_declaration, Declaration};
use super::super::typography_from_tokens;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("table", None) => table(tokens),
        ("table", Some("header")) => table_header(tokens),
        ("table", Some("cell")) => table_cell(tokens),
        ("table", Some("row")) => table_row(tokens),
        ("table", Some("row-selected")) => table_row_selected(tokens),
        ("accordion", Some("item")) => accordion_item(tokens),
        ("accordion", Some("header")) => accordion_header(tokens),
        ("accordion", Some("content")) => accordion_content(tokens),
        ("accordion", Some("item-open")) => accordion_item_open(tokens),
        _ => None,
    }
}

fn table(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = table_container(
        tokens.material.color.get("on-surface")?,
        tokens.material.color.get("surface-container")?,
        tokens.material.border.get("surface-container")?,
        tokens.material.radius.get("lg")?,
        "0.85rem 1rem",
    );
    declarations.push(token_declaration(
        "background-image",
        tokens.material.effect.get("container-tint")?,
    ));
    declarations.push(token_declaration(
        "box-shadow",
        tokens.material.shadow.get("container")?,
    ));
    declarations.push(declaration("width", "100%"));
    declarations.push(declaration("align-self", "stretch"));
    declarations.push(declaration("overflow", "hidden"));
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn table_header(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend(table_header_row(
        tokens.material.color.get("muted")?,
        "0.85rem 1rem",
    ));
    Some(declarations)
}
fn table_cell(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography_from_tokens(
        tokens,
        "body-size",
        "body-weight",
        Some(tokens.material.color.get("on-surface")?),
    )
}
fn table_row(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = table_data_row(
        tokens.material.color.get("on-surface")?,
        tokens.material.color.get("surface-container")?,
        tokens.material.border.get("surface-container")?,
        tokens.material.radius.get("md")?,
        "0.85rem 1rem",
    );
    declarations.push(declaration("width", "100%"));
    declarations.push(declaration("align-self", "stretch"));
    declarations.push(declaration("overflow", "hidden"));
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn table_row_selected(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = table_row(tokens)?;
    declarations.push(token_declaration(
        "background-image",
        tokens.material.effect.get("container-tint")?,
    ));
    declarations.push(token_declaration(
        "box-shadow",
        tokens.material.shadow.get("container")?,
    ));
    declarations.push(token_declaration(
        "border-color",
        tokens.material.color.get("primary")?,
    ));
    Some(declarations)
}
fn accordion_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = accordion_item_base(
        tokens.material.color.get("on-surface")?,
        tokens.material.color.get("surface-container")?,
        tokens.material.border.get("surface-container")?,
        tokens.material.radius.get("lg")?,
        "1rem",
    );
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn accordion_header(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend(accordion_header_base(
        tokens.material.color.get("on-surface")?,
    ));
    Some(declarations)
}
fn accordion_content(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "body-size", "body-weight", None)?;
    declarations.extend(accordion_content_base(tokens.material.color.get("muted")?));
    Some(declarations)
}
fn accordion_item_open(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = accordion_item(tokens)?;
    declarations.push(token_declaration(
        "background-image",
        tokens.material.effect.get("container-tint")?,
    ));
    declarations.push(token_declaration(
        "box-shadow",
        tokens.material.shadow.get("container")?,
    ));
    Some(declarations)
}
