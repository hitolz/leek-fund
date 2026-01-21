# Research / 调研

## Decision 1: Three-Column Master-Detail Layout / 决策 1：三列主从布局

**Decision**: Use a persistent three-column layout: lists (left), funds (middle), details (right).  
**决策**：采用固定三列布局：左侧列表，中间基金列表，右侧基金详情。

**Rationale**: This pattern minimizes navigation steps and supports quick scanning and comparison while preserving context.  
**理由**：该模式减少跳转步骤，便于快速扫描对比，并保留上下文。

**Alternatives considered**: Two-column layout with modal details; single-column drill-down.  
**备选方案**：两列布局 + 弹窗详情；单列逐级进入。

## Decision 2: Selection State and Empty States / 决策 2：选择状态与空态

**Decision**: Always show a clear selection state for list and fund; display explicit empty states when no lists, no funds, or no trend data.  
**决策**：始终展示列表与基金的选中态；在无列表、无基金、无走势数据时显示明确空态。

**Rationale**: Reduces user confusion and avoids blank panels.  
**理由**：降低用户困惑，避免界面空白。

**Alternatives considered**: Hide panels entirely or show placeholders without guidance.  
**备选方案**：隐藏面板或仅展示无引导的占位符。

## Decision 3: Daily Change Display / 决策 3：当日涨跌展示

**Decision**: Display the latest available trading day’s change next to each fund code and name.  
**决策**：在基金代码与名称旁展示最新交易日的涨跌信息。

**Rationale**: Enables at-a-glance comparison across funds without entering detail view.  
**理由**：无需进入详情即可对比基金表现。

**Alternatives considered**: Only show change inside the detail panel; show percentage only without sign.  
**备选方案**：仅在详情页展示；仅展示百分比不显示正负。

## Decision 4: Trend Window / 决策 4：走势窗口

**Decision**: Default trend view to a recent fixed window (e.g., last 30 trading days) and allow a no-data message when unavailable.  
**决策**：默认展示固定最近窗口（例如最近 30 个交易日），无数据时显示提示。

**Rationale**: Keeps the chart legible and consistent across funds.  
**理由**：保持图表可读性与一致性。

**Alternatives considered**: Full history chart; user-configurable date range.  
**备选方案**：全量历史曲线；用户自定义时间范围。
