# motif Scope

## v0.1 必做
- Rust CLI
- 扫描 `.html` `.jsx` `.tsx` `.vue` `.svelte`
- 提取静态可见的 `f-` / `m-` 类名
- 解析最小 DSL
- 生成 `motif.css`
- 最小 token 集
- 最小白名单类集
- 支持 `hover:` `focus:` `active:` `dark:`

## v0.1 非目标
- AST 级复杂表达式求值
- 动态拼接类名分析
- 任意值 DSL
- 复杂变体系统
- 插件 / IDE / WASM / HMR
- 完整 runtime 特效层
- 动态调色算法