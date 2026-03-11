# motif Status

更新时间：2026-03-11

## 当前阶段
- P0：最小扫描到 CSS 输出闭环与四类 demo 落地

## 当前已完成
- 已建立顶层目录：`agent/` `core/` `web/` `demo/` `cases/` `tests/` `tokens/`
- 已建立最小 Rust workspace：根 `Cargo.toml` + `core/Cargo.toml`
- 已建立库入口与 CLI 入口：`core/src/lib.rs` + `core/src/main.rs`
- 已恢复 `core/src/` 模块目录：`cli/` `scan/` `parse/` `gen/` `token/` `rule/` `write/` `shared/`
- 已实现最小扫描链路：递归扫描目录、过滤构建产物目录、支持 `.html` `.jsx` `.ts` `.tsx` `.vue` `.svelte`
- 已实现最小 DSL 解析：拆解变体、风格前缀、utility、value，并限制 `hover:` `focus:` `active:` `dark:`
- 已实现最小白名单规则映射：支持 `f-stack` `f-ring` `f-bg-primary` `f-text-primary` `m-surface` `m-elevation-1` `m-shadow-2`
- 已实现最小 CSS 生成：支持类名转义、伪类变体、`dark:` 媒体查询包装
- 已实现最小输出写盘：默认在扫描根目录写出 `motif.css`，也支持显式输出路径
- 已按 `framework/scenario` 重组 `cases/` 与 `demo/`：覆盖 `native/` `ts/` `react/` `vue/` 的 `basic/` 场景
- 已补四类最小 case 输入：原生、TS、React、Vue
- 已补四类最小 demo：原生直接可开；TS / React / Vue 提供最小项目文件与运行说明
- 已将 Rust 测试迁移到 `core/tests/`，不在源文件内内联
- 已验证最小闭环命令可运行：`cargo test -p motif-core`
- 已验证四类 demo 可各自生成 `motif.css`

## 当前优先级
1. 定义 `tokens/` 数据
2. 补 `tests/e2e/`
3. 将规则逐步迁移到 token 数据驱动

## 当前阻塞
- FIXME: `tokens/` 目录还没有实际数据文件。
- FIXME: `tests/e2e/` 还没有实际内容。
- FIXME: React / Vue demo 依赖未安装，当前只验证了扫描与 CSS 生成，未验证 dev server 运行。

## 当前待办
- TODO: 增加端到端断言，覆盖 `motif.css` 生成结果。
- TODO: 将白名单规则逐步迁移到 token 数据驱动。
- TODO: 为 demo 增加更完整的场景，例如 `variants/` 与 `theme/`。
