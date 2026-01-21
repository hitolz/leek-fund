# Data Model / 数据模型

## Overview / 概览

This model defines SQLite tables and fields for groups, funds, and group-fund links.  
本模型定义用于分组（group）、基金与关联关系的 SQLite 表与字段。

## Entities / 实体

### Group / 分组

**Description**: A user-defined list with ordering and timestamps.  
**说明**：用户自定义的列表，包含排序与时间戳。

**Fields**:  
**字段**：

- `id`: Integer primary key. / 整型主键。
- `name`: Group name (VARCHAR(64)). / 分组名称（VARCHAR(64)）。
- `position`: Display order. / 显示顺序。
- `created_at`: Creation timestamp (INTEGER). / 创建时间戳（INTEGER）。
- `updated_at`: Update timestamp (INTEGER). / 更新时间戳（INTEGER）。

### Fund / 基金

**Description**: Fund master data keyed by code.  
**说明**：以基金代码为主的基金基础数据。

**Fields**:  
**字段**：

- `id`: Integer primary key. / 整型主键。
- `code`: Fund code (VARCHAR(64)). / 基金代码（VARCHAR(64)）。
- `name`: Fund name (VARCHAR(64), optional). / 基金名称（VARCHAR(64)，可选）。
- `created_at`: Creation timestamp (INTEGER). / 创建时间戳（INTEGER）。
- `updated_at`: Update timestamp (INTEGER). / 更新时间戳（INTEGER）。

### Group Fund / 分组基金关联

**Description**: Link between group and fund code with ordering.  
**说明**：分组与基金代码的关联（含排序）。

**Fields**:  
**字段**：

- `id`: Integer primary key. / 整型主键。
- `group_id`: Group id (INTEGER). / 分组 id（INTEGER）。
- `fund_code`: Fund code (VARCHAR(64)). / 基金代码（VARCHAR(64)）。
- `position`: Order within group. / 分组内顺序。
- `created_at`: Creation timestamp (INTEGER). / 创建时间戳（INTEGER）。
- `updated_at`: Update timestamp (INTEGER). / 更新时间戳（INTEGER）。

## Relationships / 关系

- A **Group** has many **Group Fund** records.  
  一个 **分组** 包含多个 **分组基金关联**。
- A **Group Fund** references a **Fund** by `fund_code`.  
  一个 **分组基金关联** 通过 `fund_code` 引用 **基金**。

## Validation Rules / 校验规则

- `fund_code` must be 6 digits and unique per group.  
  `fund_code` 必须为 6 位且在同一分组内唯一。
- `name` must be non-empty and <= 64 characters.  
  `name` 必须非空且不超过 64 字符。

## State Transitions / 状态流转

- On first run after upgrade: JSON data is imported into groups/funds/group_funds.  
  升级后首次运行：JSON 数据导入到 groups/funds/group_funds。
- On subsequent runs: read group and fund relations only from SQLite.  
  后续运行：仅从 SQLite 读取分组与基金关联。

## SQLite Schema (Appendix) / SQLite 表结构（附录）

```sql
-- groups
CREATE TABLE IF NOT EXISTS groups (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(64) NOT NULL,
  position INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

-- funds
CREATE TABLE IF NOT EXISTS funds (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  code VARCHAR(64) NOT NULL UNIQUE,
  name VARCHAR(64),
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

-- group_funds
CREATE TABLE IF NOT EXISTS group_funds (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  group_id INTEGER NOT NULL,
  fund_code VARCHAR(64) NOT NULL,
  position INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE (group_id, fund_code)
);

CREATE INDEX IF NOT EXISTS idx_group_funds_group_id ON group_funds (group_id);
CREATE INDEX IF NOT EXISTS idx_group_funds_fund_code ON group_funds (fund_code);
```
