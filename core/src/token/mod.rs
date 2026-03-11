use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenRegistry {
    pub fluent: FluentTokens,
    pub material: MaterialTokens,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct FluentTokens {
    pub color: BTreeMap<String, String>,
    pub space: BTreeMap<String, String>,
    pub outline: BTreeMap<String, String>,
    pub radius: BTreeMap<String, String>,
    pub typography: BTreeMap<String, String>,
    pub border: BTreeMap<String, String>,
    pub shadow: BTreeMap<String, String>,
    pub motion: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct MaterialTokens {
    pub color: BTreeMap<String, String>,
    pub space: BTreeMap<String, String>,
    pub border: BTreeMap<String, String>,
    pub radius: BTreeMap<String, String>,
    pub shadow: BTreeMap<String, String>,
    pub typography: BTreeMap<String, String>,
    pub motion: BTreeMap<String, String>,
}

#[derive(Debug)]
pub enum TokenError {
    Parse {
        source: serde_json::Error,
    },
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { source } => write!(f, "failed to parse embedded tokens: {source}"),
        }
    }
}

impl std::error::Error for TokenError {}

pub fn load_registry() -> Result<TokenRegistry, TokenError> {
    let fluent = serde_json::from_str::<FluentTokens>(include_str!("../../../tokens/fluent.json"))
        .map_err(|source| TokenError::Parse { source })?;
    let material = serde_json::from_str::<MaterialTokens>(include_str!("../../../tokens/material.json"))
        .map_err(|source| TokenError::Parse { source })?;

    Ok(TokenRegistry { fluent, material })
}
