use super::shared::{append_inline_action_layout, append_transition, typography};
use super::{declaration, token_declaration, Declaration};
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
        ("dialog", None) => dialog(tokens),
        ("list", Some("item")) => list_item(tokens),
        ("menu", Some("item")) => menu_item(tokens),
        ("icon", Some("button")) => icon_button(tokens),
        ("nav", Some("item")) => nav_item(tokens),
        ("badge", None) => badge(tokens),
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
        ("progress", None) => progress(tokens),
        ("spinner", None) => spinner(tokens),
        ("skeleton", None) => skeleton(tokens),
        ("empty", Some("state")) => empty_state(tokens),
        ("sheet", None) => sheet(tokens),
        ("accordion", Some("item")) => accordion_item(tokens),
        ("accordion", Some("item-open")) => accordion_item_open(tokens),
        ("table", Some("row")) => table_row(tokens),
        ("table", Some("row-selected")) => table_row_selected(tokens),
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

fn field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
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

fn dialog(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = panel(tokens)?;
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
        declaration("min-height", "2.5rem"),
        token_declaration("color", tokens.fluent.color.get("muted")?),
        declaration("background-color", "transparent"),
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

fn drawer(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "flex"),
        declaration("flex-direction", "column"),
        declaration("min-width", "16rem"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface")?),
        token_declaration(
            "background-image",
            tokens.fluent.effect.get("surface-tint")?,
        ),
        token_declaration("border", tokens.fluent.border.get("panel")?),
        token_declaration("border-radius", tokens.fluent.radius.get("lg")?),
        token_declaration("padding", tokens.fluent.space.get("panel-pad")?),
        token_declaration("box-shadow", tokens.fluent.shadow.get("panel")?),
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
    let mut declarations = vec![
        declaration("display", "block"),
        declaration("min-height", "1rem"),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        declaration(
            "background-image",
            "linear-gradient(90deg, rgba(255, 255, 255, 0.06), rgba(255, 255, 255, 0.4), rgba(255, 255, 255, 0.08))",
        ),
        declaration("background-size", "200% 100%"),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
        declaration("animation", "motif-shimmer 1.4s ease-in-out infinite"),
    ];
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

fn sheet(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = panel(tokens)?;
    declarations.push(declaration("max-width", "28rem"));
    declarations.push(declaration("width", "100%"));
    declarations.push(declaration("margin-inline", "auto"));
    Some(declarations)
}

fn accordion_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "grid"),
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
    let mut declarations = vec![
        declaration("display", "grid"),
        declaration(
            "grid-template-columns",
            "minmax(0, 2fr) minmax(0, 1fr) auto",
        ),
        declaration("align-items", "center"),
        declaration("gap", "0.75rem"),
        declaration("min-height", "3rem"),
        token_declaration("color", tokens.fluent.color.get("text")?),
        token_declaration("background-color", tokens.fluent.color.get("surface-alt")?),
        token_declaration("border", tokens.fluent.border.get("action-subtle")?),
        token_declaration("border-radius", tokens.fluent.radius.get("sm")?),
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

fn table_row_selected(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = table_row(tokens)?;
    declarations.push(token_declaration(
        "background-image",
        tokens.fluent.effect.get("surface-alt-tint")?,
    ));
    declarations.push(token_declaration(
        "border-color",
        tokens.fluent.color.get("border-strong")?,
    ));
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

fn typography_from_tokens(
    tokens: &TokenRegistry,
    size_key: &str,
    weight_key: &str,
    color: Option<&str>,
) -> Option<Vec<Declaration>> {
    Some(typography(
        tokens.fluent.typography.get("font-family")?,
        tokens.fluent.typography.get(size_key)?,
        tokens.fluent.typography.get(weight_key)?,
        tokens.fluent.typography.get("line-height")?,
        tokens.fluent.typography.get("tracking")?,
        color,
    ))
}
