# motif Status

更新时间：2026-03-11

## 当前阶段
- P0：最小扫描到 CSS 输出闭环、双 preset、四类 demo、token 驱动、e2e、variants、theme 与构建级验证落地

## 当前已完成
- 已建立顶层目录：`agent/` `core/` `web/` `demo/` `cases/` `tests/` `tokens/`
- 已建立最小 Rust workspace：根 `Cargo.toml` + `core/Cargo.toml`
- 已建立库入口与 CLI 入口：`core/src/lib.rs` + `core/src/main.rs`
- 已恢复 `core/src/` 模块目录：`cli/` `scan/` `parse/` `gen/` `token/` `rule/` `write/` `shared/`
- 已实现最小扫描链路：递归扫描目录、过滤构建产物目录、支持 `.html` `.jsx` `.ts` `.tsx` `.vue` `.svelte`
- 已实现最小 DSL 解析：拆解变体、风格前缀、utility、value，并限制 `hover:` `focus:` `active:` `dark:`
- 已明确当前只支持两个内建 preset：`f-` 为 Win11 向，`m-` 为 Google 向
- 已实现最小 token 数据：`tokens/fluent.json` 与 `tokens/material.json`
- 已将 token 扩展到 color、surface、radius、shadow、typography、motion 等维度
- 已实现最小白名单规则映射，并由 token 驱动颜色、间距、圆角、阴影、字体、状态反馈等值
- 已补充 preset 差异规则：`f-surface`、`f-title`、`f-body`、`f-text-muted`、`m-bg-primary`、`m-text-primary`、`m-text-on-primary`、`m-title`、`m-body`、`m-text-muted`
- 已实现最小 CSS 生成：支持类名转义、伪类变体、`dark:` 媒体查询包装
- 已实现最小输出写盘：默认在扫描根目录写出 `motif.css`，也支持显式输出路径
- 已按 `framework/scenario` 重组 `cases/` 与 `demo/`：覆盖 `native/` `ts/` `react/` `vue/` 的 `basic/`、`variants/`、`theme/` 场景
- 已补四类最小 case 输入：原生、TS、React、Vue
- 已补四类最小 demo：原生直接可开；TS / React / Vue 提供最小项目文件与运行说明
- 已补四类 `variants/` 场景：覆盖 `focus:` `hover:` `active:` `dark:`
- 已补四类 `theme/` 场景：并排展示 Win11 向与 Google 向 preset 的视觉差异
- 已将 Rust 测试迁移到 `core/tests/`，不在源文件内内联
- 已补 token 载入测试并验证通过：`cargo test -p motif-core`
- 已补 CLI 端到端测试：覆盖 `cases/` 与四类 `demo/` 的 `basic/` / `variants/` / `theme/` 场景 CSS 生成结果
- 已补 demo 构建检查脚本：`scripts/check-demo-builds.ps1`
- 已完成一次 TS / React / Vue demo 全量构建级验证：`basic/`、`variants/`、`theme/` 均可安装依赖并成功 build

## 当前优先级
1. 继续把两套 preset 做得更像 Win11 与 Google 风格
2. 为 token 数据增加更清晰的 schema 与字段约束
3. 评估是否需要把 demo 构建检查接进固定检查流程

## 当前阻塞
- FIXME: 当前 preset 已有基础差异，但离 Win11 的云母 / 亚克力层次和 Google 的完整 Material 层级感还有距离。

## 当前待办
- TODO: 继续扩展 `f-` 的 mica / acrylic / border / state 细节。
- TODO: 继续扩展 `m-` 的 container / shape / typography 层次。
- TODO: 为 token 数据增加更清晰的 schema 与字段约束。
- TODO: 决定是否将 `scripts/check-demo-builds.ps1` 接入统一检查入口。
