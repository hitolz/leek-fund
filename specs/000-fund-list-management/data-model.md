# Data Model: Fund List Management

## Metadata
- **Created**: 2025-10-20
- **Phase**: Phase 1 - Design
- **Status**: Complete
- **Version**: 1.0.0

## Purpose

This document defines all data structures used in the Fund List Management feature, including their fields, validation rules, relationships, and state transitions.

---

## Entity Definitions

### 1. FundInfo

Represents detailed information about a mutual fund returned from external API.

#### Fields

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| `code` | String | Yes | 6 digits, numeric | Unique fund identifier |
| `name` | String | Yes | 1-100 chars | Fund name (Chinese) |
| `net_value` | Float | No | > 0 | Current net asset value |
| `update_time` | String | No | ISO 8601 format | Last update timestamp |

#### Validation Rules
- `code`: Must match regex `^\d{6}$`
- `name`: Cannot be empty, must contain valid UTF-8
- `net_value`: If present, must be positive number
- `update_time`: If present, must be valid timestamp

#### Source
External API: `fundgz.1234567.com.cn/js/{code}.js`

#### Persistence
Not persisted locally. Fetched on-demand during searches.

#### Relationships
- Referenced by `FundList.fund_codes` (many-to-many via code)

---

### 2. FundList

Represents a user-created collection of funds.

#### Fields

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| `id` | String | Yes | UUID v4 | Unique list identifier |
| `name` | String | Yes | 1-30 chars, unique | User-defined list name |
| `fund_codes` | Array<String> | Yes | 0-200 items, unique within list | Ordered collection of fund codes |
| `created_at` | Integer | Yes | Unix timestamp | List creation time |
| `position` | Integer | Yes | >= 0 | Display order (0 = first) |

#### Validation Rules
- `id`: Must be valid UUID v4 format
- `name`: 
  - Length: 1-30 characters
  - Must be unique across all user's lists
  - Cannot contain only whitespace
- `fund_codes`:
  - Each code must match `^\d{6}$`
  - No duplicates within same list
  - Same code can appear in different lists
  - Ordered by insertion (newest last unless reordered)
- `created_at`: Must be valid Unix timestamp (seconds since epoch)
- `position`: Non-negative integer, unique across lists

#### State Transitions

```
[New] --create_list()--> [Active]
[Active] --rename_list()--> [Active] (name changed)
[Active] --delete_list()--> [Deleted] (removed from storage)

Fund membership:
[List with N funds] --add_fund()--> [List with N+1 funds]
[List with N funds] --remove_fund()--> [List with N-1 funds]
```

#### Business Rules
1. List name must be unique (case-sensitive)
2. Duplicate fund codes within same list are prohibited
3. Empty lists are valid (0 fund codes)
4. Maximum 200 funds per list (performance constraint)
5. Position field must be updated on reorder operations
6. Deleting a list does not affect funds in other lists

#### Relationships
- **Has Many**: Fund codes (via `fund_codes` array)
- **Belongs To**: AppState (via `lists` collection)

#### Persistence
Stored in `lists.json` as part of `StorageFormat.lists` array.

---

### 3. StorageFormat

Root container for all persisted user data.

#### Fields

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| `version` | Integer | Yes | Currently 1 | Schema version for migrations |
| `lists` | Array<FundList> | Yes | 0-50 items | All user's fund lists |
| `created_at` | Integer | Yes | Unix timestamp | First data file creation |
| `last_modified` | Integer | Yes | Unix timestamp | Last save operation |

#### Validation Rules
- `version`: Must be supported version (currently only 1)
- `lists`:
  - All list names must be unique
  - All list positions must be unique and sequential (0, 1, 2, ...)
  - Maximum 50 lists
- `last_modified`: Must be >= `created_at`

#### File Location
- Path: `{app_data_dir}/leek-fund/lists.json`
- Platform-specific:
  - macOS: `~/Library/Application Support/leek-fund/lists.json`
  - Windows: `%APPDATA%/leek-fund/lists.json`
  - Linux: `~/.local/share/leek-fund/lists.json`

#### Persistence Strategy
- **Write**: Atomic write to temp file + rename
- **Read**: Validate version, apply migrations if needed
- **Backup**: On corruption, rename to `lists.json.backup.{timestamp}`
- **Frequency**: On every mutation (create, update, delete operations)

#### Initial State
When no file exists:
```json
{
  "version": 1,
  "lists": [],
  "created_at": <current_timestamp>,
  "last_modified": <current_timestamp>
}
```

---

### 4. AppState

Runtime application state managed by Tauri.

#### Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `lists` | Vec<FundList> | Yes | In-memory copy of user's lists |
| `storage_path` | PathBuf | Yes | Full path to storage file |

#### Lifecycle
- **Initialize**: Load from storage file on app startup
- **Runtime**: Modified by Tauri commands, immediately persisted
- **Shutdown**: Automatic persistence before app closes (failsafe)

#### Thread Safety
Managed by Tauri's state management (Mutex-protected).

---

## Data Relationships

### Entity Relationship Diagram

```
┌─────────────────┐
│  StorageFormat  │
│  (Root)         │
│  - version      │
│  - created_at   │
│  - last_modified│
└────────┬────────┘
         │ 1
         │ contains
         │ *
┌────────▼────────┐
│    FundList     │
│  - id           │
│  - name         │
│  - fund_codes[] │ ────references──> FundInfo
│  - created_at   │                   (external API)
│  - position     │
└─────────────────┘
```

### Relationship Details

