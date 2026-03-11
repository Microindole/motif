use crate::parse::{Family, ParsedClass, Variant};

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
    pub value: &'static str,
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

pub fn resolve_rule(parsed: &ParsedClass) -> Option<RuleMatch> {
    let declarations = match (parsed.family, parsed.utility.as_str(), parsed.value.as_deref()) {
        (Family::Fluent, "stack", None) => vec![
            declaration("display", "flex"),
            declaration("flex-direction", "column"),
            declaration("gap", "1rem"),
        ],
        (Family::Fluent, "ring", None) => vec![
            declaration("outline-width", "2px"),
            declaration("outline-style", "solid"),
            declaration("outline-color", "#0f6cbd"),
            declaration("outline-offset", "2px"),
        ],
        (Family::Fluent, "bg", Some("primary")) => {
            vec![declaration("background-color", "#0f6cbd")]
        }
        (Family::Fluent, "text", Some("primary")) => {
            vec![declaration("color", "#0f6cbd")]
        }
        (Family::Material, "surface", None) => vec![
            declaration("background-color", "#ffffff"),
            declaration("color", "#1f1f1f"),
            declaration("border-radius", "12px"),
        ],
        (Family::Material, "elevation", Some("1")) => {
            vec![declaration("box-shadow", "0 1px 2px rgba(0, 0, 0, 0.3)")]
        }
        (Family::Material, "shadow", Some("2")) => {
            vec![declaration("box-shadow", "0 2px 6px rgba(0, 0, 0, 0.24)")]
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
    Declaration { property, value }
}
