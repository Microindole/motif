use crate::utils::{path_from_repo, read_lines};
use std::path::Path;

const FUNCTION_WARN_LINES: usize = 60;
const FUNCTION_FAIL_LINES: usize = 90;
const PARAM_WARN_COUNT: usize = 5;
const NESTING_WARN_DEPTH: usize = 4;
const FILE_WARN_FUNCTIONS: usize = 12;

pub fn test_complexity_heuristics(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    for file in tracked {
        if !is_complexity_target(file) {
            continue;
        }

        let path = path_from_repo(root, file);
        if !path.is_file() {
            continue;
        }

        let lines = read_lines(&path)?;
        let functions = extract_functions(&lines);

        if functions.len() > FILE_WARN_FUNCTIONS {
            warnings.push(format!(
                "{file} defines {} functions; consider splitting responsibilities",
                functions.len()
            ));
        }

        for function in functions {
            let qualified = format!("{file}::{}", function.name);
            if function.line_count > FUNCTION_FAIL_LINES {
                failures.push(format!(
                    "{qualified} is {} lines, exceeds hard limit {}",
                    function.line_count, FUNCTION_FAIL_LINES
                ));
            } else if function.line_count > FUNCTION_WARN_LINES {
                warnings.push(format!(
                    "{qualified} is {} lines; consider extracting helpers",
                    function.line_count
                ));
            }

            if function.parameter_count > PARAM_WARN_COUNT {
                warnings.push(format!(
                    "{qualified} has {} parameters; consider bundling state or options",
                    function.parameter_count
                ));
            }

            if function.max_nesting > NESTING_WARN_DEPTH {
                warnings.push(format!(
                    "{qualified} reaches nesting depth {}; logic may be too tangled",
                    function.max_nesting
                ));
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
struct FunctionStats {
    name: String,
    parameter_count: usize,
    line_count: usize,
    max_nesting: usize,
}

fn is_complexity_target(file: &str) -> bool {
    (file.starts_with("core/src/") || file.starts_with("xtask/src/")) && file.ends_with(".rs")
}

fn extract_functions(lines: &[String]) -> Vec<FunctionStats> {
    let mut functions = Vec::new();
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index].trim_start();
        if !looks_like_function_start(line) {
            index += 1;
            continue;
        }

        let signature_start = index;
        let mut signature = line.to_string();
        while !signature.contains('{') && index + 1 < lines.len() {
            index += 1;
            signature.push(' ');
            signature.push_str(lines[index].trim());
        }

        let start_depth = brace_delta(&signature).max(0) as usize;
        let mut depth = start_depth;
        let body_start = index;
        let mut max_depth = depth;

        while index + 1 < lines.len() {
            index += 1;
            let next = &lines[index];
            depth = depth.saturating_add_signed(brace_delta(next));
            max_depth = max_depth.max(depth);
            if depth == 0 {
                break;
            }
        }

        functions.push(FunctionStats {
            name: extract_name(&signature),
            parameter_count: count_parameters(&signature),
            line_count: index.saturating_sub(signature_start) + 1,
            max_nesting: max_depth.saturating_sub(1),
        });

        if body_start == index {
            index += 1;
        }
    }

    functions
}

fn looks_like_function_start(line: &str) -> bool {
    ["fn ", "pub fn ", "pub(crate) fn ", "pub(super) fn "]
        .iter()
        .any(|prefix| line.starts_with(prefix))
}

fn extract_name(signature: &str) -> String {
    let after_fn = signature.split("fn ").nth(1).unwrap_or(signature);
    after_fn
        .split('(')
        .next()
        .unwrap_or(after_fn)
        .trim()
        .to_string()
}

fn count_parameters(signature: &str) -> usize {
    let Some((_, rest)) = signature.split_once('(') else {
        return 0;
    };
    let params = rest.split(')').next().unwrap_or("").trim();
    if params.is_empty() {
        0
    } else {
        params
            .split(',')
            .filter(|part| !part.trim().is_empty())
            .count()
    }
}

fn brace_delta(line: &str) -> isize {
    line.chars().fold(0, |acc, ch| match ch {
        '{' => acc + 1,
        '}' => acc - 1,
        _ => acc,
    })
}
