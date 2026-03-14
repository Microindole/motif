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

use crate::demo_builds;
use crate::utils::{npm_program, path_from_repo, repo_root};

pub fn run() -> Result<(), String> {
    let root = repo_root();
    let tracked = crate::utils::tracked_files(&root)?;

    let mut failures = Vec::new();
    let mut warnings = Vec::new();

    checks::run_core_checks(&root, &tracked, &mut failures, &mut warnings)?;
    boundary::test_architecture_boundaries(&root, &tracked, &mut failures)?;
    complexity::test_complexity_heuristics(&root, &tracked, &mut failures, &mut warnings)?;
    dependencies::test_dependency_hygiene(&root, &tracked, &mut failures, &mut warnings)?;
    duplicates::test_duplicate_blocks(&root, &tracked, &mut failures, &mut warnings)?;
    changes::test_change_size(&root, &mut failures, &mut warnings);
    commit::test_commit_message(&root, &mut failures, &mut warnings);
    pr::test_pr_description(&mut failures, &mut warnings);

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
    if let Err(error) = run_motif_vite_checks(&root) {
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

fn run_motif_vite_checks(root: &std::path::Path) -> Result<(), String> {
    let motif_vite_dir = path_from_repo(root, "packages/motif-vite");
    let npm = npm_program();

    crate::utils::run_step(
        "motif-vite: install dependencies",
        npm,
        &["install", "--no-package-lock"],
        &motif_vite_dir,
    )?;
    crate::utils::run_step(
        "motif-vite: typecheck",
        npm,
        &["run", "typecheck"],
        &motif_vite_dir,
    )?;
    crate::utils::run_step("motif-vite: test", npm, &["test"], &motif_vite_dir)?;

    Ok(())
}
