use super::super::shared::{append_inline_action_layout, append_transition};
use super::super::{declaration, token_declaration, Declaration};
use super::typography_from_tokens;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("stack", None) => stack(tokens),
        ("ring", None) => ring(tokens),
        ("bg", Some("primary")) => bg_primary(tokens),
        ("bg", Some("hover-primary")) => bg_hover_primary(tokens),
        ("bg", Some("hover-subtle")) => bg_hover_subtle(tokens),
        ("text", Some("primary")) => text_primary(tokens),
        ("text", Some("muted")) => text_muted(tokens),
        ("surface", None) => surface(tokens),
        ("surface", Some("alt")) => surface_alt(tokens),
        ("panel", None) => panel(tokens),
        ("shadow", Some("press")) => shadow_press(tokens),
        ("title", None) => title(tokens),
        ("body", None) => body(tokens),
        ("label", None) => label(tokens),
        ("border", Some("focus")) => border_focus(tokens),
        ("divider", None) => divider(tokens),
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

fn stack(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        declaration("display", "flex"),
        declaration("flex-direction", "column"),
        token_declaration("gap", tokens.fluent.space.get("md")?),
    ])
}

fn ring(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("outline-width", tokens.fluent.outline.get("focus-width")?),
        declaration("outline-style", "solid"),
        token_declaration("outline-color", tokens.fluent.color.get("primary")?),
        token_declaration("outline-offset", tokens.fluent.space.get("focus-offset")?),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn bg_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration("background-color", tokens.fluent.color.get("primary")?),
        token_declaration("color", tokens.fluent.color.get("action-foreground")?),
    ])
}

fn bg_hover_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        token_declaration(
            "background-color",
            tokens.fluent.color.get("hover-primary")?,
        ),
        token_declaration("color", tokens.fluent.color.get("action-foreground")?),
    ])
}

fn bg_hover_subtle(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "background-color",
        tokens.fluent.color.get("hover-subtle")?,
    )])
}

fn text_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "color",
        tokens.fluent.color.get("primary")?,
    )])
}

fn text_muted(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "color",
        tokens.fluent.color.get("muted")?,
    )])
}

fn surface(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("background-color", tokens.fluent.color.get("surface")?),
        token_declaration(
            "background-image",
            tokens.fluent.effect.get("surface-tint")?,
        ),
        declaration("background-blend-mode", "screen"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("border-radius", tokens.fluent.radius.get("md")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad")?),
        token_declaration("border-width", tokens.fluent.border.get("subtle-width")?),
        declaration("border-style", "solid"),
        token_declaration("border-color", tokens.fluent.color.get("border")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("surface")?),
        declaration("backdrop-filter", "blur(18px) saturate(1.15)"),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn surface_alt(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration(
            "background-image",
            tokens.fluent.effect.get("surface-alt-tint")?,
        ),
        declaration("background-blend-mode", "screen"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
        token_declaration("padding", tokens.fluent.space.get("surface-pad-sm")?),
        token_declaration("border-width", tokens.fluent.border.get("subtle-width")?),
        declaration("border-style", "solid"),
        token_declaration("border-color", tokens.fluent.color.get("border-strong")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("surface-alt")?),
        declaration("backdrop-filter", "blur(28px) saturate(1.25)"),
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn panel(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
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
    ];
    append_transition(
        &mut declarations,
        tokens.fluent.effect.get("interactive-transition")?,
        tokens.fluent.motion.get("duration")?,
        tokens.fluent.motion.get("easing")?,
    );
    Some(declarations)
}

fn shadow_press(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "box-shadow",
        tokens.fluent.shadow.get("press")?,
    )])
}

fn title(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography_from_tokens(
        tokens,
        "title-size",
        "title-weight",
        Some(tokens.fluent.color.get("text")?),
    )
}

fn body(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography_from_tokens(
        tokens,
        "body-size",
        "body-weight",
        Some(tokens.fluent.color.get("text")?),
    )
}

fn label(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    typography_from_tokens(
        tokens,
        "label-size",
        "label-weight",
        Some(tokens.fluent.color.get("muted")?),
    )
}

fn border_focus(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "border",
        tokens.fluent.color.get("border-focus")?,
    )])
}

fn divider(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        declaration("display", "block"),
        token_declaration("border-bottom", tokens.fluent.border.get("divider")?),
    ])
}

pub(super) fn field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
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
