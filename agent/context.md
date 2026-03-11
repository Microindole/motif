# motif Context

唯一入口：先按下面顺序读。

1. `agent/context.md`
2. `agent/motif.md`
3. `agent/product.md`
4. `agent/presets.md`
5. `agent/scope.md`
6. `agent/architecture.md`
7. `agent/status.md`
8. `agent/rules.md`

## 当前目录结构
- `agent/`：agent 入口与约束
- `core/`：Rust 核心引擎
- `web/`：前端界面或 playground
- `demo/`：面向用户的示例
- `cases/`：面向测试的最小输入
- `tests/`：端到端与回归测试
- `tokens/`：双 preset 设计 token 数据

## 一句话定位
- `motif` 是一个面向 AI 编程的 Rust 原子 CSS 编译器；当前固定提供两个内建 preset：Win11 向的 `f-` 与 Google 向的 `m-`，通过扫描 -> 解析 -> 生成 `motif.css` 来收敛样式输出。
