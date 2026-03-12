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
- 依赖和单次改动无节制膨胀

当前质量体系按三层执行：
- `hard gate`：客观、可自动化、误报低，直接阻止合并
- `soft gate`：自动告警，但先不阻止合并
- `review checklist`：保留给人工 review 的结构判断

## 当前入口
跨平台主入口：
- `cargo run -p xtask -- quality`
- `cargo run -p xtask -- demo-builds`

本地 Windows 便捷入口：
- `scripts/check-quality.ps1`
- `scripts/check-demo-builds.ps1`

原则：
- `cargo xtask` 是主入口
- `ps1` 只是本地包装，不再承载独立逻辑

## 当前 hard gate
### 基础构建
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test -p motif-core`
- `cargo run -p xtask -- demo-builds`

### 覆盖率
- 总体行覆盖率不得低于 `70%`
- PR diff coverage 不得低于 `80%`

### 结构约束
- `core/src/**/*.rs` 单文件不得超过 `320` 行
- `core/tests/**/*.rs` 单文件不得超过 `420` 行
- `xtask/src/**/*.rs` 单文件不得超过 `320` 行
- `scripts/**/*.ps1` 单文件不得超过 `320` 行
- `agent/**/*.md` 单文件不得超过 `400` 行
- `demo/` `cases/` 下源码 / 配置文件不得超过 `260` 行
- 单目录直接跟踪文件数不得超过 `12`
- 跟踪文件中不得出现构建产物：`target/` `node_modules/` `dist/` `coverage/` `.vite/` `motif.css` `*.tsbuildinfo`
- 极端长函数直接失败：单函数不得超过 `90` 行

### 危险写法
- 非测试源码中禁止 `unsafe`
- 禁止 `transmute`
- 禁止 `MaybeUninit`
- 禁止 `todo!()` `unimplemented!()` `dbg!()`
- `core/src/` 中除 `main.rs` 外禁止 `unwrap()` / `expect()`
- `core/src/` 中除 `main.rs` 外禁止 `println!()` / `eprintln!()`

### 架构边界
- `scan` 不得依赖 `rule` `token` `gen` `write`
- `parse` 不得依赖 `scan` `rule` `token` `gen` `write`
- `token` `rule` `gen` 不得做文件 IO
- `write` 不得反向依赖 `scan` `parse` `rule` `token` `gen`

### 依赖膨胀
- `core/Cargo.toml` 直接依赖不得超过 `8`
- `xtask/Cargo.toml` 直接依赖不得超过 `6`
- 单个 demo `package.json` 依赖总数不得超过 `10`
- PR / CI 环境下，`core` 或 `xtask` 新增直接依赖默认直接失败
- PR / CI 环境下，单个 demo 一次新增过多 npm 依赖直接失败

### 变更规模
- PR / CI 环境下，按基线分支 merge-base 计算的变更文件数不得超过 `24`
- PR / CI 环境下，按基线分支 merge-base 计算的新增行数不得超过 `700`
- PR / CI 环境下，按基线分支 merge-base 计算的删除行数不得超过 `500`
- 本地默认按 `HEAD~1..HEAD` 做硬卡，同时对 `origin/main` 的 merge-base 变化量给告警信号

### 提交信息
- `HEAD` 提交标题必须匹配 `type: description` 或 `type(scope): description`
- 允许的 type：`feat` `fix` `refactor` `docs` `test` `build` `ci` `chore` `perf` `style`
- 提交标题不得超过 `72` 个字符
- 提交标题不得以句号结尾

### PR 描述
- 仅在 `pull_request` 事件中硬生效
- PR 描述必须保留模板中的四个分区：`Summary` `Hard checks` `Structure review` `AI-specific review`
- PR 描述中不得保留未完成的模板复选框 `- [ ]`
- PR 的 `Summary` 分区不得只剩模板复选框，必须至少有一行非复选框说明

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

### 重复代码
- 当前对 `core/` `xtask/` `scripts/` 做精确重复块探测
- 只要重复窗口命中当前 diff 触及的文件，就直接失败
- 其余历史重复块继续保留为告警，用于持续降噪

### 单一职责与复杂度代理指标
- 单函数超过 `70` 行告警，超过 `90` 行直接失败
- 单函数参数超过 `5` 个告警
- 单函数嵌套深度超过 `4` 告警
- 单文件函数数超过 `12` 告警
- 当前仍以代理指标为主，review 时要特别关注：
- 一个文件同时做扫描、解析、IO、格式化、规则映射
- 一个模块同时承担产品策略和底层工具职责
- 一个 demo 页面复制了大量平行结构却没有复用策略

### 依赖与变更规模预警
- `core/Cargo.toml` 直接依赖超过 `4` 时告警
- `xtask/Cargo.toml` 直接依赖超过 `3` 时告警
- 单个 demo `package.json` 依赖总数超过 `8` 时告警
- demo 间同名 npm 依赖版本漂移时告警
- 本地对 `origin/main` merge-base 下的依赖新增给出告警信号
- 最近一个提交变更文件数超过 `12` 时告警
- 最近一个提交新增行数超过 `300` 时告警
- 最近一个提交删除行数超过 `200` 时告警
- 非平凡提交没有 body 时告警

## 当前 review checklist
每个较大改动至少回答下面这些问题：
- 这个文件还能继续拆吗，还是已经是合理边界？
- 这个逻辑是否在别处已有近似实现？
- 这个命名是真语义，还是 AI 生成的占位词？
- 这段注释是在解释约束与 tradeoff，还是在复述代码？
- 这次改动有没有顺手引入本不需要的新依赖？
- 这次改动有没有大到应该拆成两个提交或两个 PR？
- 这次改动的提交信息是否能让人快速理解范围和目的？
- PR 描述是否真的完成了模板，而不是把空复选框原样提交？
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
- 将重复代码检测视噪音情况升级为 hard gate
- 继续收紧架构边界，例如限制跨层调用的方向更细
- PR 描述进一步校验“Summary”内容不是空话或模板残留

