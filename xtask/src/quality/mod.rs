// Orchestrate quality gates in a fixed order so the local and CI entrypoints behave the same way.
mod boundary;
mod changes;
mod checks;
mod commit;
mod complexity;
mod dependencies;
mod duplicates;

use crate::demo_builds;
use crate::utils::repo_root;

pub fn run() -> Result<(), String> {
    let root = repo_root();
    let tracked = crate::utils::tracked_files(&root)?;

    let mut failures = Vec::new();
    let mut warnings = Vec::new();

    checks::run_core_checks(&root, &tracked, &mut failures, &mut warnings)?;
    boundary::test_architecture_boundaries(&root, &tracked, &mut failures)?;
    complexity::test_complexity_heuristics(&root, &tracked, &mut failures, &mut warnings)?;
    dependencies::test_dependency_hygiene(&root, &tracked, &mut failures, &mut warnings)?;
    duplicates::test_duplicate_blocks(&root, &tracked, &mut warnings)?;
    changes::test_change_size(&root, &mut failures, &mut warnings);
    commit::test_commit_message(&root, &mut failures, &mut warnings);

    if let Err(error) = crate::utils::run_step(
        "cargo fmt --all --check",
        "cargo",
        &["fmt", "--all", "--check"],
        &root,
    ) {
        failures.push(error);
    }
    if let Err(error) = crate::utils::run_step(
        "cargo clippy --workspace --all-targets --all-features -- -D warnings",
        "cargo",
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
        &root,
    ) {
        failures.push(error);
    }
    if let Err(error) = crate::utils::run_step(
        "cargo test -p motif-core",
        "cargo",
        &["test", "-p", "motif-core"],
        &root,
    ) {
        failures.push(error);
    }
    if let Err(error) = demo_builds::run() {
        failures.push(error);
    }

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
