# Quickstart: Fund Add Search Filter

**Date**: 2026-01-22  
**Feature**: `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/spec.md`

## Goal

Verify that adding an existing fund filters the lower list, while adding a new fund behaves normally.  
验证添加已存在基金会过滤下方列表，而新增基金保持正常添加行为。

## Prerequisites

- A dataset with at least two funds (Fund A and Fund B).  
  至少包含两只基金（基金A与基金B）的数据集。
- Fund A is already in the middle list before testing.  
  测试前基金A已在中间列表中。

## Run

1. Start the app in development mode.
   
   ```bash
   npm run tauri:dev
   ```
   
   启动应用的开发模式。

2. In the middle list, click the add button for Fund A (already present).  
   在中间列表中点击基金A（已存在）的添加按钮。

3. Confirm the lower list shows only Fund A and a non-blocking “already exists” hint with a clear-filter control.  
   确认下方列表仅显示基金A，并出现“已存在”的非阻塞提示与清除筛选按钮。

4. Use the clear-filter action to return to the full lower list.  
   使用清除过滤操作恢复下方列表为全量显示。

5. Click the add button for Fund B (not present).  
   点击基金B（不存在）的添加按钮。

6. Confirm the lower list remains unfiltered and Fund B is added normally.  
   确认下方列表未被过滤，且基金B正常添加。
