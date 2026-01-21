# Tauri Commands API Contract

## Metadata
- **Version**: 1.0.0
- **Created**: 2025-10-20
- **Phase**: Phase 1 - Design
- **Protocol**: Tauri IPC (Inter-Process Communication)

## Overview

This document defines all Tauri commands that serve as the API between the frontend (JavaScript/TypeScript) and backend (Rust). Each command is callable from the frontend using `invoke('command_name', { params })`.

---

## Command: search_fund

### Description
Searches for fund information by fund code via external API.

### Invocation
```typescript
invoke<FundInfo>('search_fund', { code: string })
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `code` | string | Yes | 6 digits, numeric | Fund code to search |

### Response

**Success** (`200 OK` equivalent):
```typescript
{
  code: string;        // Fund code (same as input)
  name: string;        // Fund name (Chinese)
  net_value: number | null;  // Current net value
  update_time: string | null; // Last update time
}
```

**Error** (thrown as exception):
```typescript
string  // User-friendly error message in Chinese
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| Invalid code format | "基金代码格式错误，请输入6位数字" |
| Fund not found | "基金代码 {code} 不存在" |
| Network timeout | "网络请求超时，请重试" |
| API error | "数据获取失败，请稍后重试" |
| Parse error | "数据解析失败，请稍后重试" |

### Behavior
- Async operation (returns Promise)
- Timeout: 10 seconds
- Does not cache results (always fetches fresh data)
- No side effects (read-only operation)

### Example Usage
```typescript
try {
  const fund = await invoke<FundInfo>('search_fund', { code: '001632' });
  console.log(`Found: ${fund.name}`);
} catch (error) {
  console.error(`Search failed: ${error}`);
}
```

---

## Command: get_all_lists

### Description
Retrieves all user's fund lists from local storage.

### Invocation
```typescript
invoke<FundList[]>('get_all_lists')
```

### Request Parameters
None

### Response

**Success**:
```typescript
Array<{
  id: string;           // UUID v4
  name: string;         // List name
  fund_codes: string[]; // Array of fund codes
  created_at: number;   // Unix timestamp (seconds)
  position: number;     // Display position (0-indexed)
}>
```

**Error**:
```typescript
string  // Error message
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| Storage file corrupted | "数据加载失败，已使用默认设置" |
| File read error | "无法读取数据文件" |

### Behavior
- Synchronous read from in-memory state (fast)
- Returns empty array if no lists exist
- Lists sorted by `position` field (ascending)
- Called on app startup and after any list modification

### Example Usage
```typescript
const lists = await invoke<FundList[]>('get_all_lists');
console.log(`User has ${lists.length} lists`);
```

---

## Command: create_list

### Description
Creates a new empty fund list with given name.

### Invocation
```typescript
invoke<FundList>('create_list', { name: string })
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `name` | string | Yes | 1-30 chars, unique | List name |

### Response

**Success**:
```typescript
{
  id: string;           // Newly generated UUID
  name: string;         // List name (same as input)
  fund_codes: [];       // Empty array
  created_at: number;   // Current timestamp
  position: number;     // Assigned position
}
```

**Error**:
```typescript
string  // Error message
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| Empty name | "列表名称不能为空" |
| Name too long | "列表名称不能超过30个字符" |
| Duplicate name | "列表名称已存在，请使用其他名称" |
| Max lists reached | "已达到最大列表数量限制(50个)" |
| Storage error | "列表创建失败，请重试" |

### Behavior
- Assigns next available position (max position + 1)
- Generates UUID v4 for list ID
- Immediately persists to storage
- Returns created list object

### Example Usage
```typescript
try {
  const newList = await invoke<FundList>('create_list', { name: '成长型基金' });
  console.log(`Created list with ID: ${newList.id}`);
} catch (error) {
  alert(error);
}
```

---

## Command: rename_list

### Description
Renames an existing fund list.

### Invocation
```typescript
invoke<void>('rename_list', { id: string, new_name: string })
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `id` | string | Yes | Valid UUID | List ID to rename |
| `new_name` | string | Yes | 1-30 chars, unique | New list name |

### Response

**Success**:
```typescript
void  // No return value on success
```

