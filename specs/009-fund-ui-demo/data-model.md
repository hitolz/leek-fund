# Data Model: Fund Demo UI Redesign / 数据模型：基金演示页面重设计

## Overview / 概览

The demo models four core entities: Fund List, Fund, Holding, and Sort Preference. The model is designed for deterministic rendering and acceptance testing rather than persistence.
演示包含四个核心实体：基金列表、基金、持仓、排序偏好。该模型服务于确定性渲染与验收测试，而非持久化。

## Entity: FundList / 实体：基金列表

- Purpose: Defines the active context for the visible fund set.
- 目的：定义当前可见基金集合的激活上下文。

Fields / 字段:
- id: Stable list identifier for selection and mapping.
- id：用于选择与映射的稳定列表标识。
- name: User-visible list name.
- name：用户可见的列表名称。
- isPanelVisible: Whether the list panel is currently visible.
- isPanelVisible：列表面板当前是否可见。
- fundOrder: Ordered sequence of fund ids representing the default order.
- fundOrder：表示默认顺序的基金 id 有序序列。

Validation Rules / 校验规则:
- name must be non-empty after trimming.
- name 去除首尾空白后必须非空。
- fundOrder must not contain duplicate fund ids.
- fundOrder 不得包含重复的基金 id。

State Notes / 状态说明:
- Active list selection is exclusive: exactly one list is active at a time when lists exist.
- 激活列表选择具有排他性：当存在列表时任意时刻只能有一个激活列表。

## Entity: Fund / 实体：基金

- Purpose: Represents a tracked fund within a specific list context.
- 目的：表示某个列表上下文中的被跟踪基金。

Fields / 字段:
- id: Stable fund identifier within the demo dataset.
- id：演示数据集内的稳定基金标识。
- code: Fund code shown to the user.
- code：展示给用户的基金代码。
- name: Fund display name.
- name：基金显示名称。
- dailyChangePercent: Daily percentage change used for sorting and calculations.
- dailyChangePercent：用于排序与计算的当日涨跌幅百分比。
- trendPoints: Ordered trend values used to render the chart.
- trendPoints：用于绘制走势图的有序趋势点。
- defaultIndex: The fund's position in the list's default order.
- defaultIndex：该基金在列表默认顺序中的位置索引。

Validation Rules / 校验规则:
- code must be present and treated as a unique key within a list in the demo.
- code 必须存在，并在演示中被视为列表内唯一键。
- trendPoints must contain at least two points to render a line.
- trendPoints 至少包含两个点以绘制折线。

State Transitions / 状态迁移:
- add: Fund is appended to fundOrder and assigned a defaultIndex at the end.
- add：基金追加到 fundOrder，并在末尾获得 defaultIndex。
- remove: Fund is removed from fundOrder; selection moves to the next available fund or clears.
- remove：基金从 fundOrder 中移除；选中状态移动到下一个可用基金或被清空。

## Entity: Holding / 实体：持仓

- Purpose: Captures user-provided position inputs and derived metrics for the selected fund.
- 目的：记录用户输入的持仓信息及选中基金的推导指标。

Fields / 字段:
- fundId: The fund this holding belongs to.
- fundId：该持仓所属的基金 id。
- holdingAmount: Total monetary amount invested.
- holdingAmount：投入的总金额。
- holdingShares: Total shares held.
- holdingShares：持有的总份额。
- costPerShare: Derived value when holdingShares is greater than zero.
- costPerShare：当 holdingShares 大于 0 时的推导值。
- dailyChangeAmount: Derived daily change amount based on holdingAmount and dailyChangePercent.
- dailyChangeAmount：基于 holdingAmount 与 dailyChangePercent 的推导当日涨跌额。

Validation Rules / 校验规则:
- holdingAmount must be a non-negative number.
- holdingAmount 必须是非负数。
- holdingShares must be a non-negative number.
- holdingShares 必须是非负数。
- costPerShare is only defined when holdingShares is greater than zero.
- 仅当 holdingShares 大于 0 时 costPerShare 才有定义。

Derivations / 推导规则:
- costPerShare equals holdingAmount divided by holdingShares when holdingShares is greater than zero.
- 当 holdingShares 大于 0 时，costPerShare 等于 holdingAmount 除以 holdingShares。
- dailyChangeAmount equals holdingAmount multiplied by dailyChangePercent expressed as a percentage (dailyChangePercent ÷ 100).
- 当 dailyChangePercent 以百分比表达时，dailyChangeAmount 等于 holdingAmount ×（dailyChangePercent ÷ 100）。

## Entity: SortPreference / 实体：排序偏好

- Purpose: Describes how the current fund list view is ordered.
- 目的：描述当前基金列表视图的排序方式。

Fields / 字段:
- field: One of dailyChangePercent, dailyChangeAmount, or holdingAmount.
- field：dailyChangePercent、dailyChangeAmount 或 holdingAmount 之一。
- direction: One of descending, ascending, or none.
- direction：descending、ascending 或 none 之一。
- isActive: Whether sorting is currently applied.
- isActive：当前是否应用排序。

Validation Rules / 校验规则:
- field must be one of the supported fields.
- field 必须是支持的字段之一。
- direction must be one of the supported directions.
- direction 必须是支持的方向之一。

Ordering Rules / 排序规则:
- When direction is none, ordering falls back to FundList.fundOrder.
- 当 direction 为 none 时，排序回退到 FundList.fundOrder。
- Sorting must be stable for ties by defaultIndex.
- 对于相同值，排序必须按 defaultIndex 保持稳定性。
