# motif Architecture

## 顶层结构
- `core/`：CLI 与核心生成链路
- `web/`：前端演示或交互界面
- `demo/`：原生 / TS / React / Vue 示例，按 `framework/scenario` 组织
- `cases/`：原生 / TS / React / Vue 测试输入，按 `framework/scenario` 组织
- `tests/`：e2e 与回归断言
- `tokens/`：Fluent / Material token 文件

## demo / cases 约定
- 第一层：`native/` `ts/` `react/` `vue/`
- 第二层：场景名，例如 `basic/` `variants/` `theme/`
- `cases/` 放最小扫描输入；`demo/` 放面向人看的最小可用示例
- 每个 demo 目录可单独运行 `cargo run -p motif-core -- .` 生成本目录的 `motif.css`
- `variants/` 用于覆盖 `focus:` `hover:` `active:` `dark:` 这类已支持变体

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
- `tokens/fluent.json`：Fluent 最小 token 集
- `tokens/material.json`：Material 最小 token 集
- v0.1 先由 CLI 内嵌加载 JSON，再逐步扩展为更完整的数据驱动规则

## e2e 约定
- `core/tests/e2e_cli.rs` 运行真实 `motif` 二进制
- `tests/e2e/` 保存 e2e 说明与后续夹具
- e2e 通过临时输出文件断言 CSS 结果，不污染仓库目录
- 当前 e2e 覆盖 `cases/` 与四类 `demo/` 的 `basic/` / `variants/` 场景

## 主链路
1. 扫描输入文件
2. 提取静态类名
3. 解析前缀 / 变体 / utility / value
4. 查 token 与规则
5. 生成并写出 `motif.css`
