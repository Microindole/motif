use crate::parse::{Family, ParsedClass, Variant};
use crate::token::TokenRegistry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleMatch {
    pub raw_class_name: String,
    pub family: Family,
    pub variants: Vec<Variant>,
    pub utility: String,
    pub value: Option<String>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    pub property: &'static str,
    pub value: String,
}

impl RuleMatch {
    pub fn describe(&self) -> String {
        let declarations = self
            .declarations
            .iter()
            .map(|declaration| format!("{}={}", declaration.property, declaration.value))
            .collect::<Vec<_>>()
            .join(", ");

        if self.variants.is_empty() {
            declarations
        } else {
            format!("variants={:?}; {declarations}", self.variants)
        }
    }
}

pub fn resolve_rule(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<RuleMatch> {
    let declarations = match (parsed.family, parsed.utility.as_str(), parsed.value.as_deref()) {
        (Family::Fluent, "stack", None) => vec![
            declaration("display", "flex"),
            declaration("flex-direction", "column"),
            token_declaration("gap", tokens.fluent.space.get("md")?),
        ],
        (Family::Fluent, "ring", None) => vec![
            token_declaration("outline-width", tokens.fluent.outline.get("focus-width")?),
            declaration("outline-style", "solid"),
            token_declaration("outline-color", tokens.fluent.color.get("primary")?),
            token_declaration("outline-offset", tokens.fluent.space.get("focus-offset")?),
            token_declaration("transition-duration", tokens.fluent.motion.get("duration")?),
            token_declaration("transition-timing-function", tokens.fluent.motion.get("easing")?),
        ],
        (Family::Fluent, "bg", Some("primary")) => vec![
            token_declaration("background-color", tokens.fluent.color.get("primary")?),
            declaration("color", "#ffffff"),
        ],
        (Family::Fluent, "text", Some("primary")) => {
            vec![token_declaration("color", tokens.fluent.color.get("primary")?)]
        }
        (Family::Fluent, "text", Some("muted")) => {
            vec![token_declaration("color", tokens.fluent.color.get("muted")?)]
        }
        (Family::Fluent, "surface", None) => vec![
            token_declaration("background-color", tokens.fluent.color.get("surface")?),
            token_declaration("color", tokens.fluent.color.get("text")?),
            token_declaration("border-radius", tokens.fluent.radius.get("md")?),
            token_declaration("padding", tokens.fluent.space.get("surface-pad")?),
            token_declaration("border-width", tokens.fluent.border.get("subtle-width")?),
            declaration("border-style", "solid"),
            token_declaration("border-color", tokens.fluent.color.get("border")?),
            token_declaration("box-shadow", tokens.fluent.shadow.get("surface")?),
            declaration("backdrop-filter", "blur(18px) saturate(1.15)"),
        ],
        (Family::Fluent, "title", None) => vec![
            token_declaration("font-family", tokens.fluent.typography.get("font-family")?),
            token_declaration("font-size", tokens.fluent.typography.get("title-size")?),
            token_declaration("font-weight", tokens.fluent.typography.get("title-weight")?),
            token_declaration("line-height", tokens.fluent.typography.get("line-height")?),
            token_declaration("letter-spacing", tokens.fluent.typography.get("tracking")?),
        ],
        (Family::Fluent, "body", None) => vec![
            token_declaration("font-family", tokens.fluent.typography.get("font-family")?),
            token_declaration("font-size", tokens.fluent.typography.get("body-size")?),
            token_declaration("font-weight", tokens.fluent.typography.get("body-weight")?),
            token_declaration("line-height", tokens.fluent.typography.get("line-height")?),
            token_declaration("letter-spacing", tokens.fluent.typography.get("tracking")?),
        ],
        (Family::Material, "surface", None) => vec![
            token_declaration("background-color", tokens.material.color.get("surface")?),
            token_declaration("color", tokens.material.color.get("on-surface")?),
            token_declaration("border-radius", tokens.material.radius.get("lg")?),
            declaration("padding", "1.25rem"),
            token_declaration("box-shadow", tokens.material.shadow.get("surface")?),
        ],
        (Family::Material, "bg", Some("primary")) => vec![
            token_declaration("background-color", tokens.material.color.get("primary")?),
            token_declaration("color", tokens.material.color.get("on-primary")?),
            token_declaration("border-radius", tokens.material.radius.get("pill")?),
            token_declaration("transition-duration", tokens.material.motion.get("duration")?),
            token_declaration("transition-timing-function", tokens.material.motion.get("easing")?),
        ],
        (Family::Material, "text", Some("primary")) => {
            vec![token_declaration("color", tokens.material.color.get("primary")?)]
        }
        (Family::Material, "text", Some("on-primary")) => {
            vec![token_declaration("color", tokens.material.color.get("on-primary")?)]
        }
        (Family::Material, "text", Some("muted")) => {
            vec![token_declaration("color", tokens.material.color.get("muted")?)]
        }
        (Family::Material, "title", None) => vec![
            token_declaration("font-family", tokens.material.typography.get("font-family")?),
            token_declaration("font-size", tokens.material.typography.get("title-size")?),
            token_declaration("font-weight", tokens.material.typography.get("title-weight")?),
            token_declaration("line-height", tokens.material.typography.get("line-height")?),
            token_declaration("letter-spacing", tokens.material.typography.get("tracking")?),
        ],
        (Family::Material, "body", None) => vec![
            token_declaration("font-family", tokens.material.typography.get("font-family")?),
            token_declaration("font-size", tokens.material.typography.get("body-size")?),
            token_declaration("font-weight", tokens.material.typography.get("body-weight")?),
            token_declaration("line-height", tokens.material.typography.get("line-height")?),
            token_declaration("letter-spacing", tokens.material.typography.get("tracking")?),
        ],
        (Family::Material, "elevation", Some("1")) => {
            vec![token_declaration("box-shadow", tokens.material.shadow.get("1")?)]
        }
        (Family::Material, "shadow", Some("2")) => {
            vec![token_declaration("box-shadow", tokens.material.shadow.get("2")?)]
        }
        _ => return None,
    };

    Some(RuleMatch {
        raw_class_name: parsed.raw.clone(),
        family: parsed.family,
        variants: parsed.variants.clone(),
        utility: parsed.utility.clone(),
        value: parsed.value.clone(),
        declarations,
    })
}

fn declaration(property: &'static str, value: &'static str) -> Declaration {
    Declaration {
        property,
        value: value.to_string(),
    }
}

fn token_declaration(property: &'static str, value: &str) -> Declaration {
    Declaration {
        property,
        value: value.to_string(),
    }
}
