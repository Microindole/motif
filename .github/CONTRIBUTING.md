# 贡献指南

English version: [`CONTRIBUTING.md`](./contributing/en.md)

本文档面向参与 `motif` 仓库协作的开发者，重点说明本地检查、提交流程和 PR 约束。

## 基本要求

- 尽量保持小 PR，避免把无关修改混进同一条变更。
- 行为变化要同步更新 `cases/`、`demo/` 或测试。
- 产品方向、架构或流程变化要同步更新 `agent/` 文档。
- 默认先走增量提交，不要把“大功能”一次性堆成一个难审查的 PR。

## 仓库目录职责

- `core/`：扫描、解析、规则、生成、输出等核心链路
- `xtask/`：质量门禁和 demo 构建等工程任务入口
- `demo/`：给人直接查看和运行的示例
- `cases/`：最小扫描输入和规则覆盖样本
- `tests/`：e2e 说明和测试辅助材料
- `tokens/`：preset token 数据
- `scripts/`：按平台划分的本地便捷包装脚本
- `agent/`：仓库内部产品、架构、质量和状态上下文
- `.github/`：GitHub workflow、模板和协作入口文档

## 本地检查

跨平台主入口：

- `cargo run -p xtask -- quality`
- `cargo run -p xtask -- demo-builds`

本地便捷包装：

- Linux / macOS: `./scripts/unix/check-quality.sh`
- Linux / macOS: `./scripts/unix/check-demo-builds.sh`
- Windows PowerShell: `./scripts/win/check-quality.ps1`
- Windows PowerShell: `./scripts/win/check-demo-builds.ps1`

提交前至少应确保 `cargo run -p xtask -- quality` 通过，或明确知道失败原因。

## 分支与提交

- 建议从最新的 `origin/main` 重新切分支，不要长期复用漂移过久的旧分支。
- 提交信息使用 `type: description` 或 `type(scope): description`。
- 非平凡提交建议补 body，说明变更目的和约束。

示例：

```text
feat: refine fluent workspace controls

Add subtle hover and focus border utilities for Fluent controls.
Apply the new states to workspace fields and subtle actions.
```

## Pull Request 要求

PR 描述以 [pull_request_template.md](./pull_request_template.md) 为准。

至少要做到：

- 在 `## Summary` 下写 2 到 4 行非模板说明。
- 完成或删除不适用的模板勾选项，不要把空模板直接提交。
- 本地检查和 CI 状态要与实际情况一致。

## Large change override

仓库默认限制 PR 的变更规模。如果确实是一个不能安全拆分的纵向改动，可以在 PR 模板的 `## Large change override` 中：

- 勾选 override 项
- 写清为什么不能拆分
- 写清 reviewer 应重点看什么

这样会把变更规模从 hard gate 降成 warning，但仍然要求人工审查。这个机制只用于少数确实无法继续拆分的变更，不是常规通道。

## 不建议的做法

- 把 generated artifact、缓存目录、构建输出提交进 Git
- 为了过门禁直接放宽阈值，而不是先拆分变更或写清 override 理由
- 在同一个 PR 里混入无关重构、格式清理和功能改动
