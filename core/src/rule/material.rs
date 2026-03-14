use super::shared::{append_inline_action_layout, append_transition, typography};
use super::{declaration, token_declaration, Declaration};
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    match (parsed.utility.as_str(), parsed.value.as_deref()) {
        ("surface", None) => surface(tokens),
        ("surface", Some("container")) => surface_container(tokens),
        ("surface", Some("variant")) => surface_variant(tokens),
        ("bg", Some("primary")) => bg_primary(tokens),
        ("bg", Some("hover-primary")) => bg_color(tokens, "hover-primary"),
        ("bg", Some("hover-container")) => bg_color(tokens, "hover-container"),
        ("bg", Some("hover-surface")) => bg_color(tokens, "hover-surface"),
        ("bg", Some("primary-container")) => bg_primary_container(tokens),
        ("text", Some("primary")) => text_color(tokens, "primary"),
        ("text", Some("on-primary")) => text_color(tokens, "on-primary"),
        ("text", Some("muted")) => text_color(tokens, "muted"),
        ("ring", None) => ring(tokens),
        ("title", None) => typography_from_tokens(
            tokens,
            "title-size",
            "title-weight",
            Some(tokens.material.color.get("on-surface")?),
        ),
        ("body", None) => typography_from_tokens(
            tokens,
            "body-size",
            "body-weight",
            Some(tokens.material.color.get("on-surface")?),
        ),
        ("label", None) => typography_from_tokens(
            tokens,
            "label-size",
            "label-weight",
            Some(tokens.material.color.get("muted")?),
        ),
        ("divider", None) => divider(tokens),
        ("border", Some("focus")) => border_focus(tokens),
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
        ("chip", None) => chip(tokens),
        ("tag", None) => tag(tokens),
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
        ("sheet", Some("side")) => sheet_side(tokens),
        ("sheet", Some("bottom")) => sheet_bottom(tokens),
        ("table", None) => table(tokens),
        ("table", Some("header")) => table_header(tokens),
        ("accordion", Some("item")) => accordion_item(tokens),
        ("accordion", Some("header")) => accordion_header(tokens),
        ("accordion", Some("item-open")) => accordion_item_open(tokens),
        ("table", Some("row")) => table_row(tokens),
        ("table", Some("row-selected")) => table_row_selected(tokens),
        ("action", Some("primary")) => action_primary(tokens),
        ("action", Some("tonal")) => action_tonal(tokens),
        ("action", Some("outlined")) => action_outlined(tokens),
        ("elevation", Some("1")) => shadow(tokens, "1"),
        ("shadow", Some("2")) => shadow(tokens, "2"),
        ("shadow", Some("press")) => shadow(tokens, "press"),
        _ => None,
    }
}
fn surface(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("background-color", tokens.material.color.get("surface")?),
        token_declaration(
            "background-image",
            tokens.material.effect.get("surface-tint")?,
        ),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1.25rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("surface")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn surface_container(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration(
            "background-image",
            tokens.material.effect.get("container-tint")?,
        ),
        token_declaration("color", tokens.material.color.get("on-surface")?),
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
fn surface_variant(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-variant")?,
        ),
        token_declaration(
            "background-image",
            tokens.material.effect.get("variant-tint")?,
        ),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1rem"),
        token_declaration("box-shadow", tokens.material.shadow.get("container")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn bg_primary(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration("background-color", tokens.material.color.get("primary")?),
        token_declaration("color", tokens.material.color.get("on-primary")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn bg_color(tokens: &TokenRegistry, key: &str) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "background-color",
        tokens.material.color.get(key)?,
    )])
}
fn bg_primary_container(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        token_declaration(
            "background-color",
            tokens.material.color.get("primary-container")?,
        ),
        token_declaration("color", tokens.material.color.get("on-primary-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
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
fn text_color(tokens: &TokenRegistry, key: &str) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "color",
        tokens.material.color.get(key)?,
    )])
}
fn ring(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("outline-width", "2px"),
        declaration("outline-style", "solid"),
        token_declaration("outline-color", tokens.material.color.get("primary")?),
        declaration("outline-offset", "2px"),
        declaration("box-shadow", "0 0 0 4px rgba(26, 115, 232, 0.16)"),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}
