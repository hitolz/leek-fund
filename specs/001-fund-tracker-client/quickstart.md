# Quickstart: Fund List Management
# 快速开始：基金列表管理

**Goal**: Run the desktop client and verify fund lookup + list management.
**目标**：运行桌面客户端并验证基金查询与列表管理。

## Prerequisites
## 前置条件

- Rust 1.70+
- Node.js 18+
- macOS for local development (Windows/Linux supported for release)

- Rust 1.70+
- Node.js 18+
- macOS 用于本地开发（Windows/Linux 支持发布）

## Install
## 安装

```bash
npm install
```

```bash
npm install
```

## Run (Development)
## 运行（开发）

```bash
npm run tauri:dev
```

```bash
npm run tauri:dev
```

## Verify Core Flows
## 验证核心流程

1. In the search input, enter a valid fund code (e.g., `001632`).
2. Confirm fund name, net value, change percentage, and update time appear.
3. Create a new list named "My Funds".
4. Add the searched fund to the list and verify it appears once.
5. Verify list data auto-refreshes within 1–5 minutes (default 3).
6. Remove the fund from the list and verify the list is empty.
7. Close the app, reopen it, and verify lists are still present.

1. 在搜索框输入有效基金代码（如 `001632`）。
2. 确认显示基金名称、净值、涨跌幅与更新时间。
3. 创建名为 “My Funds” 的新列表。
4. 将基金加入列表并验证仅出现一次。
5. 验证列表数据在 1–5 分钟内自动刷新（默认 3）。
6. 从列表移除基金并确认列表为空。
7. 关闭并重启应用，验证列表仍存在。

## Troubleshooting
## 故障排查

- If fund data fails to load, confirm network access and retry the query.
- If the UI is unresponsive, check for long-running operations on the main
  thread and restart the app.

- 若基金数据加载失败，检查网络并重试。
- 若 UI 卡顿，检查主线程耗时操作并重启应用。
