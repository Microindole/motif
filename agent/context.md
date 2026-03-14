# motif Context

唯一入口：先按下面顺序读。

1. `agent/context.md`
2. `agent/motif.md`
3. `agent/product.md`
4. `agent/quality.md`
5. `agent/presets.md`
6. `agent/scope.md`
7. `agent/architecture.md`
8. `agent/status.md`
9. `agent/rules.md`

## 当前目录结构
- `agent/`：agent 入口与约束
- `core/`：核心样式引擎与当前 Rust 原型实现
- `xtask/`：跨平台工程任务入口
- `packages/`：前端工具链入口骨架，例如 `motif-vite`
- `web/`：前端界面或 playground
- `demo/`：面向用户的示例
- `cases/`：面向测试的最小输入
- `tests/`：端到端与回归测试
- `tokens/`：preset 设计 token 数据
- `.github/`：CI、机器人与 PR 模板

## 一句话定位
- `motif` 是一个面向 AI 编程的、preset 驱动的原子样式工具；目标接入体验要像 Tailwind 一样直接，但风格空间保持强约束，当前固定提供两个内建 preset：Win11 向的 `f-` 与 Google 向的 `m-`，并将逐步从 Rust 原型演进到前端工具链优先的使用形态。
