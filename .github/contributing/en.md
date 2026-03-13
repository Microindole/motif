# Contributing

简体中文: [`CONTRIBUTING.md`](../CONTRIBUTING.md)

This document explains how to contribute to `motif`, with emphasis on local checks, commit discipline, and PR expectations.

## Baseline expectations

- Keep PRs small and avoid unrelated edits.
- Update `cases/`, `demo/`, or tests when behavior changes.
- Update `agent/` docs when product direction, architecture, or workflow changes.
- Prefer incremental delivery over one oversized PR.

## Repository directory roles

- `core/`: core pipeline for scanning, parsing, rule resolution, generation, and output
- `xtask/`: engineering task entrypoints such as quality gates and demo builds
- `demo/`: examples meant for direct human inspection and local running
- `cases/`: minimal scan inputs and rule coverage fixtures
- `tests/`: e2e notes and test support material
- `tokens/`: preset token data
- `scripts/`: platform-specific local convenience wrappers
- `agent/`: repository-internal product, architecture, quality, and status context
- `.github/`: GitHub workflows, templates, and collaboration entry docs

## Local checks

Cross-platform primary entrypoints:

- `cargo run -p xtask -- quality`
- `cargo run -p xtask -- demo-builds`

Convenience wrappers:

- Linux / macOS: `./scripts/unix/check-quality.sh`
- Linux / macOS: `./scripts/unix/check-demo-builds.sh`
- Windows PowerShell: `./scripts/win/check-quality.ps1`
- Windows PowerShell: `./scripts/win/check-demo-builds.ps1`

Before opening a PR, you should normally know whether `cargo run -p xtask -- quality` passes and, if not, why it fails.

## Branches and commits

- Start from the latest `origin/main` instead of reusing a stale long-lived branch.
- Use `type: description` or `type(scope): description` for commit subjects.
- Add a commit body for non-trivial changes when context would otherwise be lost.

Example:

```text
feat: refine fluent workspace controls

Add subtle hover and focus border utilities for Fluent controls.
Apply the new states to workspace fields and subtle actions.
```

## Pull request requirements

PR descriptions must follow [pull_request_template.md](../pull_request_template.md).

At minimum:

- Write 2-4 real lines under `## Summary`.
- Complete or delete template checkboxes that do not apply.
- Keep local checks and CI status aligned with reality.

## Large change override

The repository enforces a default change-size gate. If a change is genuinely a single vertical slice that cannot be split safely, use `## Large change override` in the PR template to:

- check the override item,
- explain why the change cannot be split safely,
- tell reviewers what to focus on.

This downgrades change-size failures to warnings, but still requires explicit human review. It is an exception path, not the default workflow.

## Avoid

- committing generated artifacts, caches, or build output,
- weakening gates just to force a large PR through,
- mixing unrelated refactors, cleanup, and feature work in one PR.
