mod fluent;
mod material;

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
    let declarations = match parsed.family {
        Family::Fluent => fluent::resolve(parsed, tokens),
        Family::Material => material::resolve(parsed, tokens),
    }?;

    Some(RuleMatch {
        raw_class_name: parsed.raw.clone(),
        family: parsed.family,
        variants: parsed.variants.clone(),
        utility: parsed.utility.clone(),
        value: parsed.value.clone(),
        declarations,
    })
}

pub(super) fn declaration(property: &'static str, value: &'static str) -> Declaration {
    Declaration {
        property,
        value: value.to_string(),
    }
}

pub(super) fn token_declaration(property: &'static str, value: &str) -> Declaration {
    Declaration {
        property,
        value: value.to_string(),
    }
}