fn divider(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![
        declaration("display", "block"),
        token_declaration("border-bottom", tokens.material.border.get("divider")?),
    ])
}
fn border_focus(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "border",
        tokens.material.border.get("focus")?,
    )])
}
fn field(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
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

fn badge(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("min-height", "1.75rem"),
        token_declaration("color", tokens.material.color.get("on-primary-container")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("primary-container")?,
        ),
        declaration("border", "0"),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        declaration("padding", "0.35rem 0.75rem"),
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
        token_declaration("color", tokens.material.color.get("primary")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        declaration("padding", "0.55rem 0.85rem"),
    ]);
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn tag(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "inline-flex"),
        declaration("align-items", "center"),
        declaration("min-height", "1.5rem"),
        token_declaration("color", tokens.material.color.get("primary")?),
        declaration("background-color", "transparent"),
        token_declaration("border", tokens.material.border.get("outlined-action")?),
        token_declaration("border-radius", tokens.material.radius.get("md")?),
        declaration("padding", "0.2rem 0.55rem"),
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

fn progress(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "block"),
        declaration("inline-size", "100%"),
        declaration("block-size", "0.5rem"),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        token_declaration(
            "background-image",
            tokens.material.effect.get("container-tint")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
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
        token_declaration(
            "border-color",
            tokens.material.color.get("surface-variant")?,
        ),
        token_declaration("border-top-color", tokens.material.color.get("primary")?),
        token_declaration("border-radius", tokens.material.radius.get("pill")?),
        declaration("animation", "motif-spin 0.9s linear infinite"),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("state-transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn skeleton(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "block"),
        declaration("min-height", "1rem"),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        declaration(
            "background-image",
            "linear-gradient(90deg, rgba(255, 255, 255, 0.12), rgba(255, 255, 255, 0.56), rgba(211, 227, 253, 0.2))",
        ),
        declaration("background-size", "200% 100%"),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("animation", "motif-shimmer 1.4s ease-in-out infinite"),
    ];
    append_transition(
        &mut declarations,
        tokens.material.effect.get("transition")?,
        tokens.material.motion.get("duration")?,
        tokens.material.motion.get("easing")?,
    );
    Some(declarations)
}

fn empty_state(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "grid"),
        declaration("justify-items", "start"),
        declaration("gap", "0.75rem"),
        token_declaration("color", tokens.material.color.get("muted")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1rem"),
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

fn table(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "grid"),
        declaration("gap", "0.5rem"),
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

fn table_header(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = typography_from_tokens(tokens, "label-size", "label-weight", None)?;
    declarations.extend([
        declaration("display", "grid"),
        declaration(
            "grid-template-columns",
            "minmax(0, 2fr) minmax(0, 1fr) auto",
        ),
        declaration("align-items", "center"),
        declaration("gap", "0.75rem"),
        token_declaration("color", tokens.material.color.get("muted")?),
        declaration("padding", "0.85rem 1rem"),
    ]);
    Some(declarations)
}

fn accordion_item(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = vec![
        declaration("display", "grid"),
        declaration("gap", "0.75rem"),
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("lg")?),
        declaration("padding", "1rem"),
    ];
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
    declarations.extend([
        declaration("display", "flex"),
        declaration("align-items", "center"),
        declaration("justify-content", "space-between"),
        token_declaration("color", tokens.material.color.get("on-surface")?),
    ]);
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
        token_declaration("color", tokens.material.color.get("on-surface")?),
        token_declaration(
            "background-color",
            tokens.material.color.get("surface-container")?,
        ),
        token_declaration("border", tokens.material.border.get("surface-container")?),
        token_declaration("border-radius", tokens.material.radius.get("md")?),
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

fn table_row_selected(tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    let mut declarations = table_row(tokens)?;
    declarations.push(token_declaration(
        "background-image",
        tokens.material.effect.get("container-tint")?,
    ));
    declarations.push(token_declaration(
        "border-color",
        tokens.material.color.get("primary")?,
    ));
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
fn shadow(tokens: &TokenRegistry, key: &str) -> Option<Vec<Declaration>> {
    Some(vec![token_declaration(
        "box-shadow",
        tokens.material.shadow.get(key)?,
    )])
}
fn typography_from_tokens(
    tokens: &TokenRegistry,
    size_key: &str,
    weight_key: &str,
    color: Option<&str>,
) -> Option<Vec<Declaration>> {
    Some(typography(
        tokens.material.typography.get("font-family")?,
        tokens.material.typography.get(size_key)?,
        tokens.material.typography.get(weight_key)?,
        tokens.material.typography.get("line-height")?,
        tokens.material.typography.get("tracking")?,
        color,
    ))
}
