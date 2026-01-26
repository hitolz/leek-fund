# Tauri Commands API Contract

## Metadata
- **Version**: 1.0.0
- **Created**: 2026-01-21
- **Phase**: Phase 1 - Design
- **Protocol**: Tauri IPC (Inter-Process Communication)

## Overview

This document defines Tauri commands for saving and reading holding info scoped by group and fund.

---

## Command: get_group_fund_position

### Description
Get a holding record for a specific group and fund.

### Invocation
```typescript
invoke<GroupFundPosition | null>('get_group_fund_position', { list_id: string, fund_code: string })
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `list_id` | string | Yes | UUID v4 | Group/list identifier |
| `fund_code` | string | Yes | 6 digits | Fund code |

### Response

**Success**:
```typescript
{
  list_id: string;
  fund_code: string;
  shares: number;
  unit_price: number | null;
  created_at: number;
  updated_at: number;
} | null
```

**Error**:
```typescript
string
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| Invalid list_id | "列表标识无效" |
| Invalid fund code | "基金代码格式错误" |
| Storage read failure | "读取持仓信息失败" |

---

## Command: set_group_fund_position

### Description
Create or update a holding record for a specific group and fund.

### Invocation
```typescript
invoke<GroupFundPosition>('set_group_fund_position', { list_id: string, fund_code: string, shares: number, unit_price?: number | null })
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `list_id` | string | Yes | UUID v4 | Group/list identifier |
| `fund_code` | string | Yes | 6 digits | Fund code |
| `shares` | number | Yes | > 0, max 2 decimals | Holding shares |
| `unit_price` | number | No | >= 0, max 4 decimals | Cost price per share |

### Response

**Success**:
```typescript
{
  list_id: string;
  fund_code: string;
  shares: number;
  unit_price: number | null;
  created_at: number;
  updated_at: number;
}
```

**Error**:
```typescript
string
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| Invalid list_id | "列表标识无效" |
| Invalid fund code | "基金代码格式错误" |
| Invalid shares | "持仓份额无效" |
| Invalid unit price | "成本价无效" |
| Storage write failure | "保存持仓信息失败" |

---

## Command: clear_group_fund_position

### Description
Remove a holding record for a specific group and fund.

### Invocation
```typescript
invoke<void>('clear_group_fund_position', { list_id: string, fund_code: string })
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `list_id` | string | Yes | UUID v4 | Group/list identifier |
| `fund_code` | string | Yes | 6 digits | Fund code |

### Response

**Success**: `void`

**Error**:
```typescript
string
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| Invalid list_id | "列表标识无效" |
| Invalid fund code | "基金代码格式错误" |
| Storage write failure | "清空持仓信息失败" |