1. **StorageFormat → FundList**: One-to-Many
   - One `StorageFormat` contains 0-50 `FundList` entities
   - Cascade delete: Deleting storage file removes all lists

2. **FundList → FundInfo**: Many-to-Many (via code reference)
   - `FundList.fund_codes` array stores fund codes
   - `FundInfo` fetched on-demand, not stored in list
   - Same fund can be in multiple lists
   - Removing fund from list doesn't affect other lists

---

## Validation Matrix

### Cross-Entity Validation

| Rule | Entities | Validation | Error Message |
|------|----------|------------|---------------|
| Unique list names | FundList | All list.name must be unique (case-sensitive) | "列表名称已存在，请使用其他名称" |
| Unique positions | FundList | All list.position must be unique within AppState | "列表位置冲突" (internal error) |
| No duplicate funds in list | FundList | list.fund_codes must have no duplicates | "基金 {code} 已在列表中" |
| Fund code format | FundList, FundInfo | All codes match `^\d{6}$` | "无效的基金代码格式" |
| Version compatibility | StorageFormat | version must be supported (currently 1) | "不支持的数据格式版本" |
| List count limit | StorageFormat | lists.length <= 50 | "已达到最大列表数量限制(50个)" |
| Fund count limit | FundList | fund_codes.length <= 200 | "列表已达到最大基金数量(200个)" |

---

## State Transitions

### FundList State Machine

```
Initial State: [Non-existent]
    │
    │ create_list(name)
    ▼
[Active, Empty] (position assigned, fund_codes = [])
    │
    ├─ add_fund(code) ──> [Active, Non-empty]
    │                           │
    │                           ├─ add_fund(code) ──> [Active, Non-empty]
    │                           │
    │                           └─ remove_fund(code) ──> [Active, Non-empty or Empty]
    │
    ├─ rename_list(new_name) ──> [Active] (name changed)
    │
    ├─ reorder (position changed) ──> [Active] (position updated)
    │
    └─ delete_list() ──> [Deleted] (removed from storage)
```

### StorageFormat State Machine

```
[Non-existent] ──first_operation──> [Created, Empty]
                                           │
[Created, Empty] ──create_list──> [Has Lists]
                                           │
                                           ├─ list_operations ──> [Has Lists] (updated)
                                           │
                                           └─ delete_all_lists ──> [Created, Empty]
```

---

## Serialization Format

### JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "StorageFormat",
  "type": "object",
  "required": ["version", "lists", "created_at", "last_modified"],
  "properties": {
    "version": {
      "type": "integer",
      "const": 1
    },
    "lists": {
      "type": "array",
      "maxItems": 50,
      "items": {
        "type": "object",
        "required": ["id", "name", "fund_codes", "created_at", "position"],
        "properties": {
          "id": {
            "type": "string",
            "format": "uuid"
          },
          "name": {
            "type": "string",
            "minLength": 1,
            "maxLength": 30
          },
          "fund_codes": {
            "type": "array",
            "maxItems": 200,
            "items": {
              "type": "string",
              "pattern": "^\\d{6}$"
            },
            "uniqueItems": true
          },
          "created_at": {
            "type": "integer",
            "minimum": 0
          },
          "position": {
            "type": "integer",
            "minimum": 0
          }
        }
      }
    },
    "created_at": {
      "type": "integer",
      "minimum": 0
    },
    "last_modified": {
      "type": "integer",
      "minimum": 0
    }
  }
}
```

### Example Data

```json
{
  "version": 1,
  "lists": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "成长型基金",
      "fund_codes": ["001632", "014938", "002560"],
      "created_at": 1729468800,
      "position": 0
    },
    {
      "id": "6fa459ea-ee8a-3ca4-894e-db77e160355e",
      "name": "稳健型基金",
      "fund_codes": ["001632", "110022"],
      "created_at": 1729469000,
      "position": 1
    }
  ],
  "created_at": 1729468800,
  "last_modified": 1729470000
}
```

---

## Migration Strategy

### Version 1 (Current)

Initial schema with all defined fields.

### Future Migrations

When schema changes required:

1. Increment `version` field
2. Implement migration function: `migrate_v{old}_to_v{new}()`
3. Apply migrations sequentially on load
4. Never downgrade versions (one-way migrations)

Example migration structure:
```rust
match data.version {
    1 => Ok(data),
    2 => migrate_v1_to_v2(data),
    v => Err(format!("Unsupported version: {}", v))
}
```

---

## Performance Considerations

### Memory Usage

- FundList (empty): ~100 bytes
- FundList (100 funds): ~1.5 KB
- StorageFormat (50 lists, 100 funds each): ~75 KB
- **Estimate**: <500 KB total for typical usage

### Disk Usage

- JSON file (50 lists, 100 funds each): ~100 KB (formatted)
- Growth rate: ~1 KB per 50 funds added
- **Estimate**: <1 MB for maximum capacity (50 lists × 200 funds)

### Load Time

- Parse 100 KB JSON: <10ms (typical)
- Validate all constraints: <5ms
- **Target**: <50ms total startup time for data loading

---

## Data Model Completion Checklist

- [x] All entities defined with complete field specifications
- [x] Validation rules documented for all fields
- [x] Relationships mapped between entities
- [x] State transitions documented
- [x] Serialization format specified (JSON schema)
- [x] Example data provided
- [x] Migration strategy defined
- [x] Performance characteristics estimated
- [x] Business rules documented
- [x] Error messages specified

## Status: ✅ Data Model Complete

All data structures fully specified and ready for implementation.

