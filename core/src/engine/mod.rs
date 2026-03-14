use crate::gen;
use crate::parse;
use crate::rule::{self, RuleMatch};
use crate::scan::{self, ScanError, ScanResult, SourceInput};
use crate::token::{self, TokenError, TokenRegistry};

#[derive(Debug)]
pub struct CompileResult {
    pub scan_result: ScanResult,
    pub rules: Vec<RuleMatch>,
    pub stylesheet: String,
}

#[derive(Debug)]
pub enum CompileError {
    Scan(ScanError),
    Token(TokenError),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Scan(source) => write!(f, "{source}"),
            Self::Token(source) => write!(f, "{source}"),
        }
    }
}

impl std::error::Error for CompileError {}

pub fn compile_root(root: &std::path::Path) -> Result<CompileResult, CompileError> {
    let scan_result = scan::scan_root(root).map_err(CompileError::Scan)?;
    let token_registry = token::load_registry().map_err(CompileError::Token)?;
    Ok(compile_scan_result(scan_result, &token_registry))
}

pub fn compile_sources(sources: &[SourceInput], token_registry: &TokenRegistry) -> CompileResult {
    let scan_result = scan::scan_sources(sources);
    compile_scan_result(scan_result, token_registry)
}

fn compile_scan_result(scan_result: ScanResult, token_registry: &TokenRegistry) -> CompileResult {
    let rules = scan_result
        .class_names
        .iter()
        .filter_map(|class_name| parse::parse_class_name(class_name).ok())
        .filter_map(|parsed| rule::resolve_rule(&parsed, token_registry))
        .collect::<Vec<_>>();
    let stylesheet = gen::render_stylesheet(&rules);

    CompileResult {
        scan_result,
        rules,
        stylesheet,
    }
}
