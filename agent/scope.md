# motif Scope

## v0.1 必做
- Rust CLI
- 扫描 `.html` `.jsx` `.ts` `.tsx` `.vue` `.svelte`
- 提取静态可见的 `f-` / `m-` 类名
- 解析最小 DSL
- 生成 `motif.css`
- 最小 token 集
- 最小白名单类集
- 支持 `hover:` `focus:` `active:` `dark:`
- 当前固定提供两个 preset：`f-` 为 Win11 向，`m-` 为 Google 向
- `demo/` 提供原生、TS、React、Vue 四类最小可用示例
- `cases/` 提供原生、TS、React、Vue 四类最小扫描输入
- 当前白名单规则至少由 `tokens/fluent.json` 与 `tokens/material.json` 驱动颜色、间距、圆角、阴影、排版、边框等核心值
- 提供 `theme/` 场景，直观看到两套 preset 的视觉差异
- 提供 `workspace/` 场景，验证标题、标签、分隔线、输入、主操作等真实页面高频语义

## v0.1 非目标
- AST 级复杂表达式求值
- 动态拼接类名分析
- 任意值 DSL
- 复杂变体系统
- 插件 / IDE / WASM / HMR
- 完整 runtime 特效层
- 动态调色算法
- 当前阶段支持第三套及以上 preset
