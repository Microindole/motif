#[path = "material/data.rs"]
mod data;
#[path = "material/foundation.rs"]
mod foundation;
#[path = "material/shell.rs"]
mod shell;

use super::shared::typography;
use super::Declaration;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    foundation::resolve(parsed, tokens)
        .or_else(|| shell::resolve(parsed, tokens))
        .or_else(|| data::resolve(parsed, tokens))
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
