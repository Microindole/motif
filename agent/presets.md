# motif Presets

更新时间：2026-03-11

## 当前结论
- 当前只支持两个内建 preset，不做第三套 preset 的平台化抽象。
- `f-` 是 Win11 / Fluent 向 preset。
- `m-` 是 Google / Material 向 preset。
- 目标不是“借用两个名字”，而是让最终视觉表现明显接近这两套官方设计语言的体感。

## 官方依据
### Win11 / Fluent
- Fluent 2 `Material` 明确区分 `solid`、`mica`、`acrylic`、`smoke`；其中 `mica` 与 `acrylic` 都是 Windows 上典型底层材料。
- Windows Mica 文档强调：Mica 是不透明、动态、会吸收桌面壁纸与主题的背景材料，用来建立持久窗口的视觉层级；Mica Alt 比 Mica 有更强的着色。
- Fluent 2 `Shapes` 说明：默认矩形圆角多数是 4px，大型组件用 8px / 12px；不是一味做很大圆角。
- Fluent 2 `Text` 说明：Web 默认基准字体优先使用 `Segoe UI`，并用预设文本层级而不是随意拼字号。
- 官方体验参考：WinUI 3 Gallery、Windows Settings、Fluent 2 官方示例。

### Google / Material
- Material 3 官方将主题拆成 `color scheme`、`typography`、`shapes` 三个核心子系统，并用 `Reply` sample 作为完整示例。
- Material 3 官方强调：`primary` 是主色，用于 prominent buttons、active states、以及 elevated surfaces 的 tint。
- Material 3 官方强调：`onPrimary` 必须用于 `primary` 之上，确保对比度；官方示例明确展示了正确与错误的前景 / 背景搭配。
- Material 3 形状系统默认是不同程度的 rounded rectangles；按钮和卡片都使用较明显但克制的圆角，而不是尖锐矩形。
- Material 3 类型系统强调 display / headline / title / body / label 分层，而不是零散字号。
- 官方体验参考：Reply sample、Material 3 官方示例、Compose Material 3 文档。

## 感知层目标
### `f-`：Win11 / Fluent 向
用户应该先感受到：
- 背景更像 Windows 11 的云母 / 类云母基底，而不是纯平大色块。
- 次级浮层更接近亚克力或轻雾化的感觉，而不是厚重投影。
- 颜色以中性灰白为主，微软蓝作为重点强调，而不是全局强品牌色灌满。
- 圆角偏克制，更多是 4 / 8 / 12 像素节奏，而不是 Material 那种更饱满的圆角。
- focus 态很重要，边界与 ring 要清楚，体现 Windows 的键盘友好性。
- 整体气质更安静、轻、柔和、桌面化。

### `m-`：Google / Material 向
用户应该先感受到：
- 主色、容器色、前景色关系更明确，颜色角色分工强。
- 按钮与卡片更像“干净明确的色块 + 合理圆角 + 清晰层级”。
- 圆角比 Win11 向更饱满，但仍要克制，不能做成夸张的胶囊风。
- elevation 不只是阴影，还包括更明显的 primary tint / container 层次。
- 主按钮应该更“直接”和“明确”，而不是 Win11 的轻雾化克制表达。
- 整体气质更清楚、现代、移动优先、品牌感更强。

## 工程映射
### `f-` 需要优先映射的东西
- `surface`：偏中性浅色 / 深色，允许带轻微云母感或桌面着色感。
- `surface-alt`：用于更像亚克力的浮层与临时表面。
- `text-primary`：偏中性正文色，不应过蓝。
- `bg-primary`：微软蓝强调色，但只用于重点交互。
- `ring`：要明显，作为 Win11 向 preset 的关键辨识点。
- `radius`：默认优先 4 / 8 / 12 的矩形圆角节奏。
- `border`：要比 Material 更重视边界线，而不只靠阴影。

### `m-` 需要优先映射的东西
- `surface`：更干净的容器底色。
- `surface-variant` / `primary-container`：用于形成更清楚的层次与卡片强调。
- `bg-primary`：主按钮核心背景。
- `text-on-primary`：必须和 `bg-primary` 成对出现，保证对比度。
- `text-primary`：品牌主色文本或重点文本。
- `elevation-*`：不仅表现阴影，也要体现 Material 的表面层级逻辑。
- `radius`：比 `f-` 更饱满，但保持系统化，不随意放大。

## 当前实现应遵守的判断标准
如果一个场景在 `f-` 和 `m-` 下切换后：
- 只是颜色换了一下，但气质没变，不合格。
- 只是 token 名不同，但视觉层次没区别，不合格。
- 能让人一眼感觉一边更像 Win11，一边更像 Google，才算合格。

## 当前 `theme/` 场景的用途
- `theme/` 不是普通样例页。
- 它是双 preset 的视觉基准页，用来回答“现在看起来到底像不像”。
- 每次调整 token、shape、state、surface，都应该先看 `theme/` 场景是否把两套风格拉开了。

## 下一阶段重点
1. typography
`f-` 往 `Segoe UI` / Fluent 的层级感收敛，`m-` 往 Material 的 title/body/label 结构收敛。

2. surface layering
`f-` 增强 mica / acrylic 感，`m-` 增强 surface / container / tint 分层。

3. state
把 hover / focus / active / dark 做成两套 preset 各自有辨识度的反馈，而不是共享同一套体感。

4. shape
继续把 `f-` 控制在更克制的圆角体系，把 `m-` 调到更明显但不过度的圆角体系。

## 禁止事项
- 不把 `f-` / `m-` 退化成单纯 token 别名。
- 不把项目做成类似 Tailwind 的开放式风格工具箱。
- 不在当前阶段引入第三套 preset 的目录、注册表或插件系统。
- 不因为“好实现”就把 Win11 向做成纯平卡片，或把 Google 向做成一堆泛蓝色块。