**Error**:
```typescript
string  // Error message
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| List not found | "列表不存在" |
| Empty name | "列表名称不能为空" |
| Name too long | "列表名称不能超过30个字符" |
| Duplicate name | "列表名称已存在，请使用其他名称" |
| Storage error | "列表重命名失败，请重试" |

### Behavior
- Validates new name uniqueness (excluding current list)
- Immediately persists to storage
- Does not affect fund memberships or position

### Example Usage
```typescript
try {
  await invoke('rename_list', { 
    id: 'list-uuid-here', 
    new_name: '稳健型基金' 
  });
  console.log('List renamed successfully');
} catch (error) {
  alert(error);
}
```

---

## Command: delete_list

### Description
Permanently deletes a fund list.

### Invocation
```typescript
invoke<void>('delete_list', { id: string })
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `id` | string | Yes | Valid UUID | List ID to delete |

### Response

**Success**:
```typescript
void  // No return value on success
```

**Error**:
```typescript
string  // Error message
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| List not found | "列表不存在" |
| Storage error | "列表删除失败，请重试" |

### Behavior
- Permanently removes list and all fund memberships
- Does not affect funds in other lists
- Immediately persists to storage
- Adjusts positions of remaining lists to maintain sequence

### Example Usage
```typescript
if (confirm('确认删除此列表？')) {
  try {
    await invoke('delete_list', { id: listId });
    console.log('List deleted');
  } catch (error) {
    alert(error);
  }
}
```

---

## Command: add_fund_to_list

### Description
Adds a fund to a list with duplicate checking.

### Invocation
```typescript
invoke<void>('add_fund_to_list', { 
  list_id: string, 
  fund_code: string 
})
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `list_id` | string | Yes | Valid UUID | Target list ID |
| `fund_code` | string | Yes | 6 digits | Fund code to add |

### Response

**Success**:
```typescript
void  // No return value on success
```

**Error**:
```typescript
string  // Error message
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| List not found | "列表不存在" |
| Invalid fund code | "无效的基金代码格式" |
| Duplicate fund | "基金 {code} 已在列表中" |
| List full | "列表已达到最大基金数量(200个)" |
| Storage error | "添加失败，请重试" |

### Behavior
- Validates fund code format (6 digits)
- Checks for duplicates within target list
- Appends fund to end of list (highest position)
- Immediately persists to storage
- Same fund can be added to different lists

### Example Usage
```typescript
try {
  await invoke('add_fund_to_list', { 
    list_id: selectedListId, 
    fund_code: '001632' 
  });
  showToast('已添加到列表');
} catch (error) {
  showToast(error, 'error');
}
```

---

## Command: remove_fund_from_list

### Description
Removes a fund from a list.

### Invocation
```typescript
invoke<void>('remove_fund_from_list', { 
  list_id: string, 
  fund_code: string 
})
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `list_id` | string | Yes | Valid UUID | Source list ID |
| `fund_code` | string | Yes | 6 digits | Fund code to remove |

### Response

**Success**:
```typescript
void  // No return value on success
```

**Error**:
```typescript
string  // Error message
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| List not found | "列表不存在" |
| Fund not in list | "基金不在此列表中" |
| Storage error | "移除失败，请重试" |

### Behavior
- Removes fund from specified list only
- Does not affect fund's presence in other lists
- Maintains order of remaining funds
- Immediately persists to storage

### Example Usage
```typescript
if (confirm('确认从列表中移除此基金？')) {
  try {
    await invoke('remove_fund_from_list', { 
      list_id: currentListId, 
      fund_code: fundCode 
    });
    refreshListView();
  } catch (error) {
    alert(error);
  }
}
```

---

## Command: get_list_funds

### Description
Retrieves detailed information for all funds in a list.

### Invocation
```typescript
invoke<FundInfo[]>('get_list_funds', { list_id: string })
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `list_id` | string | Yes | Valid UUID | List ID to query |

### Response

**Success**:
```typescript
Array<{
  code: string;
  name: string;
  net_value: number | null;
  update_time: string | null;
}>
```

