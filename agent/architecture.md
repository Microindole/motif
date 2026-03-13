# motif Architecture

## 顶层结构
- `core/`：CLI 与核心生成链路
- `xtask/`：跨平台工程任务入口，承载质量检查与 demo 构建验证
- `web/`：前端演示或交互界面
- `demo/`：原生 / TS / React / Vue 示例，按 `framework/scenario` 组织
- `cases/`：原生 / TS / React / Vue 测试输入，按 `framework/scenario` 组织
- `tests/`：e2e 与回归断言
- `tokens/`：双 preset token 文件；当前仅 `f-` / `m-`
- `.github/`：GitHub Actions、Dependabot、PR 模板

## demo / cases 约定
- 第一层：`native/` `ts/` `react/` `vue/`
- 第二层：场景名，例如 `basic/` `variants/` `theme/` `workspace/`
- `cases/` 放最小扫描输入；`demo/` 放面向人看的最小可用示例
- 每个 demo 目录可单独运行 `cargo run -p motif-core -- .` 生成本目录的 `motif.css`
- `variants/` 用于覆盖 `focus:` `hover:` `active:` `dark:` 这类已支持变体
- `theme/` 用于并排展示 Win11 向 `f-` preset 与 Google 向 `m-` preset 的视觉差异
- `workspace/` 用于验证真实页面高频语义：`label`、`divider`、`field`、`action`

## core 结构
- `src/lib.rs`：库入口，暴露扫描、解析、规则、生成、token、输出模块
- `src/main.rs`：CLI 入口
- `src/cli/`：命令参数与流程编排
- `src/scan/`：文件发现与类名提取
- `src/parse/`：DSL 解析
- `src/gen/`：CSS 生成
- `src/token/`：token 载入与模型
- `src/rule/`：规则映射
- `src/write/`：输出写入
- `src/shared/`：共享类型与错误
- `tests/`：Rust 集成测试，不在源文件内内联

## token 约定
- `tokens/fluent.json`：Win11 向 `f-` preset 的最小 token 集
- `tokens/material.json`：Google 向 `m-` preset 的最小 token 集
- v0.1 先由 CLI 内嵌加载 JSON，不提前平台化第三套 preset

## 质量闸门结构
- `cargo run -p xtask -- quality`：跨平台硬闸门入口
- `cargo run -p xtask -- demo-builds`：跨平台 demo 构建验证入口
- `scripts/unix/check-quality.sh`：Linux/macOS/POSIX shell 本地便捷包装
- `scripts/unix/check-demo-builds.sh`：Linux/macOS/POSIX shell 本地便捷包装
- `scripts/win/check-quality.ps1`：Windows PowerShell 本地便捷包装
- `scripts/win/check-demo-builds.ps1`：Windows PowerShell 本地便捷包装
- `.github/workflows/quality.yml`：运行格式化、lint、测试、结构检查、demo 构建
- `.github/workflows/coverage.yml`：运行 Rust 覆盖率阈值检查
- `.github/workflows/codeql.yml`：运行 GitHub CodeQL 扫描
- `.github/dependabot.yml`：维护 Cargo、npm、GitHub Actions 依赖
- `.github/pull_request_template.md`：保留给人工 review 的结构清单

## e2e 约定
- `core/tests/e2e_cli.rs` 运行真实 `motif` 二进制
- `tests/e2e/` 保存 e2e 说明与后续夹具
- e2e 通过临时输出文件断言 CSS 结果，不污染仓库目录
- 当前 e2e 覆盖 `cases/` 与四类 `demo/` 的 `basic/` / `variants/` / `theme/` / `workspace/` 场景

## 主链路
1. 扫描输入文件
2. 提取静态类名
3. 解析前缀 / 变体 / utility / value
4. 查 token 与规则
5. 生成并写出 `motif.css`
6. 通过 `xtask` 质量闸门压制 AI 生成代码的熵增



