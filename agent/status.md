# motif Status

更新时间：2026-03-16

## 当前阶段
- P0：最小扫描到 CSS 输出闭环、双 preset、四类 demo、token 驱动、e2e、构建级验证，以及 AI 熵增防护的多轮质量闸门落地
- P1：前端工具链入口与完整组件矩阵并行推进；`motif-vite` 已可直接接入 Vite demo，当前主线转为“组件收口 + 风格拟合 + 参数层”

## 当前已完成
- 已建立顶层目录：`agent/` `core/` `web/` `demo/` `cases/` `tests/` `tokens/`
- 已建立最小 Rust workspace：根 `Cargo.toml` + `core/Cargo.toml` + `xtask/Cargo.toml`
- 已建立库入口与 CLI 入口：`core/src/lib.rs` + `core/src/main.rs`
- 已建立跨平台工程入口：`xtask/`
- 已恢复 `core/src/` 模块目录：`cli/` `scan/` `parse/` `gen/` `token/` `rule/` `write/` `shared/`
- 已实现最小扫描链路：递归扫描目录、过滤构建产物目录、支持 `.html` `.jsx` `.ts` `.tsx` `.vue` `.svelte`
- 已实现最小 DSL 解析：拆解变体、风格前缀、utility、value，并限制 `hover:` `focus:` `active:` `dark:`
- 已明确当前只支持两个内建 preset：`f-` 为 Win11 向，`m-` 为 Google 向
- 已建立产品入口文档：`agent/product.md`
- 已建立质量纲领文档：`agent/quality.md`
- 已实现最小 token 数据：`tokens/fluent.json` 与 `tokens/material.json`
- 已将 token 扩展到 color、surface、radius、shadow、typography、motion、border、space 等维度
- 已为 token 增加 `effect` 分组、`serde` 字段约束与必填 key 校验，避免 preset 数据悄悄漂移
- 已实现最小白名单规则映射，并由 token 驱动颜色、间距、圆角、阴影、字体、状态反馈等值
- 已补充 preset 差异规则：`f-surface`、`f-surface-alt`、`f-panel`、`f-title`、`f-body`、`f-label`、`f-text-muted`、`f-field`、`f-divider`、`f-action-primary`、`f-action-subtle`、`m-surface-variant`、`m-surface-container`、`m-bg-primary-container`、`m-bg-primary`、`m-text-primary`、`m-text-on-primary`、`m-title`、`m-body`、`m-label`、`m-text-muted`、`m-field`、`m-divider`、`m-action-primary`、`m-action-tonal`、`m-action-outlined`
- 已继续细化两套 preset：为 `surface` / `surface-alt` / `panel` / `field` / `action` 增加 tint、transition、caret-color 与更明确的层次反馈
- 已抽出 `engine::compile_root` 与 `engine::compile_sources` 两个编译入口，开始把 CLI 从核心 contract 上层剥离
- 已为扫描结果保留文件级来源，并补上内存源码扫描入口，为未来 watch / plugin / migration 铺路
- 已新增 `packages/motif-vite/` 最小骨架：提供 `virtual:motif.css`、目录扫描、adapter 注入与文件变更失效流程
- 已让 `motif-vite` 在开发态源码变更后主动触发样式刷新，先保证 watch / 热修改链路可感知
- 已让 `motif-vite` 默认走 TS core adapter，Rust core 继续作为稳定 contract 与对照实现
- 已为 `motif-vite` 补上独立 `test` / `typecheck`，并接入 `xtask -- quality`
- 已将 `motif-vite` 的 shared helper 与 tests 按目录下沉拆分，避免 `shared.ts` / `core.test.ts` 继续膨胀成上帝文件
- 已引入第一版与 preset 解耦的 `ui-*` 参数层，用于尺寸、圆角、文字大小等非风格参数调节
- 已建立组件矩阵文档 `agent/components.md`，开始将主线从零散 rule 补齐切到按组件域推进
- 已实现最小 CSS 生成：支持类名转义、伪类变体、`dark:` 媒体查询包装
- 已实现最小输出写盘：默认在扫描根目录写出 `motif.css`，也支持显式输出路径
- 已按 `framework/scenario` 重组 `cases/` 与 `demo/`：覆盖 `native/` `ts/` `react/` `vue/` 的 `basic/`、`variants/`、`theme/`、`workspace/` 场景
- 已补四类最小 case 输入：原生、TS、React、Vue
- 已补四类最小 demo：原生直接可开；TS / React / Vue 提供最小项目文件与运行说明
- 已补四类 `variants/` 场景：覆盖 `focus:` `hover:` `active:` `dark:`
- 已补四类 `theme/` 场景：并排展示 Win11 向与 Google 向 preset 的视觉差异
- 已补四类 `workspace/` 场景：验证标签、输入、分隔线、主操作、次级操作、面板 / 容器层次等真实页面高频语义
- 已将 Rust 测试迁移到 `core/tests/`，不在源文件内内联
- 已补 token 载入测试与 schema 失效测试并验证通过：`cargo test -p motif-core`
- 已补 CLI 端到端测试：覆盖 `cases/` 与四类 `demo/` 的 `basic/` / `variants/` / `theme` / `workspace` 场景 CSS 生成结果
- 已补 in-memory 编译测试，验证 plugin-like 入口可直接复用核心链路
- 已补跨平台工程任务：`cargo run -p xtask -- quality` 与 `cargo run -p xtask -- demo-builds`
- 已将 `scripts/unix/check-quality.sh` / `scripts/win/check-quality.ps1` 与 `scripts/unix/check-demo-builds.sh` / `scripts/win/check-demo-builds.ps1` 收敛为本地包装
- 已补 GitHub Actions：`quality`、`coverage`、`CodeQL`
- 已补 Dependabot 配置与 PR 模板
- 已完成质量闸门第一轮：文件大小、目录扁平度、危险写法、文档入口一致性
- 已完成质量闸门第二轮：diff coverage、重复代码软告警、架构边界、复杂度代理检查
- 已完成质量闸门第三轮：依赖膨胀检查与变更规模检查
- 已完成质量闸门第四轮：提交信息检查与 PR 基线优先的变更规模计算
- 已完成质量闸门第五轮：diff 级依赖新增审查
- 已完成质量闸门第六轮：PR 描述模板检查
- 已完成质量闸门第七轮：PR Summary 实质内容检查，以及“重复块命中当前变更文件”时的 hard gate
- 已完成质量闸门第八轮：soft warning 严重级别前缀（`[info]` / `[warn]` / `[candidate]`）与 PR Summary 空话检测
- 已完成多次 `cargo run -p xtask -- quality` 本地验证并通过
- 已完成 `cargo run -p xtask -- demo-builds` 全量验证并通过
- 已把产品文档主线更新为：最终接入体验要像 Tailwind 一样直接，但产品能力继续坚持少量 preset 与强约束语义，而不是走 Tailwind 式大全量 utility 路线
- 已将 React / Vue 的 Vite demo 接到 `motif-vite`，改为通过 `virtual:motif.css` 注入样式
- 已开始补第一批完整组件语义：`checkbox` `radio` `switch`
- 已将 `checkbox` `radio` `switch` 接入 variants 场景与回归断言，开始从“零散 utility”转向真实组件矩阵
- 已补第二批组件语义：`textarea` `select` `tab` `dialog` `list-item` `menu-item`，并接入 workspace 场景
- 已补第三批组件语义：`icon-button` `nav-item` `badge` `tooltip` `banner`，并接入 workspace 场景
- 已补第四批组件语义：`drawer` `toast` `segmented-button` `search-field` `breadcrumb-item` `avatar`，并接入 workspace 场景
- 已补第五批组件语义：`progress` `spinner` `skeleton` `empty-state` `sheet`，并为生成器补上 `motif-spin` / `motif-shimmer` 关键帧
- 已补第六批组件语义：`accordion-item` `table-row`，开始覆盖结构化数据与折叠信息块
- 已补第七批组合件语义：`breadcrumb` `persona`
- 已补第八批状态语义：`accordion-item-open` `table-row-selected`
- 已补第九批展示/容器语义：`chip` `sheet-side` `sheet-bottom`
- 已补第十批数据组合语义：`table` `tag`
- 已补第十一批结构语义：`table-header` `accordion-header`
- 已补第十二批结构语义：`table-cell` `accordion-content`
- 已补 Fluent 侧 `f-action-outlined`，并接入 workspace 场景、Rust 规则断言、e2e 断言与 `motif-vite` TS core 测试
- 已继续收紧 `drawer/nav` 的壳层语义：侧栏改为填满容器宽度，导航项补上完整行宽与更明确的壳层表现，Rust / TS core 规则继续保持对齐
- 已继续收紧 `list-item` 的条目壳层：列表项改为完整行宽并裁切内部内容，Rust / TS core 与 workspace 规则断言继续保持对齐
- 已继续收紧 `sheet / toast` 的壳层语义：面板补齐拉伸与裁切约束，toast 补上完整浮层壳层与 preset 差异背景，Rust / TS core 规则继续保持对齐
- 已继续收紧 `table / row` 的数据壳层：表格容器与选中行补上完整宽度、裁切和层次反馈，Rust / TS core 规则继续保持对齐
- 已完成 Rust rule、`motif-vite` shared、`core/tests/e2e` 与 `core/tests/rule` 的结构拆分，当前代码侧 duplicate hard gate 已清空

