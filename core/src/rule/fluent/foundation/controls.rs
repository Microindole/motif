use super::super::super::shared::{append_inline_action_layout, append_transition};
use super::super::super::{declaration, token_declaration, Declaration};
use super::super::typography_from_tokens;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("field", None) => field(tokens),
        ("checkbox", None) => checkbox(tokens),
        ("radio", None) => radio(tokens),
        ("switch", None) => switch(tokens),
        ("textarea", None) => textarea(tokens),
        ("select", None) => select(tokens),
        ("tab", None) => tab(tokens),
        ("action", Some("primary")) => action_primary(tokens),
        ("action", Some("subtle")) => action_subtle(tokens),
        _ => None,
    }
}

pub(crate) fn field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
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
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn checkbox(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("appearance", "none"),
        declaration("display", "inline-grid"),
        declaration("place-items", "center"),
        declaration("inline-size", "1.125rem"),
        declaration("block-size", "1.125rem"),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("field")?),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("field")?),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn radio(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = checkbox(tokens)?;
    declarations.retain(|decl| decl.property != "border-radius");
    declarations.push(declaration("border-radius", "999px"));
    Some(declarations)
}

fn switch(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("appearance", "none"),
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("inline-size", "2.5rem"),
        declaration("block-size", "1.5rem"),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("field")?),
        declaration("border-radius", "999px"),
        token_declaration("box-shadow", tokens.fluent.shadow.get("field")?),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn textarea(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = field(tokens)?;
    declarations.push(declaration("min-height", "6.5rem"));
    declarations.push(declaration("resize", "vertical"));
    Some(declarations)
}

fn select(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = field(tokens)?;
    declarations.push(declaration("appearance", "none"));
    declarations.push(declaration("background-repeat", "no-repeat"));
    declarations.push(declaration("background-position", "right 0.85rem center"));
    Some(declarations)
}

fn tab(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "center"),
        declaration("min-height", "2.5rem"),
        token_declaration("color", tokens.fluent.color.get("muted")?),
        declaration("background-color", "transparent"),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("action-pad")?),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn action_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    append_inline_action_layout(&mut declarations);
    declarations.extend([
        token_declaration("color", tokens.fluent.color.get("action-foreground")?),
        token_declaration("background-color", tokens.fluent.color.get("primary")?),
        token_declaration("border", tokens.fluent.border.get("action-primary")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("action-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("action")?),
    ]);
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn action_subtle(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    append_inline_action_layout(&mut declarations);
    declarations.extend([
        token_declaration("color", tokens.fluent.color.get("primary")?),
        token_declaration(
            "background-color",
            tokens.fluent.color.get("action-subtle")?,
        ),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("action-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("action-subtle")?),
        declaration("backdrop-filter", "blur(18px) saturate(1.08)"),
    ]);
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}
