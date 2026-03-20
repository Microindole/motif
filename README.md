# motif

English: [README.md](docs/readme/en.md)

`motif` 是一个从静态类名扫描到 CSS 输出的最小工具链，当前聚焦两套内建 preset：

- `f-`：Win11 / Fluent 向
- `m-`：Google / Material 向

## 当前已有内容

- Rust CLI 和核心生成链路
- token 驱动的 preset 规则映射
- `cases/` 与 `demo/` 的多框架场景
- `xtask` 质量门禁与 demo 构建验证
- `packages/motif-vite/` 最小 Vite 插件骨架

## 本地入口

- `cargo run -p xtask -- quality`
- `cargo run -p xtask -- demo-builds`
- `cargo run -p motif-core -- <scan-root>`

本地便捷脚本：

- Linux / macOS: `./scripts/unix/check-quality.sh`
- Linux / macOS: `./scripts/unix/check-demo-builds.sh`
- Windows PowerShell: `./scripts/win/check-quality.ps1`
- Windows PowerShell: `./scripts/win/check-demo-builds.ps1`

## 当前方向

- 最终接入体验要像 Tailwind 一样直接
- 但能力仍坚持 preset-first，而不是开放式超大 utility 集
- Rust 现在是核心原型，不是最终用户入口
- 下一阶段会继续推进 `npm` / `Vite` / watch / HMR 路线

## 仓库入口

- 协作规则：`.github/CONTRIBUTING.md`
- 仓库上下文：`agent/context.md`
- 质量与状态：`agent/quality.md`、`agent/status.md`
