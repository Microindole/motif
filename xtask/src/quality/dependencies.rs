// Keep dependency growth visible before AI changes turn small demos or tooling into dependency sinks.

use super::changes;
use super::dependencies_parse::{
    collect_version_map, count_cargo_dependencies, count_json_object_entries,
    extract_added_cargo_dependencies, extract_added_json_dependencies,
};
use crate::utils::{command_output, path_from_repo, read_lines};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const CORE_WARN_DEPS: usize = 4;
const CORE_FAIL_DEPS: usize = 8;
const XTASK_WARN_DEPS: usize = 3;
const XTASK_FAIL_DEPS: usize = 6;
const DEMO_WARN_DEPS: usize = 8;
const DEMO_FAIL_DEPS: usize = 10;

pub fn test_dependency_hygiene(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    test_cargo_manifest(
        root,
        "core/Cargo.toml",
        CORE_WARN_DEPS,
        CORE_FAIL_DEPS,
        failures,
        warnings,
    )?;
    test_cargo_manifest(
        root,
        "xtask/Cargo.toml",
        XTASK_WARN_DEPS,
        XTASK_FAIL_DEPS,
        failures,
        warnings,
    )?;
    test_package_manifests(root, tracked, failures, warnings)?;
    test_added_dependencies(root, failures, warnings)?;
    Ok(())
}

fn test_cargo_manifest(
    root: &Path,
    manifest: &str,
    warn_limit: usize,
    fail_limit: usize,
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let path = path_from_repo(root, manifest);
    let lines = read_lines(&path)?;
    let count = count_cargo_dependencies(&lines);
    if count > fail_limit {
        failures.push(format!(
            "{manifest} declares {count} direct dependencies; hard limit is {fail_limit}"
        ));
    } else if count > warn_limit {
        warnings.push(format!(
            "{manifest} declares {count} direct dependencies; review whether they all pay for themselves"
        ));
    }
    Ok(())
}

fn test_package_manifests(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let manifests: Vec<&String> = tracked
        .iter()
        .filter(|file| file.ends_with("package.json"))
        .collect();

    let mut versions: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for manifest in manifests {
        let path = path_from_repo(root, manifest);
        let content = std::fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {manifest}: {error}"))?;
        let dependency_count = count_json_object_entries(&content, "dependencies")
            + count_json_object_entries(&content, "devDependencies");

        if dependency_count > DEMO_FAIL_DEPS {
            failures.push(format!(
                "{manifest} declares {dependency_count} npm dependencies; hard limit is {DEMO_FAIL_DEPS}"
            ));
        } else if dependency_count > DEMO_WARN_DEPS {
            warnings.push(format!(
                "{manifest} declares {dependency_count} npm dependencies; review whether the demo is pulling too much"
            ));
        }

        collect_version_map(
            &mut versions,
            &content,
            &["dependencies", "devDependencies"],
        );
    }

    for (name, version_set) in versions {
        if version_set.len() >= 2 {
            warnings.push(format!(
                "npm dependency `{name}` uses multiple versions across demos: {}",
                version_set.into_iter().collect::<Vec<_>>().join(", ")
            ));
        }
    }

    Ok(())
}

fn test_added_dependencies(
    root: &Path,
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let Some(spec) = changes::diff_spec(root) else {
        warnings.push("dependency-diff check skipped: no usable diff base available".to_string());
        return Ok(());
    };

    for manifest in ["core/Cargo.toml", "xtask/Cargo.toml"] {
        let diff = manifest_diff(root, &spec.range, manifest)?;
        let names = extract_added_cargo_dependencies(&diff);
        if names.is_empty() {
            continue;
        }

        let message = format!(
            "{manifest} adds direct dependencies in {}: {}",
            spec.label,
            names.join(", ")
        );
        if spec.hard_gate {
            failures.push(message);
        } else {
            warnings.push(format!("{message}; local branch signal only"));
        }
    }

    for manifest in tracked_demo_manifests(root)? {
        let diff = manifest_diff(root, &spec.range, &manifest)?;
        let names = extract_added_json_dependencies(&diff);
        if names.is_empty() {
            continue;
        }

        let message = format!(
            "{manifest} adds npm dependencies in {}: {}",
            spec.label,
            names.join(", ")
        );
        if spec.hard_gate && names.len() >= 3 {
            failures.push(format!(
                "{message}; demo dependency additions are too broad for one change"
            ));
        } else {
            warnings.push(message);
        }
    }

    Ok(())
}

fn tracked_demo_manifests(root: &Path) -> Result<Vec<String>, String> {
    let output = command_output("git", &["ls-files", "demo/**/package.json"], root)?;
    Ok(output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.replace('\\', "/"))
        .collect())
}

fn manifest_diff(root: &Path, range: &str, manifest: &str) -> Result<Vec<String>, String> {
    let output = command_output(
        "git",
        &["diff", "--unified=20", range, "--", manifest],
        root,
    )?;
    Ok(output.lines().map(|line| line.to_string()).collect())
}
