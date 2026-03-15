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
        ("action", Some("tonal")) => action_tonal(tokens),
        ("action", Some("outlined")) => action_outlined(tokens),
        _ => None,
    }
}

pub(crate) fn field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
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

fn checkbox(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("appearance", "none"),
        declaration("display", "inline-grid"),
        declaration("place-items", "center"),
        declaration("inline-size", "1.125rem"),
        declaration("block-size", "1.125rem"),
        token_declaration("background-color", tokens.material.color.get("surface")?),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("md")?),
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

fn radio(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = checkbox(tokens)?;
    declarations.retain(|decl| decl.property != "border-radius");
    declarations.push(token_declaration(
        "border-radius",
        tokens.material.radius.get("pill")?,
    ));
    Some(declarations)
}

fn switch(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("appearance", "none"),
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("inline-size", "2.5rem"),
        declaration("block-size", "1.5rem"),
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
        token_declaration("color", tokens.material.color.get("primary")?),
        declaration("background-color", "transparent"),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration("padding", tokens.material.space.get("action-pad")?),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
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
