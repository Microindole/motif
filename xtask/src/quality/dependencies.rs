// Keep dependency growth visible before AI changes turn small demos or tooling into dependency sinks.

use super::changes;
use super::dependencies_parse::{
    collect_version_map, count_cargo_dependencies, count_json_object_entries,
    extract_added_cargo_dependencies, extract_added_json_dependencies,
};
use crate::utils::{command_output, path_from_repo, read_lines};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const CORE_LIMITS: ManifestLimits = ManifestLimits::new(4, 8);
const XTASK_LIMITS: ManifestLimits = ManifestLimits::new(3, 6);
const DEMO_LIMITS: ManifestLimits = ManifestLimits::new(8, 10);

#[derive(Clone, Copy)]
struct ManifestLimits {
    warn: usize,
    fail: usize,
}

impl ManifestLimits {
    const fn new(warn: usize, fail: usize) -> Self {
        Self { warn, fail }
    }
}

pub fn test_dependency_hygiene(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    test_cargo_manifest(root, "core/Cargo.toml", CORE_LIMITS, failures, warnings)?;
    test_cargo_manifest(root, "xtask/Cargo.toml", XTASK_LIMITS, failures, warnings)?;
    test_package_manifests(root, tracked, failures, warnings)?;
    test_added_dependencies(root, failures, warnings)?;
    Ok(())
}

fn test_cargo_manifest(
    root: &Path,
    manifest: &str,
    limits: ManifestLimits,
    failures: &mut Vec<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let path = path_from_repo(root, manifest);
    let lines = read_lines(&path)?;
    let count = count_cargo_dependencies(&lines);
    if count > limits.fail {
        failures.push(format!(
            "{manifest} declares {count} direct dependencies; hard limit is {}",
            limits.fail
        ));
    } else if count > limits.warn {
        warnings.push(format!(
            "[warn] {manifest} declares {count} direct dependencies; review whether they all pay for themselves"
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

        if dependency_count > DEMO_LIMITS.fail {
            failures.push(format!(
                "{manifest} declares {dependency_count} npm dependencies; hard limit is {}",
                DEMO_LIMITS.fail
            ));
        } else if dependency_count > DEMO_LIMITS.warn {
            warnings.push(format!(
                "[warn] {manifest} declares {dependency_count} npm dependencies; review whether the demo is pulling too much"
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
                "[warn] npm dependency `{name}` uses multiple versions across demos: {}",
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
        warnings.push(
            "[info] dependency-diff check skipped: no usable diff base available".to_string(),
        );
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
            warnings.push(format!("[warn] {message}; local branch signal only"));
        }
    }

    for manifest in tracked_demo_manifests(root)? {
        let diff = manifest_diff(root, &spec.range, &manifest)?;
        let names = extract_added_json_dependencies(&diff);
        if names.is_empty() {
            continue;
        }

        let severity = classify_demo_dependency_risk(&names);
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
            warnings.push(format!("[{severity}] {message}"));
        }
    }

    Ok(())
}

fn classify_demo_dependency_risk(names: &[String]) -> &'static str {
    let mut has_warn = false;
    for name in names {
        if is_candidate_dependency(name) {
            return "candidate";
        }
        if !is_toolchain_dependency(name) {
            has_warn = true;
        }
    }

    if has_warn {
        "warn"
    } else {
        "info"
    }
}

fn is_toolchain_dependency(name: &str) -> bool {
    name == "vite"
        || name == "typescript"
        || name.starts_with("@types/")
        || name.starts_with("@vitejs/")
        || name == "tslib"
}

fn is_candidate_dependency(name: &str) -> bool {
    matches!(
        name,
        "next"
            | "nuxt"
            | "gatsby"
            | "tailwindcss"
            | "bootstrap"
            | "antd"
            | "@mui/material"
            | "@chakra-ui/react"
            | "lodash"
            | "moment"
    )
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

#[cfg(test)]
mod tests {
    use super::classify_demo_dependency_risk;

    #[test]
    fn marks_toolchain_only_demo_additions_as_info() {
        let names = vec!["vite".to_string(), "typescript".to_string()];
        assert_eq!(classify_demo_dependency_risk(&names), "info");
    }

    #[test]
    fn marks_regular_demo_additions_as_warn() {
        let names = vec!["react".to_string(), "zustand".to_string()];
        assert_eq!(classify_demo_dependency_risk(&names), "warn");
    }

    #[test]
    fn marks_heavy_or_overlapping_demo_additions_as_candidate() {
        let names = vec!["tailwindcss".to_string()];
        assert_eq!(classify_demo_dependency_risk(&names), "candidate");
    }
}
