use super::super::shared::{
    accordion_content_base, accordion_header_base, accordion_item_base, append_transition,
    table_container, table_data_row, table_header_row,
};
use super::super::{declaration, token_declaration, Declaration};
use super::typography_from_tokens;
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
        ("table", None) => table(tokens),
        ("table", Some("header")) => table_header(tokens),
        ("table", Some("cell")) => table_cell(tokens),
        ("accordion", Some("item")) => accordion_item(tokens),
        ("accordion", Some("header")) => accordion_header(tokens),
        ("accordion", Some("content")) => accordion_content(tokens),
        ("accordion", Some("item-open")) => accordion_item_open(tokens),
        ("table", Some("row")) => table_row(tokens),
        ("table", Some("row-selected")) => table_row_selected(tokens),
        _ => None,
    }
}

fn badge(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("min-height", "1.75rem"),
        token_declaration("color", tokens.fluent.color.get("primary")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        declaration("border-radius", "999px"),
        token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
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
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        declaration("border-radius", "999px"),
        token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn tag(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("min-height", "1.5rem"),
        token_declaration("color", tokens.fluent.color.get("primary")?),
        declaration("background-color", "transparent"),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
        declaration("padding", "0.2rem 0.55rem"),
    ]);
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn progress(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "block"),
        declaration("inline-size", "100%"),
        declaration("block-size", "0.5rem"),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        declaration("border-radius", "999px"),
        token_declaration(
            "background-image",
            tokens.fluent.effect.get("surface-alt-tint")?,
        ),
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
fn spinner(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "inline-block"),
        declaration("inline-size", "1.25rem"),
        declaration("block-size", "1.25rem"),
        declaration("border-width", "2px"),
        declaration("border-style", "solid"),
        token_declaration("border-color", tokens.fluent.color.get("border")?),
        token_declaration("border-top-color", tokens.fluent.color.get("primary")?),
        declaration("border-radius", "999px"),
        declaration("animation", "motif-spin 0.9s linear infinite"),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn skeleton(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![declaration("display", "block"), declaration("min-height", "1rem"), token_declaration("background-color", tokens.fluent.color.get("surface-alt")?), declaration("background-image", "linear-gradient(90deg, rgba(255, 255, 255, 0.06), rgba(255, 255, 255, 0.4), rgba(255, 255, 255, 0.08))"), declaration("background-size", "200% 100%"), token_declaration("border-radius", tokens.fluent.radius.get("sm")?), declaration("animation", "motif-shimmer 1.4s ease-in-out infinite")];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn empty_state(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "grid"),
        declaration("justify-items", "start"),
        declaration("gap", "0.75rem"),
        token_declaration("color", tokens.fluent.color.get("muted")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("lg")?),
        token_declaration("padding", tokens.fluent.space.get("panel-pad")?),
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

fn table(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = table_container(
        tokens.fluent.color.get("text")?,
        tokens.fluent.color.get("surface-alt")?,
        tokens.fluent.border.get("action-subtle")?,
        tokens.fluent.radius.get("md")?,
        tokens.fluent.space.get("surface-pad-sm")?,
    );
    declarations.push(declaration("width", "100%"));
    declarations.push(declaration("align-self", "stretch"));
    declarations.push(token_declaration(
        "background-image",
        tokens.fluent.effect.get("surface-alt-tint")?,
    ));
    declarations.push(declaration("background-blend-mode", "screen"));
    declarations.push(token_declaration(
        "box-shadow",
        tokens.fluent.shadow.get("surface-alt")?,
    ));
    declarations.push(declaration("overflow", "hidden"));
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn table_header(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend(table_header_row(
        tokens.fluent.color.get("muted")?,
        tokens.fluent.space.get("surface-pad-sm")?,
    ));
    Some(declarations)
}
fn table_cell(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography_from_tokens(
        tokens,
        "body-size",
        "body-weight",
        Some(tokens.fluent.color.get("text")?),
    )
}
fn accordion_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = accordion_item_base(
        tokens.fluent.color.get("text")?,
        tokens.fluent.color.get("surface-alt")?,
        tokens.fluent.border.get("action-subtle")?,
        tokens.fluent.radius.get("md")?,
        tokens.fluent.space.get("surface-pad-sm")?,
    );
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn accordion_header(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend(accordion_header_base(tokens.fluent.color.get("text")?));
    Some(declarations)
}
fn accordion_content(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "body-size", "body-weight", None)?;
    declarations.extend(accordion_content_base(tokens.fluent.color.get("muted")?));
    Some(declarations)
}
fn accordion_item_open(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = accordion_item(tokens)?;
    declarations.push(token_declaration(
        "background-image",
        tokens.fluent.effect.get("surface-alt-tint")?,
    ));
    declarations.push(token_declaration(
        "box-shadow",
        tokens.fluent.shadow.get("surface-alt")?,
    ));
    Some(declarations)
}
fn table_row(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = table_data_row(
        tokens.fluent.color.get("text")?,
        tokens.fluent.color.get("surface-alt")?,
        tokens.fluent.border.get("action-subtle")?,
        tokens.fluent.radius.get("sm")?,
        tokens.fluent.space.get("surface-pad-sm")?,
    );
    declarations.push(declaration("width", "100%"));
    declarations.push(declaration("align-self", "stretch"));
    declarations.push(declaration("overflow", "hidden"));
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
fn table_row_selected(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = table_row(tokens)?;
    declarations.push(token_declaration(
        "background-image",
        tokens.fluent.effect.get("surface-alt-tint")?,
    ));
    declarations.push(token_declaration(
        "box-shadow",
        tokens.fluent.shadow.get("surface-alt")?,
    ));
    declarations.push(token_declaration(
        "border-color",
        tokens.fluent.color.get("border-strong")?,
    ));
    Some(declarations)
}
