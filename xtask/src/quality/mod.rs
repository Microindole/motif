// Orchestrate quality gates in a fixed order so the local and CI entrypoints behave the same way.
mod boundary;
mod changes;
mod checks;
pub mod commit;
mod complexity;
mod dependencies;
pub mod dependencies_parse;
mod duplicates;
mod github_event;
pub mod pr;
mod runner;

use crate::utils::repo_root;

pub fn run() -> Result<(), String> {
    let root = repo_root();
    let tracked = crate::utils::tracked_files(&root)?;

    let mut failures = Vec::new();
    let mut warnings = Vec::new();

    run_repo_quality_checks(&root, &tracked, &mut failures, &mut warnings)?;
    runner::run_command_quality_checks(&root, &mut failures);

    if !warnings.is_empty() {
        println!("\nSoft warnings:");
        for warning in warnings {
            println!("- {warning}");
        }
    }

    if failures.is_empty() {
        println!("\nquality checks passed");
        Ok(())
    } else {
        println!("\nHard gate failures:");
        for failure in failures {
            println!("- {failure}");
        }
        Err("quality checks failed".to_string())
    }
}

fn run_repo_quality_checks(
    root: &std::path::Path,
    tracked: &[String],
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    checks::run_core_checks(root, tracked, failures, warnings)?;
    boundary::test_architecture_boundaries(root, tracked, failures)?;
    complexity::test_complexity_heuristics(root, tracked, failures, warnings)?;
    dependencies::test_dependency_hygiene(root, tracked, failures, warnings)?;
    duplicates::test_duplicate_blocks(root, tracked, failures, warnings)?;
    changes::test_change_size(root, failures, warnings);
    commit::test_commit_message(root, failures, warnings);
    pr::test_pr_description(failures, warnings);
    Ok(())
}
