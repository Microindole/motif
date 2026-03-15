#[path = "data/feedback.rs"]
mod feedback;
#[path = "data/structured.rs"]
mod structured;

use super::super::Declaration;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    feedback::resolve(parsed, tokens).or_else(|| structured::resolve(parsed, tokens))
}
