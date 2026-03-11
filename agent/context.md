# motif Context

唯一入口：先按下面顺序读。

1. `agent/context.md`
2. `agent/motif.md`
3. `agent/scope.md`
4. `agent/architecture.md`
5. `agent/status.md`
6. `agent/rules.md`

## 当前目录结构
- `agent/`：agent 入口与约束
- `core/`：Rust 核心引擎
- `web/`：前端界面或 playground
- `demo/`：面向用户的示例
- `cases/`：面向测试的最小输入
- `tests/`：端到端与回归测试
- `tokens/`：设计语言 token 数据

## 一句话定位
- `motif` 是一个 Rust 多风格原子 CSS 引擎；v0.1 先验证扫描 -> 解析 -> 生成 `motif.css` 的最小闭环。