# E2E notes

These fixtures are exercised by `core/tests/e2e_cli.rs`, which runs the real `motif` binary against repo `cases/` and demo directories while writing CSS into temporary files.

The current e2e matrix covers `basic/`, `variants/`, `theme/`, and `workspace/` across `native`, `ts`, `react`, and `vue`.

Build-level demo verification lives in `cargo run -p xtask -- demo-builds`.

Repository-wide hard gates live in `cargo run -p xtask -- quality`, with `scripts/check-quality.ps1` kept only as a Windows-local wrapper.
