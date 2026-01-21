# Research: Fund List Management
# 研究：基金列表管理

**Date**: 2026-01-21
**Scope**: Fund lookup, list management, auto-refresh, local persistence for a Tauri desktop app

**日期**：2026-01-21
**范围**：基金查询、列表管理、自动刷新、Tauri 桌面应用的本地持久化

## Decisions
## 决策

### Decision 1: Fund data source
### 决策 1：基金数据源

- **Decision**: Use `fundgz.1234567.com.cn` JSONP endpoint for fund lookup.
- **Rationale**: Required by constitution; returns near-real-time fund data by
  6-digit code and supports update time display.
- **Alternatives considered**: Eastmoney HTML pages; Sina fund endpoints.

- **决策**：使用 `fundgz.1234567.com.cn` JSONP 接口查询基金。
- **理由**：宪法要求；提供近实时基金数据并包含更新时间。
- **备选**：东财页面接口、Sina 基金接口。

### Decision 2: Persistence strategy
### 决策 2：持久化策略

- **Decision**: Store user lists and memberships in a local JSON file in the
  Tauri app data directory.
- **Rationale**: Matches local-only privacy requirement and ensures offline
  access to list structure.
- **Alternatives considered**: SQLite embedded DB; cloud sync service.

- **决策**：在 Tauri 应用数据目录存储本地 JSON 文件。
- **理由**：符合本地隐私要求，并支持离线查看列表结构。
- **备选**：SQLite 内嵌数据库；云同步服务。

### Decision 3: Data limits for lists
### 决策 3：列表规模限制

- **Decision**: Use practical limits of 50 lists and 200 funds per list.
- **Rationale**: Keeps UI responsive and storage small; aligns with typical usage.
- **Alternatives considered**: Unlimited lists or configurable limits.

- **决策**：列表上限 50，单列表基金上限 200。
- **理由**：保证 UI 流畅与存储规模合理，符合常见使用场景。
- **备选**：无限制或可配置上限。

### Decision 4: Command surface
### 决策 4：命令边界

- **Decision**: Expose fund search and list operations through Tauri commands.
- **Rationale**: Keeps business logic in Rust modules, aligns with existing
  architecture and testing guidance.
- **Alternatives considered**: Direct frontend-only persistence.

- **决策**：通过 Tauri commands 暴露基金查询与列表操作。
- **理由**：业务逻辑保持在 Rust 模块，符合架构与测试要求。
- **备选**：仅在前端实现持久化。

### Decision 5: List auto-refresh
### 决策 5：列表自动刷新

- **Decision**: Auto-refresh list fund data every N minutes (default 3, configurable 1–5).
- **Rationale**: Meets product requirement for timely fund changes without manual action.
- **Alternatives considered**: Manual refresh only; active-list-only refresh.

- **决策**：列表基金数据每 N 分钟自动刷新（默认 3 分钟，可配置 1–5）。
- **理由**：满足“查看涨跌”实时性需求，减少手动操作。
- **备选**：仅手动刷新；仅当前列表自动刷新。
