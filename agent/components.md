# motif Components

更新时间：2026-03-16

## 组件目标
`motif` 的终局不是少量按钮和输入框，而是两整套可落完整页面的组件语义系统：
- `f-*`：Fluent 2 / Win11 向
- `m-*`：Material 3 / Google 向
- `ui-*`：与风格无关的参数层，例如尺寸、密度、圆角、间距、文字级别

目标不是复制官方 React 组件 API。
目标是尽量覆盖官方 Web 设计文档里的组件语义与视觉层级，并用受控类名让 AI 和开发者能稳定写出完整页面。

## 覆盖原则
- 以 Fluent 2 Web 和 Material 3 Web 文档为主要对照基线。
- 优先覆盖高频组件和完整页面骨架，而不是先做边缘组件。
- 对“风格”与“参数”拆层：
- `f-*` / `m-*` 决定设计语言
- `ui-*` 决定尺寸、密度、圆角、留白、文字级别
- 对于官方文档中平台差异极大、Web 实现不稳定或仍在演进的部分，先保证语义和视觉方向，不把“100% 一字不差复刻”写成当前硬承诺。

## 状态标记
- `planned`：已进入矩阵，但未开始
- `scaffolded`：已有基础语义或第一版样式
- `in_progress`：正在补风格拟合、状态、参数层或 demo
- `done_v1`：已有双 preset、基础状态、demo、测试
- `parity_gap`：已支持但仍明显不像官方

## P0 组件矩阵
| 组件域 | 组件 | Fluent | Material | 参数层 | 当前状态 | 备注 |
| --- | --- | --- | --- | --- | --- | --- |
| Surface | `surface` | `f-surface` | `m-surface` | `ui-pad-*` `ui-radius-*` | `done_v1` | 已有 theme/workspace 覆盖 |
| Surface | `panel/card` | `f-panel` | `m-surface-container` | `ui-pad-*` `ui-radius-*` | `done_v1` | 仍需更强拟合 |
| Typography | `title/body/label` | `f-title` `f-body` `f-label` | `m-title` `m-body` `m-label` | `ui-text-*` | `done_v1` | 仍需更细级别 |
| Action | `button primary` | `f-action-primary` | `m-action-primary` | `ui-control-*` `ui-radius-*` | `done_v1` | 需扩成完整按钮矩阵 |
| Action | `button secondary` | `f-action-subtle` | `m-action-tonal` | `ui-control-*` `ui-radius-*` | `done_v1` | |
| Action | `button outlined` | `f-action-outlined` | `m-action-outlined` | `ui-control-*` `ui-radius-*` | `done_v1` | Fluent 侧已补第一版安静描边表达 |
| Form | `text field` | `f-field` | `m-field` | `ui-control-*` | `done_v1` | |
| Form | `textarea` | `f-textarea` | `m-textarea` | `ui-control-*` `ui-text-*` | `done_v1` | 已接 workspace、Rust/TS tests |
| Form | `select` | `f-select` | `m-select` | `ui-control-*` | `done_v1` | 已接 workspace、Rust/TS tests |
| Form | `checkbox` | `f-checkbox` | `m-checkbox` | `ui-control-*` | `done_v1` | 已接 variants/workspace、Rust/TS tests |
| Form | `radio` | `f-radio` | `m-radio` | `ui-control-*` | `done_v1` | 已接 variants/workspace、Rust/TS tests |
| Form | `switch` | `f-switch` | `m-switch` | `ui-control-*` | `done_v1` | 已接 variants/workspace、Rust/TS tests |
| Feedback | `divider` | `f-divider` | `m-divider` | - | `done_v1` | |
| Navigation | `tabs` | `f-tab` | `m-tab` | `ui-control-*` | `done_v1` | 已接 workspace、Rust/TS tests |
| Navigation | `drawer/nav` | `f-drawer` `f-nav-item` | `m-drawer` `m-nav-item` | `ui-pad-*` | `done_v1` | 已补壳层与整行导航语义，后续继续做官方拟合 |
| Overlay | `dialog` | `f-dialog` | `m-dialog` | `ui-pad-*` `ui-radius-*` | `done_v1` | 已接 workspace、Rust/TS tests |
| Overlay | `menu/menu-item` | `f-menu-item` | `m-menu-item` | `ui-control-*` | `done_v1` | 已接 workspace、Rust/TS tests |
| Data | `list-item` | `f-list-item` | `m-list-item` | `ui-control-*` `ui-pad-*` | `done_v1` | 已补整行条目壳层，后续继续做风格拟合 |
| Data | `badge/chip/tag` | `f-badge` `f-chip` `f-tag` | `m-badge` `m-chip` `m-tag` | `ui-text-*` `ui-radius-*` | `done_v1` | 已接 workspace、Rust/TS tests |
| Data | `table / row` | `f-table` `f-table-row` `f-table-row-selected` | `m-table` `m-table-row` `m-table-row-selected` | `ui-pad-*` | `done_v1` | 已补完整容器与选中层次，后续继续做表格拟合 |
| Overlay | `sheet / bottom sheet` | `f-sheet` `f-sheet-side` `f-sheet-bottom` | `m-sheet` `m-sheet-side` `m-sheet-bottom` | `ui-pad-*` `ui-radius-*` | `done_v1` | 已补面板拉伸与裁切约束，后续继续做层次拟合 |
| Feedback | `tooltip` | `f-tooltip` | `m-tooltip` | `ui-text-*` | `done_v1` | 已接 workspace、Rust/TS tests |
| Feedback | `banner` | `f-banner` | `m-banner` | `ui-pad-*` | `done_v1` | 已接 workspace、Rust/TS tests |
| Feedback | `toast/snackbar` | `f-toast` | `m-toast` | `ui-pad-*` | `done_v1` | 已补完整浮层壳层，后续继续做角色拟合 |

