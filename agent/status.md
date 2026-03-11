# motif Status

更新时间：2026-03-11

## 当前阶段
- P0：最小扫描到 CSS 输出闭环、双 preset、四类 demo、token 驱动、e2e、构建级验证，以及 AI 熵增防护的第一轮质量闸门落地

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
- 已实现最小白名单规则映射，并由 token 驱动颜色、间距、圆角、阴影、字体、状态反馈等值
- 已补充 preset 差异规则：`f-surface`、`f-surface-alt`、`f-title`、`f-body`、`f-label`、`f-text-muted`、`f-field`、`f-divider`、`f-action-primary`、`m-surface-variant`、`m-bg-primary-container`、`m-bg-primary`、`m-text-primary`、`m-text-on-primary`、`m-title`、`m-body`、`m-label`、`m-text-muted`、`m-field`、`m-divider`、`m-action-primary`、`m-action-tonal`
- 已实现最小 CSS 生成：支持类名转义、伪类变体、`dark:` 媒体查询包装
- 已实现最小输出写盘：默认在扫描根目录写出 `motif.css`，也支持显式输出路径
- 已按 `framework/scenario` 重组 `cases/` 与 `demo/`：覆盖 `native/` `ts/` `react/` `vue/` 的 `basic/`、`variants/`、`theme/`、`workspace/` 场景
- 已补四类最小 case 输入：原生、TS、React、Vue
- 已补四类最小 demo：原生直接可开；TS / React / Vue 提供最小项目文件与运行说明
- 已补四类 `variants/` 场景：覆盖 `focus:` `hover:` `active:` `dark:`
- 已补四类 `theme/` 场景：并排展示 Win11 向与 Google 向 preset 的视觉差异
- 已补四类 `workspace/` 场景：验证标签、输入、分隔线、主操作、容器层次等真实页面高频语义
- 已将 Rust 测试迁移到 `core/tests/`，不在源文件内内联
- 已补 token 载入测试并验证通过：`cargo test -p motif-core`
- 已补 CLI 端到端测试：覆盖 `cases/` 与四类 `demo/` 的 `basic/` / `variants/` / `theme/` / `workspace/` 场景 CSS 生成结果
- 已补跨平台工程任务：`cargo run -p xtask -- quality` 与 `cargo run -p xtask -- demo-builds`
- 已将 `scripts/check-quality.ps1` 与 `scripts/check-demo-builds.ps1` 收敛为本地包装
- 已补 GitHub Actions：`quality`、`coverage`、`CodeQL`
- 已补 Dependabot 配置与 PR 模板
- 已完成一次 `cargo run -p xtask -- quality` 本地验证并通过`r`n- 已补质量闸门第三轮：依赖膨胀检查与最近提交变更规模检查

## 当前优先级
1. 把质量闸门在 GitHub 仓库设置中真正接成 required checks
2. 继续把两套 preset 做得更像 Win11 与 Google 风格
3. 把 `field` / `action` / `divider` / `label` 继续做细，让真实页面更稳
4. 为 token 数据增加更清晰的 schema 与字段约束

## 当前阻塞
- FIXME: 当前质量闸门已在仓库中落地，并已补上 diff coverage、重复代码软告警、架构边界和复杂度代理检查，但 GitHub 分支保护和 CodeQL / Dependabot 的仓库级启用仍需在仓库设置里打开。
- FIXME: 当前 preset 已能看出较明显差异，但 Win11 向的云母/亚克力层次与 Google 向的完整 container / field / action 系统仍是第一版。

## 当前待办
- TODO: 在 GitHub 仓库设置中启用 required checks、审查要求与分支保护。
- TODO: 继续扩展 `f-` 的 mica / acrylic / border / hover / field 细节。
- TODO: 继续扩展 `m-` 的 container / shape / typography / field / action 层次。
- TODO: 为 token 数据增加更清晰的 schema 与字段约束。
- TODO: 继续把重复代码检测与复杂度代理检查调到低噪音，再决定哪些升级为 hard gate。
- TODO: 继续观察依赖膨胀检查与变更规模检查的噪音，再决定哪些升级为 hard gate。`r`n- TODO: 评估提交规范检查与 diff 级依赖新增审查。


