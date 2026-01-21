# Data Model: Fund List Management
# 数据模型：基金列表管理

**Date**: 2026-01-21
**日期**：2026-01-21

## Entities
## 实体

### Fund
### 基金

Represents a fund returned by the external data source.

代表从外部数据源返回的基金。

- **Fields**:
  - `code`: string, 6-digit numeric identifier
  - `name`: string, fund name
  - `net_value`: number | null, current net value
  - `change_percent`: string | null, daily change percentage
  - `update_time`: string | null, last update time
- **Validation**:
  - `code` MUST match `^[0-9]{6}$`
  - `name` MUST be non-empty when a fund is found

- **字段**：
  - `code`：string，6 位数字代码
  - `name`：string，基金名称
  - `net_value`：number | null，当前净值
  - `change_percent`：string | null，当日涨跌幅
  - `update_time`：string | null，更新时间
- **校验**：
  - `code` 必须匹配 `^[0-9]{6}$`
  - `name` 必须非空

### FundList
### 基金列表

User-defined list of funds.

用户自定义基金列表。

- **Fields**:
  - `id`: string, UUID
  - `name`: string, 1-30 characters, unique
  - `fund_codes`: string[], ordered set of fund codes
  - `created_at`: number, unix timestamp (seconds)
  - `updated_at`: number, unix timestamp (seconds)
  - `position`: number, display order (0-based)
- **Validation**:
  - `name` MUST be unique across lists
  - `fund_codes` MUST contain unique codes
  - list size MUST NOT exceed 200 funds

- **字段**：
  - `id`：string，UUID
  - `name`：string，1-30 字符，唯一
  - `fund_codes`：string[]，有序基金代码集合
  - `created_at`：number，Unix 时间戳（秒）
  - `updated_at`：number，Unix 时间戳（秒）
  - `position`：number，显示顺序（从 0 开始）
- **校验**：
  - `name` 必须唯一
  - `fund_codes` 必须去重
  - 单列表基金数不得超过 200

### UserData
### 用户数据

Persisted application state.

持久化的应用状态。

- **Fields**:
  - `schema_version`: number
  - `lists`: FundList[]
  - `last_migrated_at`: number | null
  - `preferences`: object (optional, reserved for future)

- **字段**：
  - `schema_version`：number
  - `lists`：FundList[]
  - `last_migrated_at`：number | null
  - `preferences`：object（可选，预留）

## Relationships
## 关系

- A **FundList** contains many **Fund** references via `fund_codes`.
- A **Fund** can appear in multiple **FundLists**.
- **UserData** owns all **FundLists** and metadata.

- **FundList** 通过 `fund_codes` 引用多个 **Fund**。
- **Fund** 可存在于多个 **FundList**。
- **UserData** 持有所有 **FundList** 与元数据。

## State Transitions
## 状态转换

- **Create list**: adds a new FundList with empty `fund_codes`.
- **Rename list**: updates `name` and `updated_at`.
- **Delete list**: removes the FundList and its memberships.
- **Add fund**: appends a fund code to `fund_codes` if not present.
- **Remove fund**: removes a fund code from `fund_codes` if present.
- **Load data**: validates `schema_version`; runs migration if needed.

- **创建列表**：新增空 `fund_codes` 的 FundList。
- **重命名列表**：更新 `name` 与 `updated_at`。
- **删除列表**：移除列表及其成员。
- **添加基金**：若不存在则追加 `fund_codes`。
- **移除基金**：从 `fund_codes` 移除对应代码。
- **加载数据**：校验 `schema_version` 并按需迁移。

## Persistence Notes
## 持久化说明

- UserData is serialized to JSON and stored locally in the Tauri app data
  directory.
- On corruption, load should fail gracefully and prompt recovery guidance.

- UserData 序列化为 JSON，保存到 Tauri 应用数据目录。
- 数据损坏时，需优雅失败并提示恢复指引。
