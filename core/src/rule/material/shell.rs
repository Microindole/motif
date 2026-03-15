#[path = "shell/feedback.rs"]
mod feedback;
#[path = "shell/layout.rs"]
mod layout;

use super::super::Declaration;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    feedback::resolve(parsed, tokens).or_else(|| layout::resolve(parsed, tokens))
}