## 当前优先级
1. 按 `agent/components.md` 把 P0 中仍为 `scaffolded` / `in_progress` 的高频组件收口到 `done_v1`
2. 继续做双 preset 风格拟合，优先磨 `surface/panel/field/action/dialog/drawer/list-item`
3. 扩展 `ui-*` 参数层，从尺寸/圆角继续补到密度、间距和组件级参数
4. 保持 `motif-core` contract 稳定，让 Rust core 与 `motif-vite` TS core 持续对齐
5. 在 watch 已可用的基础上，再评估更细的 CSS update / HMR 粒度
6. 继续降低 soft warning 噪音，并把质量闸门维持在低噪音状态

## 当前阻塞
- FIXME: 当前质量闸门已在仓库中落地，并已补上 diff coverage、重复代码检查、架构边界、复杂度代理、依赖膨胀、变更规模与提交信息检查，但 GitHub 分支保护和 CodeQL / Dependabot 的仓库级启用仍需在仓库设置里打开。
- FIXME: 当前 preset 已能看出较明显差异，并已补到 panel / container / subtle / outlined action，但 Win11 向的云母/亚克力层次与 Google 向的完整 container / field / action 系统仍是第一版。
- FIXME: 当前 `motif-vite` 已走 TS core，但 Rust core 与 TS core 的规则覆盖仍需继续压齐。

## 当前待办
- TODO: 在 GitHub 仓库设置中启用 required checks、审查要求与分支保护。
- TODO: 继续把 `motif-core` contract 压稳，避免 plugin 接入时再次牵动 rule / token 主体。
- TODO: 继续补齐 TS core 与 Rust core 的规则覆盖差距。
- TODO: 评估开发态 watch / 热更新的更细失效策略。
- TODO: 按 `agent/components.md` 推进 P0 组件矩阵，并为每个组件补齐 demo / tests / 参数层。
- TODO: 继续扩展 `f-` 的 mica / acrylic / border / hover / panel / subtle action 细节。
- TODO: 继续扩展 `m-` 的 container / shape / typography / field / outlined action 层次。
- TODO: 将 token schema 从“必填 key 校验”继续升级到更细的字段语义约束。
- TODO: 为未来文件级或仓库级 preset migration 预留扫描与映射结构。
- TODO: 继续把重复代码检测、复杂度代理检查、依赖膨胀检查与变更规模检查调到低噪音，再决定哪些升级为 hard gate。
- TODO: 评估 PR 描述检查与更细的依赖风险分级。




