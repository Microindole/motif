// Keep core layers one-way so AI changes cannot quietly tangle scanning, parsing, rules, generation, and IO.
use crate::utils::path_from_repo;
use std::path::Path;

pub fn test_architecture_boundaries(
    root: &Path,
    tracked: &[String],
    failures: &mut Vec<String>,
) -> Result<(), String> {
    for file in tracked {
        if !(file.starts_with("core/src/") && file.ends_with(".rs")) {
            continue;
        }

        let content = std::fs::read_to_string(path_from_repo(root, file))
            .map_err(|error| format!("failed to read {file}: {error}"))?;

        for rule in boundary_rules() {
            if !file.starts_with(rule.prefix) {
                continue;
            }

            for (needle, message) in rule.forbidden {
                if content.contains(needle) {
                    failures.push(format!("{file}: {message}"));
                }
            }
        }
    }

    Ok(())
}

struct BoundaryRule {
    prefix: &'static str,
    forbidden: &'static [(&'static str, &'static str)],
}

fn boundary_rules() -> &'static [BoundaryRule] {
    &[
        BoundaryRule {
            prefix: "core/src/scan/",
            forbidden: &[
                ("crate::gen", "scan layer must not depend on gen"),
                ("crate::rule", "scan layer must not depend on rule"),
                ("crate::token", "scan layer must not depend on token"),
                ("crate::write", "scan layer must not depend on write"),
            ],
        },
        BoundaryRule {
            prefix: "core/src/parse/",
            forbidden: &[
                ("crate::scan", "parse layer must not depend on scan"),
                ("crate::rule", "parse layer must not depend on rule"),
                ("crate::token", "parse layer must not depend on token"),
                ("crate::gen", "parse layer must not depend on gen"),
                ("crate::write", "parse layer must not depend on write"),
            ],
        },
        BoundaryRule {
            prefix: "core/src/token/",
            forbidden: &[
                ("std::fs", "token layer must not do file IO"),
                ("std::io", "token layer must not do file IO"),
                ("crate::scan", "token layer must not depend on scan"),
                ("crate::gen", "token layer must not depend on gen"),
                ("crate::write", "token layer must not depend on write"),
            ],
        },
        BoundaryRule {
            prefix: "core/src/rule/",
            forbidden: &[
                ("std::fs", "rule layer must not do file IO"),
                ("std::io", "rule layer must not do file IO"),
                ("crate::scan", "rule layer must not depend on scan"),
                ("crate::gen", "rule layer must not depend on gen"),
                ("crate::write", "rule layer must not depend on write"),
            ],
        },
        BoundaryRule {
            prefix: "core/src/gen/",
            forbidden: &[
                ("std::fs", "gen layer must not do file IO"),
                ("std::io", "gen layer must not do file IO"),
                ("crate::scan", "gen layer must not depend on scan"),
                ("crate::write", "gen layer must not depend on write"),
            ],
        },
        BoundaryRule {
            prefix: "core/src/write/",
            forbidden: &[
                ("crate::scan", "write layer must not depend on scan"),
                ("crate::parse", "write layer must not depend on parse"),
                ("crate::rule", "write layer must not depend on rule"),
                ("crate::token", "write layer must not depend on token"),
                ("crate::gen", "write layer must not depend on gen"),
            ],
        },
    ]
}