**Error**:
```typescript
string  // Error message
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| List not found | "列表不存在" |
| Network errors | "部分基金信息获取失败" (partial failure) |

### Behavior
- Fetches fund info for each code in list from external API
- Returns array in same order as list's fund_codes
- Partial failures: Returns available data, logs errors
- Can be slow for large lists (sequential API calls)
- Does not cache results

### Example Usage
```typescript
try {
  const funds = await invoke<FundInfo[]>('get_list_funds', { 
    list_id: selectedListId 
  });
  displayFunds(funds);
} catch (error) {
  console.error('Failed to load funds:', error);
}
```

---

## Command: reorder_lists

### Description
Updates the display order of all lists.

### Invocation
```typescript
invoke<void>('reorder_lists', { list_ids: string[] })
```

### Request Parameters

| Parameter | Type | Required | Constraints | Description |
|-----------|------|----------|-------------|-------------|
| `list_ids` | string[] | Yes | All list IDs in new order | Ordered array of list UUIDs |

### Response

**Success**:
```typescript
void  // No return value on success
```

**Error**:
```typescript
string  // Error message
```

### Error Cases

| Condition | Error Message |
|-----------|---------------|
| Missing list IDs | "列表ID不完整" |
| Unknown list ID | "包含无效的列表ID" |
| Storage error | "排序保存失败，请重试" |

### Behavior
- Validates all list IDs exist
- Updates position field for each list (index = position)
- Immediately persists to storage
- Subsequent `get_all_lists()` returns lists in new order

### Example Usage
```typescript
const newOrder = ['uuid3', 'uuid1', 'uuid2']; // Drag-drop result
try {
  await invoke('reorder_lists', { list_ids: newOrder });
  console.log('Lists reordered');
} catch (error) {
  alert('排序失败: ' + error);
}
```

---

## Type Definitions (TypeScript)

```typescript
// Frontend type definitions for all commands

interface FundInfo {
  code: string;
  name: string;
  net_value: number | null;
  update_time: string | null;
}

interface FundList {
  id: string;
  name: string;
  fund_codes: string[];
  created_at: number;
  position: number;
}

// Command wrappers
const TauriAPI = {
  searchFund: (code: string): Promise<FundInfo> => 
    invoke('search_fund', { code }),
  
  getAllLists: (): Promise<FundList[]> => 
    invoke('get_all_lists'),
  
  createList: (name: string): Promise<FundList> => 
    invoke('create_list', { name }),
  
  renameList: (id: string, newName: string): Promise<void> => 
    invoke('rename_list', { id, new_name: newName }),
  
  deleteList: (id: string): Promise<void> => 
    invoke('delete_list', { id }),
  
  addFundToList: (listId: string, fundCode: string): Promise<void> => 
    invoke('add_fund_to_list', { list_id: listId, fund_code: fundCode }),
  
  removeFundFromList: (listId: string, fundCode: string): Promise<void> => 
    invoke('remove_fund_from_list', { list_id: listId, fund_code: fundCode }),
  
  getListFunds: (listId: string): Promise<FundInfo[]> => 
    invoke('get_list_funds', { list_id: listId }),
  
  reorderLists: (listIds: string[]): Promise<void> => 
    invoke('reorder_lists', { list_ids: listIds }),
};
```

---

## Error Handling Pattern

### Frontend Error Handling

```typescript
async function handleTauriCommand<T>(
  commandFn: () => Promise<T>,
  errorMessage: string = '操作失败'
): Promise<T | null> {
  try {
    return await commandFn();
  } catch (error) {
    // error is a string from Rust
    console.error('Tauri command failed:', error);
    showToast(`${errorMessage}: ${error}`, 'error');
    return null;
  }
}

// Usage
const fund = await handleTauriCommand(
  () => TauriAPI.searchFund('001632'),
  '基金查询失败'
);
```

### Backend Error Pattern

```rust
// Rust command handler pattern
#[tauri::command]
async fn search_fund(code: String) -> Result<FundInfo, String> {
    fund_api::search(&code)
        .await
        .map_err(|e| e.user_message())
}
```

---

## Performance Specifications

| Command | Expected Latency | Notes |
|---------|-----------------|-------|
| `search_fund` | 500-2000ms | Network dependent |
| `get_all_lists` | <10ms | In-memory read |
| `create_list` | <50ms | Includes file write |
| `rename_list` | <50ms | Includes file write |
| `delete_list` | <50ms | Includes file write |
| `add_fund_to_list` | <50ms | Includes file write |
| `remove_fund_from_list` | <50ms | Includes file write |
| `get_list_funds` | 500ms per fund | Sequential API calls |
| `reorder_lists` | <50ms | Includes file write |

---

## Security Considerations

1. **Input Validation**: All inputs validated in Rust before processing
2. **Error Messages**: Never expose internal paths or technical details
3. **Rate Limiting**: Consider adding rate limit for `search_fund` to prevent API abuse
4. **Data Sanitization**: List names sanitized to prevent injection attacks

---

## Contract Completion Checklist

- [x] All commands documented with signatures
- [x] Request parameters specified with constraints
- [x] Response formats defined with TypeScript types
- [x] Error cases enumerated with user-facing messages
- [x] Behavior and side effects described
- [x] Example usage provided for each command
- [x] TypeScript type definitions included
- [x] Error handling patterns documented
- [x] Performance expectations specified
- [x] Security considerations noted

## Status: ✅ API Contract Complete

All Tauri commands fully specified and ready for implementation.

