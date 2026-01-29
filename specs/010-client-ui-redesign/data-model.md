# Data Model: Client UI Redesign From Demo / 数据模型：参照演示的客户端界面重设计

## Overview / 概览

The redesign reuses existing domain entities but highlights UI-focused state needed to reproduce the demo layout, including selection and panel visibility.
重设计复用现有领域实体，并强调为复现 demo 布局所需的 UI 状态（如选择与面板显隐）。

## Entity: FundList / 实体：基金列表

- Purpose: Active list context for the fund list view.
- 目的：基金列表视图的激活上下文。

Fields / 字段:
- id: Stable identifier for list selection.
- id：用于列表选择的稳定标识。
- name: Display name for the list.
- name：列表显示名称。
- fundOrder: Preserved default order of fund IDs in the list.
- fundOrder：列表内基金 ID 的默认顺序。

Validation Rules / 校验规则:
- name must be non-empty after trimming.
- name 去除空白后必须非空。
- fundOrder must not contain duplicates.
- fundOrder 不得包含重复项。

## Entity: Fund / 实体：基金

- Purpose: Fund summary displayed in the list and detail panel.
- 目的：在列表与详情中展示的基金摘要。

Fields / 字段:
- id: Fund identifier.
- id：基金标识。
- code: 6-digit fund code.
- code：6 位基金代码。
- name: Fund display name.
- name：基金显示名称。
- dailyChangePercent: Daily change percentage.
- dailyChangePercent：当日涨跌幅。
- trendPoints: Time-series data for the detail chart.
- trendPoints：详情走势图的时间序列。

Validation Rules / 校验规则:
- code must be a 6-digit string.
- code 必须为 6 位字符串。
- trendPoints must contain at least two points.
- trendPoints 至少包含两个点。

## Entity: Holding / 实体：持仓

- Purpose: User inputs and derived metrics shown in the detail panel.
- 目的：详情面板展示的用户输入与推导指标。

Fields / 字段:
- holdingAmount: Total holding amount.
- holdingAmount：持仓金额。
- holdingShares: Total holding shares.
- holdingShares：持仓份额。
- costPerShare: Derived from holdingAmount / holdingShares when shares > 0.
- costPerShare：当份额 > 0 时由 holdingAmount / holdingShares 推导。
- dailyChangeAmount: Derived from holdingAmount and dailyChangePercent (percentage).
- dailyChangeAmount：由 holdingAmount 与 dailyChangePercent（百分比）推导。

Validation Rules / 校验规则:
- holdingAmount and holdingShares are non-negative numbers.
- holdingAmount 与 holdingShares 为非负数。

## Entity: SortPreference / 实体：排序偏好

- Purpose: Sorting configuration for the fund list view.
- 目的：基金列表视图的排序配置。

Fields / 字段:
- field: dailyChangePercent | dailyChangeAmount | holdingAmount.
- field：dailyChangePercent | dailyChangeAmount | holdingAmount。
- direction: descending | ascending | none.
- direction：descending | ascending | none。

Ordering Rules / 排序规则:
- When direction is none, ordering falls back to FundList.fundOrder.
- 当 direction 为 none 时，顺序回退到 FundList.fundOrder。
- Sorting is stable for ties using default order positions.
- 相同值时按默认顺序保持稳定性。

## UI State: ViewLayoutState / 界面状态：视图布局状态

- panelVisible: Whether the list panel is shown.
- panelVisible：列表面板是否显示。
- activeListId: Current active list identifier.
- activeListId：当前激活列表标识。
- selectedFundId: Current selected fund identifier.
- selectedFundId：当前选中基金标识。
