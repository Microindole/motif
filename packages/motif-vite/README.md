# motif-vite

`motif-vite` 是 `motif` 的最小 Vite 插件骨架。

当前已经内置一版纯 TypeScript 的默认 core adapter，目标是尽早验证前端接入体验：
- 提供虚拟 CSS 模块 `virtual:motif.css`
- 扫描前端源码文件
- 走 preset-first 的最小 scan / parse / rule / gen 链路
- 在文件变更时失效并刷新样式

## 当前限制
- 默认 adapter 还只是对 Rust 原型能力的 TS 复刻，不是最终版内核
- 目前仍是全量扫描，尚未做更细的增量失效
- 后续可以把 adapter 换成：
  - Rust Node binding
  - WASM binding
  - 更完整的 TypeScript core

## 最小接法
```ts
import { defineConfig } from 'vite';
import { motifVite } from 'motif-vite';

export default defineConfig({
  plugins: [motifVite()],
});
```

然后在应用入口里导入：
```ts
import 'virtual:motif.css';
```

## 自定义 adapter
如果后续你想切换到 Rust binding 或实验版 adapter，也可以显式传入：
```ts
import { defineConfig } from 'vite';
import { motifVite } from 'motif-vite';

export default defineConfig({
  plugins: [
    motifVite({
      adapter: {
        async compileSources(sources) {
          return {
            stylesheet: '/* custom motif build */\n',
          };
        },
      },
    }),
  ],
});
```
