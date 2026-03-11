# motif Architecture

## 顶层结构
- `core/`：CLI 与核心生成链路
- `web/`：前端演示或交互界面
- `demo/`：原生 / TS / React / Vue 示例
- `cases/`：原生 / TS / React / Vue 测试输入
- `tests/`：e2e 与回归断言
- `tokens/`：Fluent / Material token 文件

## core 结构
- `src/main.rs`：CLI 入口
- `src/cli/`：命令参数与流程编排
- `src/scan/`：文件发现与类名提取
- `src/parse/`：DSL 解析
- `src/gen/`：CSS 生成
- `src/token/`：token 载入与模型
- `src/rule/`：规则映射
- `src/write/`：输出写入
- `src/shared/`：共享类型与错误

## 主链路
1. 扫描输入文件
2. 提取静态类名
3. 解析前缀 / 变体 / utility / value
4. 查 token 与规则
5. 生成并写出 `motif.css`