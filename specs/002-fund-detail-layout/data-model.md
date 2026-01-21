# Data Model / 数据模型

## Overview / 概览

This model defines the list selection, fund summary, and fund detail data needed for the three-column layout.  
本模型定义三列布局所需的列表选择、基金摘要与基金详情数据。

## Entities / 实体

### Fund List / 基金列表

**Description**: A user-defined collection of funds.  
**说明**：用户自定义的基金集合。

**Fields**:  
**字段**：

- `id`: Stable identifier for the list. / 列表稳定标识。
- `name`: Display name for the list. / 列表显示名称。
- `fund_codes`: Ordered set of fund codes. / 有序基金代码集合。

**Validation Rules**:  
**校验规则**：

- `name` is required and non-empty. / `name` 必填且非空。
- `fund_codes` contains unique 6-digit numeric codes. / `fund_codes` 需唯一且为 6 位数字。

### Fund Summary / 基金摘要

**Description**: Minimal data shown in the middle column list.  
**说明**：中间列表中显示的最小信息。

**Fields**:  
**字段**：

- `code`: 6-digit fund code. / 6 位基金代码。
- `name`: Fund name. / 基金名称。
- `daily_change`: Latest trading day change value. / 最新交易日涨跌值。
- `daily_change_percent`: Latest trading day change percent. / 最新交易日涨跌幅。
- `as_of`: Timestamp for the latest change. / 最新涨跌时间戳。

**Validation Rules**:  
**校验规则**：

- `code` must match `^[0-9]{6}$`. / `code` 必须匹配 `^[0-9]{6}$`。
- `daily_change` can be positive, negative, or zero; may be absent if data missing. / `daily_change` 可正可负可为 0；缺失时允许为空。

### Fund Detail / 基金详情

**Description**: Detailed information shown in the right column.  
**说明**：右侧详情面板显示的信息。

**Fields**:  
**字段**：

- `code`: 6-digit fund code. / 6 位基金代码。
- `name`: Fund name. / 基金名称。
- `net_value`: Latest net value. / 最新净值。
- `net_value_date`: Date of latest net value. / 最新净值日期。
- `daily_change`: Latest trading day change value. / 最新交易日涨跌值。
- `daily_change_percent`: Latest trading day change percent. / 最新交易日涨跌幅。
- `as_of`: Timestamp for latest data. / 最新数据时间戳。

### Fund Trend / 基金走势

**Description**: Time series used for the trend chart.  
**说明**：用于绘制走势图的时间序列。

**Fields**:  
**字段**：

- `code`: 6-digit fund code. / 6 位基金代码。
- `points`: List of `(date, value)` pairs ordered by date. / 按日期排序的 `(日期, 数值)` 点列表。
- `window`: The time window covered (e.g., last 30 trading days). / 覆盖的时间窗口（如最近 30 个交易日）。

**Validation Rules**:  
**校验规则**：

- `points` must be ordered by date ascending. / `points` 必须按日期升序。
- Empty `points` allowed when no trend data. / 无走势数据时允许为空。

## Relationships / 关系

- A **Fund List** contains zero or more **Fund Summary** entries via `fund_codes`.  
  一个 **基金列表** 通过 `fund_codes` 包含零个或多个 **基金摘要**。
- A **Fund Summary** corresponds to one **Fund Detail** and zero or one **Fund Trend**.  
  一个 **基金摘要** 对应一个 **基金详情**，以及零个或一个 **基金走势**。

## State Transitions / 状态流转

- Selecting a list updates the active **Fund List** and refreshes the **Fund Summary** set.  
  选择列表会更新当前 **基金列表** 并刷新 **基金摘要** 集合。
- Selecting a fund loads **Fund Detail** and **Fund Trend** for that fund.  
  选择基金会加载对应的 **基金详情** 与 **基金走势**。
