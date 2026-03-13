# motif

English: [`README.md`](docs/readme/en.md)

`motif` 是一个从静态类名扫描到 CSS 输出的最小工具链，当前聚焦两套内建 preset：

- `f-`：Win11 / Fluent 向
- `m-`：Google / Material 向

## 当前已有内容

- Rust CLI 和核心生成链路
- token 驱动的 preset 规则映射
- `cases/` 与 `demo/` 的多框架场景
- `xtask` 质量门禁与 demo 构建验证

## 本地入口

- `cargo run -p xtask -- quality`
- `cargo run -p xtask -- demo-builds`
- `cargo run -p motif-core -- <scan-root>`

本地便捷脚本：

- Linux / macOS: `./scripts/unix/check-quality.sh`
- Linux / macOS: `./scripts/unix/check-demo-builds.sh`
- Windows PowerShell: `./scripts/win/check-quality.ps1`
- Windows PowerShell: `./scripts/win/check-demo-builds.ps1`

## 仓库入口

- 协作规则：`.github/CONTRIBUTING.md`
- 仓库上下文：`agent/context.md`
- 质量与状态：`agent/quality.md`、`agent/status.md`
