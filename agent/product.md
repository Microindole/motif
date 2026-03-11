# motif Product

更新时间：2026-03-11

## 产品目标
`motif` 的目标不是再做一个通用 CSS 工具箱，也不是再做一个组件库。

它要解决的是 AI 编程场景下的样式失控问题：
- 直接让 AI 拼组件库，容易被库的 API 和结构绑死。
- 直接让 AI 写 Tailwind，类名会越来越长，风格一致性依赖 prompt 约束。
- 直接让 AI 生成 CSS，最容易出现重复规则、局部补丁和不可维护的样式堆积。

`motif` 要提供的是：
- 一个高约束、可扫描、可编译的原子 CSS 语言。
- 一个面向 AI 的双 preset 样式系统。
- 一个能把重复样式需求收敛成稳定输出 CSS 的生成链路。

## 当前产品定义
当前阶段，`motif` 只做两个内建 preset：
- `f-`：Win11 向 preset
- `m-`：Google 向 preset

用户或 AI 不应该“自己拼出一种风格”，而应该直接选择进入其中一套预设风格。

也就是说：
- `motif` 不是开放式风格平台。
- `motif` 不是 Tailwind 的换皮。
- `motif` 不是组件库运行时。
- `motif` 是一个双 preset 驱动的样式编译器。

## 要解决的核心问题
### 1. 约束 AI 输出空间
AI 不应自由生成任意 CSS。
AI 应优先生成受控的 `f-*` / `m-*` 类名和少量变体。

结果是：
- 更稳定
- 更容易扫描
- 更容易 dedupe
- 更容易做统一风格控制

### 2. 把风格判断前置
如果没有 preset，AI 每次都会在 prompt 里重新猜：
- 圆角该多大
- 阴影该多重
- 主色该多强
- hover / focus 该怎么表现

`motif` 要把这些判断固化到 preset 里。
用户只需要决定：
- 用 `f-`
- 或用 `m-`

### 3. 把最终 CSS 收敛
`motif` 的主链路是：
- 扫描
- 解析
- 规则映射
- token 取值
- 生成 CSS

这意味着：
- 相同类名只会生成一次
- 相同视觉决策不会被 AI 反复写出多份相近 CSS
- 最终产物比“AI 直接输出 CSS 文件”更可控、更稳定、更容易维护

## 与其他方案的区别
### 与组件库的区别
组件库的问题是：
- 结构太重
- API 太强约束
- 定制常常要绕组件抽象层
- AI 很容易被组件树和 props 体系绑住

`motif` 选择：
- 不先提供一大套运行时组件
- 先提供能稳定支撑组件的样式语义层

### 与 Tailwind 的区别
Tailwind 的问题不是不能用，而是：
- 风格本身不是内建的
- 需要大量 utility 组合才能逼近目标视觉
- AI 会不断堆类名，最后虽然可用，但很难保持统一审美

`motif` 选择：
- 减少自由度
- 让相同语义在不同 preset 下自动映射不同视觉
- 让风格一致性来自系统，而不是 prompt 的运气

### 与 AI 直接生成 CSS 的区别
AI 直接生成 CSS 的问题是：
- 重复规则多
- 局部修补多
- 命名漂移严重
- 很难在多页面之间复用

`motif` 选择：
- 只允许 AI 落到有限的语言表面
- 再由编译器统一输出 CSS

## 当前不做的事
- 不做完整组件库
- 不做开放式多 preset 平台
- 不做任意值 DSL
- 不做复杂表达式求值
- 不做“AI 想写什么都能写”的自由样式系统

## 当前正确的迭代方向
### 第一层：完善 preset token
继续补齐：
- typography
- spacing rhythm
- surface layering
- state colors
- motion
- shape rhythm

### 第二层：完善语义 utility
围绕真实页面高频需求补齐：
- text
- surface
- action
- field
- feedback
- layout

### 第三层：用真实页面反推缺口
不是先做一大堆组件，
而是通过 `demo/` 和 `cases/` 的页面场景，反推缺少哪些 utility 和 token。

## 组件策略
现在不应该直接追求“有很多组件”。
当前更合理的做法是：
- 先做组件所需的最小语义样式层
- 再用 demo 页面证明这些语义层足以支撑常见 UI

所以比起先做 `Button` / `Card` 组件，当前更应该优先稳定这些语义：
- surface
- surface-alt
- surface-variant
- title
- body
- label
- primary
- primary-container
- field
- divider
- interactive state

## 成功标准
如果 `motif` 成功，应该出现这些结果：
- AI 写页面时不再频繁输出大段重复 CSS。
- AI 用很少的类名，就能落到明显像 Win11 或 Google 的页面。
- 相同页面换 preset 时，不是“换个蓝色”，而是整体气质发生变化。
- 代码库里的样式产物持续收敛，而不是随着对话轮数不断膨胀。

## 当前阶段的一句话判断
`motif` 现在已经不是一个空骨架了。
它已经具备：
- 双 preset 基础方向
- CSS 生成链路
- demo / cases / e2e / 构建验证

但它还没有进入“足够完整可用于复杂页面”的阶段。
接下来最重要的，不是继续堆基础设施，而是继续打磨两套 preset 的真实体感。
