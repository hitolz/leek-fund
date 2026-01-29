# Quickstart: Fund Demo UI Redesign / 快速开始：基金演示页面重设计

## Prerequisites / 前置条件

- Repository is on branch `009-fund-ui-demo`.
- 仓库当前位于分支 `009-fund-ui-demo`。
- Rust toolchain is available for compile checks.
- 本地具备 Rust 工具链用于编译检查。

## 1. Compile Check (Rust) / 步骤 1：编译检查（Rust）

Run a compile check to ensure the repository still builds:
运行编译检查，确保仓库仍可构建：

```bash
cargo check
```

Working directory:
执行目录：

```bash
/Users/hitol/code/ai/leek-fund/src-tauri
```

## 2. Open the Demo Page / 步骤 2：打开演示页面

Open the single HTML demo page in a browser:
在浏览器中打开单页 HTML 演示页面：

```bash
open /Users/hitol/code/ai/leek-fund/demo/index.html
```

If the demo file is not yet present, complete the implementation tasks first.
如果演示文件尚未创建，请先完成实现任务。

## Demo Scope / 演示范围

- This demo is a single HTML page with embedded sample data only.
- 该演示为单页 HTML，且仅包含内嵌示例数据。
- No live network requests or production persistence paths are used.
- 不会发起实时网络请求，也不会使用生产持久化路径。

## 3. Acceptance Walkthrough / 步骤 3：验收走查

Validate the core flows against the spec:
对照规格验证核心流程：

1. Toggle the list panel visibility and confirm the selected list remains active.
1. 切换列表面板显隐，并确认当前选中列表保持不变。
2. Select a different list and confirm the fund list updates to that list only.
2. 选择其他列表，确认基金列表仅展示该列表内容。
3. Add a fund with a code and name; confirm it appears in the active list.
3. 使用基金代码与名称添加基金，确认其出现在当前列表。
4. Remove a fund and confirm it no longer appears.
4. 删除一只基金并确认其不再显示。
5. Select a fund and confirm details and the trend chart area are visible.
5. 选中一只基金，确认详情与走势图区域可见。
6. Enter holding amount and holding shares; confirm cost per share and daily change amount update correctly.
6. 输入持仓金额与持仓份额，确认每份成本与当日涨跌额正确更新。
7. Apply sorting by daily change percent, daily change amount, and holding amount in descending, ascending, and none states; confirm the order is correct and resets when none is chosen.
7. 分别按当日涨跌幅、当日涨跌额、持仓金额进行降序、升序与不排序操作，确认顺序正确且选择不排序后恢复默认顺序。

## US1 Validation Notes / US1 验证记录

- Panel toggle preserves the active list and fund selection.
- 面板显隐切换不会丢失当前激活列表与基金选择。
- List switching immediately scopes the fund list to the selected list only.
- 列表切换会立即将基金列表限定为所选列表。
- Adding a new fund appends it to the default order and selects it.
- 添加新基金会将其追加到默认顺序末尾，并自动选中。
- Removing the selected fund moves selection to the next available fund or clears it.
- 删除已选基金时会选中下一个可用基金，或在无可用基金时清空选择。

## US2 Validation Notes / US2 验证记录

- Selecting a fund updates the detail header, daily indicators, and the trend chart together.
- 选中基金会同时更新详情头部、当日指标与走势图。
- Trend points render as a continuous inline SVG line without external dependencies.
- 趋势点会以连续的内联 SVG 折线呈现，无需外部依赖。
- Updating holding amount and holding shares immediately updates cost per share and daily change amount.
- 更新持仓金额与持仓份额后，每份成本与当日涨跌额会立即更新。
- When holding shares is 0, the cost per share display shows a neutral placeholder and an explanatory hint.
- 当持仓份额为 0 时，每份成本显示中性占位，并给出提示说明。

## US3 Validation Notes / US3 验证记录

- Each sort field supports tri-state cycling: descending, ascending, and none.
- 每个排序字段都支持三态循环：降序、升序、不排序。
- Sorting uses derived values for daily change amount and holding amount, and remains stable for ties.
- 排序会使用当日涨跌额与持仓金额的推导值，并在相同值时保持稳定。
- Choosing “none” always restores the list’s preserved default order.
- 选择“不排序”会始终恢复列表保留的默认顺序。

## Implementation Notes / 实现说明

- The demo is implemented as a single self-contained page at `/Users/hitol/code/ai/leek-fund/demo/index.html`.
- 演示以单页自包含形式实现，路径为 `/Users/hitol/code/ai/leek-fund/demo/index.html`。
- Sorting, derived values, and default-order restoration are all computed from in-memory sample data.
- 排序、推导值与默认顺序恢复均基于内存中的示例数据计算完成。
