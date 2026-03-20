# motif Architecture

## 顶层结构
- `core/`：核心样式引擎与当前 Rust 原型实现
- `xtask/`：跨平台工程任务入口，承载质量检查与 demo 构建验证
- `packages/`：前端工具链入口骨架，例如 `motif-vite`
- `web/`：前端演示或交互界面
- `demo/`：原生 / TS / React / Vue 示例，按 `framework/scenario` 组织
- `cases/`：原生 / TS / React / Vue 测试输入，按 `framework/scenario` 组织
- `tests/`：e2e 与回归断言
- `tokens/`：preset token 文件；当前仅 `f-` / `m-`
- `.github/`：GitHub Actions、Dependabot、PR 模板

## 目标分层
### 1. `motif-core`
负责稳定的样式语言与规则内核：
- 类名扫描与提取
- DSL 解析
- rule resolution
- token schema
- CSS 生成 contract

当前这层由 Rust 原型承载，后续可以继续保留为底层引擎、对照实现或迁移参考，但不应与最终用户入口强绑定。

### 2. `motif-dev`
负责开发态体验：
- watch
- 增量扫描
- 自动更新 CSS
- bundler 集成
- 热更新

这一层最终应该更贴近前端工具链，例如 `npm` 包与 `Vite` 插件，而不是要求用户手动运行 Rust CLI。

### 3. `motif-tools`
负责未来的高级能力：
- preset migration
- 文件级或仓库级 preset 切换
- codemod / lint / fix
- 风格迁移前的分析与预检

这层不是当前阶段必做，但现在的语义设计要为它预留空间。

## 当前 core contract
当前 Rust 原型里，优先保住下面这些稳定边界：
- `scan::scan_root`：从真实目录扫描输入
- `scan::scan_sources`：从内存源码集合扫描输入，供未来 plugin/watch 复用
- `parse::parse_class_name`：类名解析入口
- `rule::resolve_rule`：preset rule resolution 入口
- `engine::compile_root`：目录级编译入口
- `engine::compile_sources`：纯内存编译入口
- `gen::render_stylesheet`：CSS 渲染入口

原则：
- CLI 只是这些 contract 的一个调用者。
- 后续 `Vite` 插件也应尽量复用这些 contract，而不是重新发明一套流程。

## 当前 plugin 状态
- `packages/motif-vite/` 已不只是骨架，当前默认走 TS core adapter
- 插件仍保留 `adapter` 注入层，不把 Rust binding 直接写死在插件内部
- 插件当前已具备：
- 虚拟模块 `virtual:motif.css`
- 源码目录递归扫描
- 文件变更后失效并重新编译样式
- React / Vue Vite demo 的真实接入路径
- 独立 `typecheck` / `test`
- 当前仍缺：
- Rust binding / WASM 与 TS core 的统一策略
- 更细的增量失效策略
- 开发态 HMR 优化与验证

## demo / cases 约定
- 第一层：`native/` `ts/` `react/` `vue/`
- 第二层：场景名，例如 `basic/` `variants/` `theme/` `workspace/`
- `cases/` 放最小扫描输入；`demo/` 放面向人看的最小可用示例
- 当前 demo 目录仍可单独运行 `cargo run -p motif-core -- .` 生成本目录的 `motif.css`
- `variants/` 用于覆盖 `focus:` `hover:` `active:` `dark:` 这类已支持变体
- `theme/` 用于并排展示 Win11 向 `f-` preset 与 Google 向 `m-` preset 的视觉差异
- `workspace/` 用于验证真实页面高频语义：`label`、`divider`、`field`、`action`

## core 结构
- `src/lib.rs`：库入口，暴露扫描、解析、规则、生成、token、输出模块
- `src/main.rs`：当前 Rust CLI 入口
- `src/engine/`：编译 orchestration，聚合 scan / parse / rule / gen
- `src/cli/`：命令参数与流程编排
- `src/scan/`：文件发现、类名提取、文件级扫描来源
- `src/parse/`：DSL 解析
- `src/gen/`：CSS 生成
- `src/token/`：token 载入、schema 与模型
- `src/rule/`：规则映射
- `src/write/`：输出写入
- `src/shared/`：共享类型与错误
- `tests/`：Rust 集成测试，不在源文件内内联

## token 与 rule 约定
- `tokens/fluent.json`：Win11 向 `f-` preset token 集
- `tokens/material.json`：Google 向 `m-` preset token 集
- 当前 preset 只开放少量内建风格家族，不走开放式第三方主题市场
- token 结构应优先服务 preset consistency，而不是追求无限自由配置
- rule 设计应尽量围绕语义 utility，而不是无限扩张成视觉碎片 utility
- 组件语义应优先落在 `f-*` / `m-*`，而尺寸、密度、圆角、文字级别等非风格参数应优先落在 `ui-*`

## 用户入口演进
### 当前入口
- Rust CLI 仍可直接生成 `motif.css`
- `xtask` 负责质量闸门与 demo 构建验证
- `packages/motif-vite` 已可作为真实前端入口，默认走 TS core

### 目标入口
- `npm` 包
- bundler plugin，优先考虑 `Vite`
- 开发态自动扫描与热更新
- 生产态构建时收敛最终 CSS

原则：
- Rust 可以继续存在，但不应成为最终用户必须感知的“后端”。
- 最终接入体验要更像 Tailwind，而不是“前端项目外挂一个 Rust 编译器”。

## plugin 需要的额外能力
下一阶段的 `motif-vite` 至少需要下面这些能力：
- 接收文件变更后的源码内容并走 `scan_sources`
- 保留文件级扫描结果，便于增量失效与后续 preset migration
- 输出内存中的 stylesheet，而不是强依赖写盘
- 将 CSS 注入虚拟模块或 dev server 管线
- 在 watch 模式下避免每次全量重编译整个工程

## 质量闸门结构
- `cargo run -p xtask -- quality`：跨平台硬闸门入口
- `cargo run -p xtask -- demo-builds`：跨平台 demo 构建验证入口
- `node scripts/node/quality.mjs`：当前本地/CI 首选质量入口
- `node scripts/node/demo-builds.mjs`：当前本地/CI 首选 demo 构建入口
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
- 当前 e2e 覆盖 `cases/` 与四类 `demo/` 的 `basic/` / `variants/` / `theme` / `workspace` 场景
- 后续若进入 `npm` / plugin 形态，需要补一层开发态 watch / HMR 验证

## 当前主链路
1. 扫描输入文件
2. 提取静态类名
3. 解析前缀 / 变体 / utility / value
4. 查 token 与规则
5. 生成并写出 `motif.css`
6. 通过 `xtask` 质量闸门压制 AI 生成代码的熵增

## 目标主链路
1. bundler 或 dev server 监听文件变化
2. 增量扫描类名变化
3. 走同一套 parse / rule / token / gen 内核
4. 更新虚拟 CSS 模块或产物文件
5. 开发态热更新，生产态输出最终 CSS
6. 后续支持 preset migration 与更高层工具能力

