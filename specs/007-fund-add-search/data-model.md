# Data Model: Fund Add Search Filter

**Date**: 2026-01-22  
**Feature**: `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/spec.md`

## Entities

### Fund

Represents a single fund item available to users.  
表示用户可查看的单个基金条目。

**Fields**:
- `id`: Unique identifier or code for the fund.  
  `id`：基金唯一标识或代码。
- `name`: Display name.  
  `name`：显示名称。

### FundList

Represents a collection of funds shown in the UI.  
表示界面中展示的基金集合。

**Fields**:
- `listType`: `middle` or `lower`.  
  `listType`：`middle` 或 `lower`。
- `fundIds`: Ordered list of fund identifiers.  
  `fundIds`：基金标识的有序列表。

### FundFilterState

Captures whether the lower list is filtered to a single fund.  
描述下方列表是否被过滤为单一基金。

**Fields**:
- `isFiltered`: Boolean flag.  
  `isFiltered`：布尔标志。
- `fundId`: Identifier of the filtered fund (when applicable).  
  `fundId`：被过滤的基金标识（适用时）。
- `reason`: `existing-add` when triggered by adding an existing fund.  
  `reason`：当由添加已存在基金触发时为 `existing-add`。

## Relationships

- The middle list references Fund items by `fundIds`.  
  中间列表通过 `fundIds` 引用基金条目。
- The lower list view is derived from all funds, optionally filtered by `FundFilterState.fundId`.  
  下方列表基于所有基金生成，可由 `FundFilterState.fundId` 进行过滤。

## Validation Rules

- `Fund.id` must be unique across all fund collections.  
  `Fund.id` 在所有基金集合中必须唯一。
- `FundFilterState.fundId` must exist in the lower list dataset when `isFiltered` is true.  
  当 `isFiltered` 为 true 时，`FundFilterState.fundId` 必须存在于下方列表数据中。

## State Transitions

- Unfiltered → Filtered when an add action targets an existing fund.  
  未过滤 → 过滤：当添加动作命中已存在基金。
- Filtered → Unfiltered when the user clears the filter.  
  过滤 → 未过滤：当用户清除过滤。
