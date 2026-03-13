# motif

简体中文: [`README.md`](../../README.md)

`motif` is a minimal pipeline that scans static class names and emits CSS. It currently focuses on two built-in presets:

- `f-`: Win11 / Fluent-oriented
- `m-`: Google / Material-oriented

## Current contents

- Rust CLI and core generation pipeline
- token-driven preset rule mapping
- multi-framework scenarios under `cases/` and `demo/`
- `xtask` quality gates and demo build verification

## Local entrypoints

- `cargo run -p xtask -- quality`
- `cargo run -p xtask -- demo-builds`
- `cargo run -p motif-core -- <scan-root>`

Convenience wrappers:

- Linux / macOS: `./scripts/unix/check-quality.sh`
- Linux / macOS: `./scripts/unix/check-demo-builds.sh`
- Windows PowerShell: `./scripts/win/check-quality.ps1`
- Windows PowerShell: `./scripts/win/check-demo-builds.ps1`

## Repository entrypoints

- contribution rules: `.github/CONTRIBUTING.md`
- repository context: `agent/context.md`
- quality and status: `agent/quality.md`, `agent/status.md`
