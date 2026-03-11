# E2E notes

These fixtures are exercised by `core/tests/e2e_cli.rs`, which runs the real `motif` binary against repo `cases/` and demo directories while writing CSS into temporary files.

The current e2e matrix covers `basic/`, `variants/`, `theme/`, and `workspace/` across `native`, `ts`, `react`, and `vue`.

Build-level demo verification lives in `scripts/check-demo-builds.ps1`. It generates `motif.css`, installs demo dependencies, and runs each TS / React / Vue build.
