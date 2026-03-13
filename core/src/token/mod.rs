use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;

const FLUENT_COLOR_KEYS: &[&str] = &[
    "primary",
    "surface",
    "surface-alt",
    "panel",
    "text",
    "muted",
    "border",
    "border-strong",
    "hover-primary",
    "hover-subtle",
    "border-focus",
    "field",
    "action-foreground",
    "action-subtle",
];
const FLUENT_SPACE_KEYS: &[&str] = &[
    "md",
    "focus-offset",
    "surface-pad",
    "surface-pad-sm",
    "panel-pad",
    "field-pad",
    "action-pad",
];
const FLUENT_TYPOGRAPHY_KEYS: &[&str] = &[
    "font-family",
    "title-size",
    "title-weight",
    "body-size",
    "body-weight",
    "label-size",
    "label-weight",
    "line-height",
    "tracking",
];
const FLUENT_BORDER_KEYS: &[&str] = &[
    "subtle-width",
    "divider",
    "field",
    "panel",
    "action-subtle",
    "action-primary",
];
const FLUENT_SHADOW_KEYS: &[&str] = &[
    "surface",
    "surface-alt",
    "panel",
    "field",
    "action",
    "action-subtle",
    "press",
];
const FLUENT_EFFECT_KEYS: &[&str] = &[
    "surface-tint",
    "surface-alt-tint",
    "panel-tint",
    "field-tint",
    "transition",
    "interactive-transition",
];
const MATERIAL_COLOR_KEYS: &[&str] = &[
    "primary",
    "on-primary",
    "primary-container",
    "hover-primary",
    "hover-container",
    "hover-surface",
    "on-primary-container",
    "surface",
    "surface-container",
    "surface-variant",
    "on-surface",
    "muted",
    "field",
];
const MATERIAL_BORDER_KEYS: &[&str] = &[
    "divider",
    "field",
    "focus",
    "outlined-action",
    "surface-container",
];
const MATERIAL_SHADOW_KEYS: &[&str] = &[
    "1",
    "2",
    "surface",
    "container",
    "container-high",
    "field",
    "action",
    "outlined-action",
    "tonal-action",
    "press",
];
const MATERIAL_EFFECT_KEYS: &[&str] = &[
    "surface-tint",
    "container-tint",
    "variant-tint",
    "field-tint",
    "transition",
    "state-transition",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenRegistry {
    pub fluent: FluentTokens,
    pub material: MaterialTokens,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FluentTokens {
    pub color: BTreeMap<String, String>,
    pub space: BTreeMap<String, String>,
    pub outline: BTreeMap<String, String>,
    pub radius: BTreeMap<String, String>,
    pub typography: BTreeMap<String, String>,
    pub border: BTreeMap<String, String>,
    pub shadow: BTreeMap<String, String>,
    pub motion: BTreeMap<String, String>,
    pub effect: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialTokens {
    pub color: BTreeMap<String, String>,
    pub space: BTreeMap<String, String>,
    pub border: BTreeMap<String, String>,
    pub radius: BTreeMap<String, String>,
    pub shadow: BTreeMap<String, String>,
    pub typography: BTreeMap<String, String>,
    pub motion: BTreeMap<String, String>,
    pub effect: BTreeMap<String, String>,
}

#[derive(Debug)]
pub enum TokenError {
    Parse {
        source: serde_json::Error,
    },
    MissingKey {
        preset: &'static str,
        group: &'static str,
        key: &'static str,
    },
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { source } => write!(f, "failed to parse embedded tokens: {source}"),
            Self::MissingKey { preset, group, key } => {
                write!(f, "missing token key {preset}.{group}.{key}")
            }
        }
    }
}

impl std::error::Error for TokenError {}

pub fn load_registry() -> Result<TokenRegistry, TokenError> {
    parse_registry(
        include_str!("../../../tokens/fluent.json"),
        include_str!("../../../tokens/material.json"),
    )
}

pub fn parse_registry(fluent_json: &str, material_json: &str) -> Result<TokenRegistry, TokenError> {
    let fluent = serde_json::from_str::<FluentTokens>(fluent_json)
        .map_err(|source| TokenError::Parse { source })?;
    let material = serde_json::from_str::<MaterialTokens>(material_json)
        .map_err(|source| TokenError::Parse { source })?;
    validate_fluent(&fluent)?;
    validate_material(&material)?;
    Ok(TokenRegistry { fluent, material })
}

fn validate_fluent(tokens: &FluentTokens) -> Result<(), TokenError> {
    validate_groups(
        "fluent",
        &[
            ("color", &tokens.color, FLUENT_COLOR_KEYS),
            ("space", &tokens.space, FLUENT_SPACE_KEYS),
            ("outline", &tokens.outline, &["focus-width"]),
            ("radius", &tokens.radius, &["sm", "md", "lg"]),
            ("typography", &tokens.typography, FLUENT_TYPOGRAPHY_KEYS),
            ("border", &tokens.border, FLUENT_BORDER_KEYS),
            ("shadow", &tokens.shadow, FLUENT_SHADOW_KEYS),
            ("motion", &tokens.motion, &["duration", "easing"]),
            ("effect", &tokens.effect, FLUENT_EFFECT_KEYS),
        ],
    )
}

fn validate_material(tokens: &MaterialTokens) -> Result<(), TokenError> {
    validate_groups(
        "material",
        &[
            ("color", &tokens.color, MATERIAL_COLOR_KEYS),
            ("space", &tokens.space, &["field-pad", "action-pad"]),
            ("border", &tokens.border, MATERIAL_BORDER_KEYS),
            ("radius", &tokens.radius, &["md", "lg", "pill"]),
            ("shadow", &tokens.shadow, MATERIAL_SHADOW_KEYS),
            ("typography", &tokens.typography, FLUENT_TYPOGRAPHY_KEYS),
            ("motion", &tokens.motion, &["duration", "easing"]),
            ("effect", &tokens.effect, MATERIAL_EFFECT_KEYS),
        ],
    )
}

fn validate_groups(
    preset: &'static str,
    groups: &[(&'static str, &BTreeMap<String, String>, &[&'static str])],
) -> Result<(), TokenError> {
    for (group, values, keys) in groups {
        require_keys(preset, group, values, keys)?;
    }
    Ok(())
}

fn require_keys(
    preset: &'static str,
    group: &'static str,
    values: &BTreeMap<String, String>,
    keys: &[&'static str],
) -> Result<(), TokenError> {
    for key in keys {
        if !values.contains_key(*key) {
            return Err(TokenError::MissingKey { preset, group, key });
        }
    }
    Ok(())
}
