use std::path::Path;

use crate::demo_builds;
use crate::utils::{npm_program, path_from_repo};

pub(crate) fn run_command_quality_checks(root: &Path, failures: &mut Vec<String>) {
    run_cargo_checks(root, failures);
    if let Err(error) = run_motif_vite_checks(root) {
        failures.push(error);
    }
    if let Err(error) = demo_builds::run() {
        failures.push(error);
    }
}

fn run_cargo_checks(root: &Path, failures: &mut Vec<String>) {
    for (label, args) in [
        ("cargo fmt --all --check", vec!["fmt", "--all", "--check"]),
        (
            "cargo clippy --workspace --all-targets --all-features -- -D warnings",
            vec![
                "clippy",
                "--workspace",
                "--all-targets",
                "--all-features",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("cargo test -p motif-core", vec!["test", "-p", "motif-core"]),
    ] {
        if let Err(error) = crate::utils::run_step(label, "cargo", &args, root) {
            failures.push(error);
        }
    }
}

fn run_motif_vite_checks(root: &Path) -> Result<(), String> {
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
