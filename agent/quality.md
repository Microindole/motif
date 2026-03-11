# motif Quality

更新时间：2026-03-11

## 目标
`motif` 的质量体系不是为了做一套抽象的大而全规范，而是为了抑制 AI 编程最常见的熵增：
- 文件越来越大
- 目录越来越平
- 同类代码横向复制
- 注释失真
- 黑魔法与临时补丁混入主线
- 文档与实际实现逐步脱节

当前质量体系按三层执行：
- `hard gate`：客观、可自动化、误报低，直接阻止合并
- `soft gate`：自动告警，但先不阻止合并
- `review checklist`：保留给人工 review 的结构判断

## 当前 hard gate
### 基础构建
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test -p motif-core`
- `scripts/check-demo-builds.ps1`

### 结构约束
- `core/src/**/*.rs` 单文件不得超过 `320` 行
- `core/tests/**/*.rs` 单文件不得超过 `420` 行
- `scripts/**/*.ps1` 单文件不得超过 `320` 行
- `agent/**/*.md` 单文件不得超过 `400` 行
- `demo/` `cases/` 下源码 / 配置文件不得超过 `260` 行
- 单目录直接跟踪文件数不得超过 `12`
- 跟踪文件中不得出现构建产物：`target/` `node_modules/` `dist/` `coverage/` `.vite/` `motif.css` `*.tsbuildinfo`

### 危险写法
- 非测试源码中禁止 `unsafe`
- 禁止 `transmute`
- 禁止 `MaybeUninit`
- 禁止 `todo!()` `unimplemented!()` `dbg!()`
- `core/src/` 中除 `main.rs` 外禁止 `unwrap()` / `expect()`
- `core/src/` 中除 `main.rs` 外禁止 `println!()` / `eprintln!()`

### 文档入口一致性
- `agent/context.md` 必须包含并链接当前强制入口文档
- `agent/product.md` `agent/quality.md` `agent/presets.md` `agent/scope.md` `agent/architecture.md` `agent/status.md` `agent/rules.md` 必须存在

## 当前 soft gate
### 注释质量
- 简单源码文件注释密度过高告警
- 长函数或高分支文件完全无注释告警
- 目标是压制“废话注释”和“复杂逻辑无说明”两种极端

### 命名和目录模式
- 同目录下出现大量相同前缀 / 后缀命名时告警
- `helper` `util` `manager` `service` `handler` `processor` `temp` `misc` `final` `old` `new` 这类命名出现过多时告警
- 目的是抓住 AI 横向复制概念而不抽象的趋势

### 单一职责代理指标
当前暂不做硬卡，但 review 时要特别关注：
- 一个文件同时做扫描、解析、IO、格式化、规则映射
- 一个模块同时承担产品策略和底层工具职责
- 一个 demo 页面复制了大量平行结构却没有复用策略

## 当前 review checklist
每个较大改动至少回答下面这些问题：
- 这个文件还能继续拆吗，还是已经是合理边界？
- 这个逻辑是否在别处已有近似实现？
- 这个命名是真语义，还是 AI 生成的占位词？
- 这段注释是在解释约束与 tradeoff，还是在复述代码？
- 这次改动有没有让 `agent/` 文档过期？
- 这次改动有没有把 demo / cases / tests 一起更新？

## GitHub 自动化
当前推荐启用：
- GitHub Actions：跑 `quality` 和 `coverage`
- Dependabot：维护 Cargo、npm、GitHub Actions 依赖
- CodeQL：做 Rust、JavaScript/TypeScript、GitHub Actions 工作流扫描

## GitHub 分支保护建议
建议把 `main` 保护成至少以下规则：
- Require a pull request before merging
- Require approvals：至少 `1`
- Dismiss stale approvals when new commits are pushed
- Require status checks to pass before merging
- Required checks：`quality`、`coverage`、`CodeQL`
- Block force pushes
- Block deletions

## 下一步可升级项
- diff coverage 下限
- 重复代码检测
- 架构边界检查，例如 `rule` 不做 IO、`scan` 不依赖 `gen`
- 依赖膨胀检查
- 变更规模检查
