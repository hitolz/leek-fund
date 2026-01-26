# Data Model: Holding Amount by Group & Fund

## Metadata
- **Created**: 2026-01-21
- **Phase**: Phase 1 - Design
- **Status**: Draft
- **Version**: 1.0.0

## Purpose

Define the data structures needed to store holding info scoped by group and fund, and to compute holding and daily change amounts in the fund detail panel.

---

## Entity Definitions

### 1. GroupFundPosition

Represents a user-defined holding record for a specific group and fund.

#### Fields

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| `list_id` | String | Yes | UUID v4 | The group/list identifier |
| `fund_code` | String | Yes | 6 digits, numeric | Fund code |
| `shares` | Float | Yes | > 0, max 2 decimals | Holding shares |
| `unit_price` | Float | No | >= 0, max 4 decimals | Cost price per share |
| `created_at` | Integer | Yes | Unix timestamp | Creation time |
| `updated_at` | Integer | Yes | Unix timestamp | Last update time |

#### Validation Rules
- `list_id`: Must be a valid UUID v4
- `fund_code`: Must match regex `^\d{6}$`
- `shares`: Positive number, up to 2 decimal places
- `unit_price`: Non-negative number, up to 4 decimal places
- `updated_at`: Must be >= `created_at`

#### Derived Fields (not persisted)
- `holding_amount`: `shares * latest_nav`
- `daily_change_amount`: `holding_amount * daily_change_percent / 100`

#### Relationships
- Belongs to `FundList` via `list_id`
- References fund data by `fund_code`

---

### 2. StorageFormat (extension)

Extend existing local storage with a `positions` collection.

#### Fields (new)

| Field | Type | Required | Constraints | Description |
|-------|------|----------|-------------|-------------|
| `positions` | Array<GroupFundPosition> | Yes | 0..N | Holding records by group + fund |

#### Migration Rules
- If `positions` is missing, initialize to empty array on load
- Existing storage fields are preserved

---

## Data Relationships

```
FundList (list_id) 1 ──── * GroupFundPosition (list_id, fund_code)
```

---

## State Transitions

```
[No Position] --set_position()--> [Active Position]
[Active Position] --update_position()--> [Active Position]
[Active Position] --clear_position()--> [No Position]
```
