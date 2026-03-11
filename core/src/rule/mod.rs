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
        ],
        (Family::Fluent, "bg", Some("primary")) => {
            vec![token_declaration("background-color", tokens.fluent.color.get("primary")?)]
        }
        (Family::Fluent, "text", Some("primary")) => {
            vec![token_declaration("color", tokens.fluent.color.get("primary")?)]
        }
        (Family::Material, "surface", None) => vec![
            token_declaration("background-color", tokens.material.color.get("surface")?),
            token_declaration("color", tokens.material.color.get("on-surface")?),
            token_declaration("border-radius", tokens.material.radius.get("lg")?),
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
