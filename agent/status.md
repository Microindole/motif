# motif Status

更新时间：2026-03-11

## 当前阶段
- P0：最小扫描、解析、规则映射链路落地

## 当前已完成
- 已建立顶层目录：`agent/` `core/` `web/` `demo/` `cases/` `tests/` `tokens/`
- 已建立最小 Rust workspace：根 `Cargo.toml` + `core/Cargo.toml`
- 已建立最小 CLI 入口：`core/src/main.rs`
- 已恢复 `core/src/` 模块目录：`cli/` `scan/` `parse/` `gen/` `token/` `rule/` `write/` `shared/`
- 已实现最小扫描链路：递归扫描目录、过滤构建产物目录、支持 `.html` `.jsx` `.ts` `.tsx` `.vue` `.svelte`
- 已实现最小 DSL 解析：拆解变体、风格前缀、utility、value，并限制 `hover:` `focus:` `active:` `dark:`
- 已实现最小白名单规则映射：支持 `f-stack` `f-ring` `f-bg-primary` `f-text-primary` `m-surface` `m-elevation-1` `m-shadow-2`
- 已按 `framework/scenario` 重组 `cases/` 与 `demo/`：覆盖 `native/` `ts/` `react/` `vue/` 的 `basic/` 场景骨架
- 已补四类最小 case 输入：原生、TS、React、Vue
- 已补最小扫描 / 解析 / 规则测试并验证通过：`cargo test -p motif-core`
- 已验证最小链路命令可运行：`cargo run -p motif-core -- cases/scan-basic`

## 当前优先级
1. 落地 `gen/` / `write/`
2. 定义 `tokens/` 数据
3. 补四类 `demo/` 实际内容
4. 补 `tests/e2e/`

## 当前阻塞
- FIXME: generator、writer 还未实现。
- FIXME: `tokens/` 目录还没有实际数据文件。
- FIXME: 四类 `demo/` 与 `tests/e2e/` 还没有实际内容。

## 当前待办
- TODO: 建立最小 CSS 输出链路。
- TODO: 为原生、TS、React、Vue 完成最小 demo 页面。
- TODO: 将白名单规则逐步迁移到 token 数据驱动。
