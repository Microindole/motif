#[path = "foundation/base.rs"]
mod base;
#[path = "foundation/controls.rs"]
mod controls;

use super::super::Declaration;
use crate::parse::ParsedClass;
use crate::token::TokenRegistry;

pub(super) fn resolve(parsed: &ParsedClass, tokens: &TokenRegistry) -> Option<Vec<Declaration>> {
    base::resolve(parsed, tokens).or_else(|| controls::resolve(parsed, tokens))
}

pub(crate) use base::surface_container;
pub(crate) use controls::field;
