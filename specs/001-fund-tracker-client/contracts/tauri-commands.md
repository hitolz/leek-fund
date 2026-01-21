# Tauri Commands API Contract
# Tauri 命令 API 契约

## Metadata
## 元数据

- **Version**: 1.0.0
- **Created**: 2026-01-21
- **Protocol**: Tauri IPC

- **版本**：1.0.0
- **创建日期**：2026-01-21
- **协议**：Tauri IPC

## Overview
## 概述

Defines the Tauri commands used by the frontend to query funds and manage lists.

定义前端用于查询基金与管理列表的 Tauri 命令。

---

## Command: search_fund
## 命令：search_fund

**Description**: Fetch fund info by 6-digit code.
**描述**：通过 6 位代码获取基金信息。

**Invocation**:
```typescript
invoke<FundInfo>('search_fund', { code: string })
```

**调用方式**：
```typescript
invoke<FundInfo>('search_fund', { code: string })
```

**Parameters**:

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `code` | string | Yes | 6 digits | Fund code |

**参数**：

| 参数 | 类型 | 必填 | 约束 | 说明 |
|------|------|------|------|------|
| `code` | string | 是 | 6 位数字 | 基金代码 |

**Response**:

```typescript
{
  code: string;
  name: string;
  net_value: number | null;
  change_percent: string | null;
  update_time: string | null;
}
```

**响应**：

```typescript
{
  code: string;
  name: string;
  net_value: number | null;
  change_percent: string | null;
  update_time: string | null;
}
```

**Error Cases**: Invalid code, not found, network failure, parse failure.
**错误情况**：无效代码、未找到、网络失败、解析失败。

---

## Command: get_all_lists
## 命令：get_all_lists

**Description**: Return all lists and membership codes.
**描述**：返回所有列表及其成员代码。

**Invocation**:
```typescript
invoke<FundList[]>('get_all_lists')
```

**调用方式**：
```typescript
invoke<FundList[]>('get_all_lists')
```

---

## Command: create_list
## 命令：create_list

**Description**: Create a new list.
**描述**：创建新列表。

**Invocation**:
```typescript
invoke<FundList>('create_list', { name: string })
```

**调用方式**：
```typescript
invoke<FundList>('create_list', { name: string })
```

---

## Command: rename_list
## 命令：rename_list

**Description**: Rename an existing list.
**描述**：重命名已有列表。

**Invocation**:
```typescript
invoke<void>('rename_list', { id: string, new_name: string })
```

**调用方式**：
```typescript
invoke<void>('rename_list', { id: string, new_name: string })
```

---

## Command: delete_list
## 命令：delete_list

**Description**: Delete a list.
**描述**：删除列表。

**Invocation**:
```typescript
invoke<void>('delete_list', { id: string })
```

**调用方式**：
```typescript
invoke<void>('delete_list', { id: string })
```

---

## Command: add_fund_to_list
## 命令：add_fund_to_list

**Description**: Add a fund to a list (no duplicates).
**描述**：向列表添加基金（不允许重复）。

**Invocation**:
```typescript
invoke<void>('add_fund_to_list', { list_id: string, fund_code: string })
```

**调用方式**：
```typescript
invoke<void>('add_fund_to_list', { list_id: string, fund_code: string })
```

---

## Command: remove_fund_from_list
## 命令：remove_fund_from_list

**Description**: Remove a fund from a list.
**描述**：从列表移除基金。

**Invocation**:
```typescript
invoke<void>('remove_fund_from_list', { list_id: string, fund_code: string })
```

**调用方式**：
```typescript
invoke<void>('remove_fund_from_list', { list_id: string, fund_code: string })
```

---

## Command: get_list_funds
## 命令：get_list_funds

**Description**: Fetch fund details for all fund codes in a list.
**描述**：获取列表内所有基金的详情。

**Invocation**:
```typescript
invoke<FundInfo[]>('get_list_funds', { list_id: string })
```

**调用方式**：
```typescript
invoke<FundInfo[]>('get_list_funds', { list_id: string })
```

---

## Type Definitions
## 类型定义

```typescript
interface FundInfo {
  code: string;
  name: string;
  net_value: number | null;
  change_percent: string | null;
  update_time: string | null;
}

interface FundList {
  id: string;
  name: string;
  fund_codes: string[];
  created_at: number;
  updated_at: number;
  position: number;
}
```

```typescript
interface FundInfo {
  code: string;
  name: string;
  net_value: number | null;
  change_percent: string | null;
  update_time: string | null;
}

interface FundList {
  id: string;
  name: string;
  fund_codes: string[];
  created_at: number;
  updated_at: number;
  position: number;
}
```
