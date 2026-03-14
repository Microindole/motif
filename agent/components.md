# motif Components

更新时间：2026-03-14

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
| Action | `button outlined` | `planned` | `m-action-outlined` | `ui-control-*` `ui-radius-*` | `in_progress` | Fluent 侧仍缺对应表达 |
| Form | `text field` | `f-field` | `m-field` | `ui-control-*` | `done_v1` | |
| Form | `textarea` | `f-textarea` | `m-textarea` | `ui-control-*` `ui-text-*` | `scaffolded` | 第一版已接 workspace |
| Form | `select` | `f-select` | `m-select` | `ui-control-*` | `scaffolded` | 第一版已接 workspace |
| Form | `checkbox` | `f-checkbox` | `m-checkbox` | `ui-control-*` | `scaffolded` | 第一版已接 variants/workspace |
| Form | `radio` | `f-radio` | `m-radio` | `ui-control-*` | `scaffolded` | 第一版已接 variants/workspace |
| Form | `switch` | `f-switch` | `m-switch` | `ui-control-*` | `scaffolded` | 第一版已接 variants/workspace |
| Feedback | `divider` | `f-divider` | `m-divider` | - | `done_v1` | |
| Navigation | `tabs` | `f-tab` | `m-tab` | `ui-control-*` | `scaffolded` | 第一版已接 workspace |
| Navigation | `drawer/nav` | `f-drawer` `f-nav-item` | `m-drawer` `m-nav-item` | `ui-pad-*` | `scaffolded` | 第一版已接 workspace |
| Overlay | `dialog` | `f-dialog` | `m-dialog` | `ui-pad-*` `ui-radius-*` | `scaffolded` | 第一版已接 workspace |
| Overlay | `menu/menu-item` | `f-menu-item` | `m-menu-item` | `ui-control-*` | `scaffolded` | 第一版已接 workspace |
| Data | `list-item` | `f-list-item` | `m-list-item` | `ui-control-*` `ui-pad-*` | `scaffolded` | 第一版已接 workspace |
| Data | `badge/chip` | `f-badge` | `m-badge` | `ui-text-*` `ui-radius-*` | `scaffolded` | 第一版已接 workspace |
| Feedback | `tooltip` | `f-tooltip` | `m-tooltip` | `ui-text-*` | `scaffolded` | 第一版已接 workspace |
| Feedback | `banner` | `f-banner` | `m-banner` | `ui-pad-*` | `scaffolded` | 第一版已接 workspace |
| Feedback | `toast/snackbar` | `f-toast` | `m-toast` | `ui-pad-*` | `scaffolded` | 第一版已接 workspace |

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
1. 表单控件补齐：`checkbox` `radio` `switch` `select` `textarea`
2. 按钮矩阵补齐：primary / secondary / outlined / icon
3. 结构容器补齐：card / section / dialog / menu
4. 导航补齐：tabs / nav-item / drawer
5. 数据展示补齐：list-item / badge / toast / tooltip
6. 应用壳层补齐：search / segmented / breadcrumb / avatar / persona / toast / drawer
7. 反馈与加载补齐：progress / spinner / skeleton / empty state / sheet
8. 结构化数据补齐：accordion / table-row
9. 组合导航与身份补齐：breadcrumb / persona

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
当前项目已经具备：
- 双 preset 核心方向
- 参数层第一版
- Vite 插件入口

但离“完整组件系统”还差一个组件矩阵驱动阶段。
从本文件开始，后续推进应按组件域和矩阵状态前进，而不是继续零散加 rule。