## P1 组件矩阵
- icon-button
- segmented button
- search field
- breadcrumb
- avatar / persona
- progress / spinner
- skeleton
- accordion
- table / row
- empty state
- sheet / bottom sheet

## P2 组件矩阵
- date picker
- time picker
- command bar / rich toolbar
- tree / tree item
- teaching bubble / coachmark
- advanced data grid states

## 每个组件的完成定义
每个组件至少要同时满足：
- 有 `f-*` 与 `m-*` 两套语义类
- 有 hover / focus / active / selected / disabled 中的必要状态
- 有 `ui-*` 参数层可调尺寸或密度
- 至少一个 `demo/` 场景真实使用
- Rust rule 测试覆盖关键样式
- `motif-vite` 的 TS core 具备对应能力

## 当前实现顺序
1. 把已落第一版的高频组件从 `scaffolded` / `in_progress` 收口到 `done_v1`
2. 按钮矩阵补齐：primary / secondary / outlined / icon，尤其 Fluent 侧对应关系
3. 结构容器拟合：panel / dialog / menu / drawer / sheet
4. 数据与结构拟合：list-item / table / accordion
5. 组合导航与身份拟合：search / segmented / breadcrumb / avatar / persona

## 风格验收维度
### Fluent / Win11
- 背景材料是否偏 mica / acrylic，而不是纯平卡片
- 边界、focus、键盘态是否清楚
- 圆角是否克制
- 主次操作是否安静、桌面化

### Material / Google
- container 分层是否清楚
- primary / tonal / outlined 是否角色明确
- 圆角与颜色角色是否更直接
- 移动优先感与系统化层次是否明显

## 当前结论
当前项目已经进入“组件矩阵驱动阶段”：
- 双 preset 核心方向已稳定
- 参数层第一版已可用
- `motif-vite` 已成为真实前端入口

接下来不应再回到零散加 rule，而应持续按矩阵状态推进：
- 先把高频组件做到 `done_v1`
- 再继续做 Win11 / Google 风格拟合



