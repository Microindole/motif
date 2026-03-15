#[path = "fluent/data.rs"]
mod data;
#[path = "fluent/foundation.rs"]
mod foundation;
#[path = "fluent/shell.rs"]
mod shell;

use super::shared::typography;
use super::Declaration;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    data::resolve(parsed, tokens)
        .or_else(|| foundation::resolve(parsed, tokens))
        .or_else(|| shell::resolve(parsed, tokens))
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
