# Quickstart: Client UI Redesign From Demo / 快速开始：参照演示的客户端界面重设计

## Prerequisites / 前置条件

- Repository is on branch `010-client-ui-redesign`.
- 仓库当前位于分支 `010-client-ui-redesign`。
- Rust and Node toolchains are available.
- 本地具备 Rust 与 Node 工具链。

## 1. Build & Run / 步骤 1：构建与运行

Run the client in dev mode:
运行客户端开发模式：

```bash
npm run tauri:dev
```

Optional build validation:
可选构建验证：

```bash
npm run tauri:build
```

## 2. Visual Alignment Check / 步骤 2：视觉对齐检查

Open the demo reference for comparison:
打开 demo 参考页面对照：

```bash
open /Users/hitol/code/ai/leek-fund/demo/index.html
```

Confirm the client page uses the same three-column structure, spacing rhythm, and visual state styles as the demo.
确认客户端页面与 demo 采用相同的三列结构、间距节奏与状态样式。

## UI Alignment Checklist / UI 对齐检查清单

- Panel layout matches demo: left list, middle fund list, right detail.  
  面板布局与 demo 一致：左侧列表、中间基金列表、右侧详情。
- Visual states are clear: selected, disabled, up/down/flat.  
  状态明确：选中、禁用、上涨/下跌/持平。
- Controls follow demo hierarchy (primary/ghost buttons, sort controls).  
  控件层级与 demo 一致（主按钮/幽灵按钮、排序控件）。

## 3. Functional Walkthrough / 步骤 3：功能走查

1. Select different lists and verify the fund list updates accordingly.
1. 选择不同列表并验证基金列表正确更新。
2. Select different funds and verify the detail panel updates accordingly.
2. 选择不同基金并验证详情面板正确更新。
3. Add a fund, then remove it, and verify list membership updates correctly.
3. 添加基金后删除，验证列表成员更新正确。
4. Update holding amount and shares and verify derived metrics update.
4. 更新持仓金额与份额，验证推导指标更新。
5. Apply sorting across all fields and ensure tri-state behavior and default order restoration.
5. 对所有排序字段应用三态排序，并确保可恢复默认顺序。
