# Feature Specification: Fund List Management
# 功能规格：基金列表管理

**Feature Branch**: `001-fund-tracker-client`  
**Created**: 2026-01-21  
**Status**: Draft  
**Input**: User description: "我要做一个客户端，rust + tauri，现在 macos 系统上开发调试，客户端是用来查看基金的涨跌情况， specs/001-fund-list-management/spec.md 跟这个里面一样的功能 specs/001-fund-list-management/README.md"

**分支**：`001-fund-tracker-client`  
**创建日期**：2026-01-21  
**状态**：草稿  
**输入**：用户描述：“我要做一个客户端，rust + tauri，现在 macos 系统上开发调试，客户端是用来查看基金的涨跌情况， specs/001-fund-list-management/spec.md 跟这个里面一样的功能 specs/001-fund-list-management/README.md”

## Clarifications
## 澄清

### Session 2026-01-21
### 会话 2026-01-21

- Q: How should list fund data refresh? → A: Auto-refresh list fund data every N minutes (e.g., 1–5 min).
- 问：列表基金数据如何刷新？→ 答：每 N 分钟自动刷新（如 1–5 分钟）。

## User Scenarios & Testing *(mandatory)*
## 用户场景与测试（必需）

### User Story 1 - Search and View Fund Info (Priority: P1)
### 用户故事 1 - 搜索并查看基金信息（优先级 P1）

User searches by fund code to see current fund details and recent change.

用户通过基金代码搜索，查看当前基金详情与涨跌信息。

**Why this priority**: Core value is being able to quickly check fund performance.

**优先级原因**：核心价值是快速查看基金表现。

**Independent Test**: Can be fully tested by entering valid/invalid fund codes and
verifying the displayed information and errors.

**独立测试**：输入有效/无效基金代码，验证展示信息与错误提示。

**Acceptance Scenarios**:

1. **Given** the app is open, **When** a user enters a valid 6-digit fund code,
   **Then** the fund name, current net value, change percentage, and update time
   are displayed.
2. **Given** the app is open, **When** a user enters an invalid or unknown code,
   **Then** a clear error message is shown and no stale results remain visible.

**验收场景**：

1. **Given** 应用已打开，**When** 输入有效 6 位基金代码，**Then** 显示基金名称、当前净值、涨跌幅、更新时间。
2. **Given** 应用已打开，**When** 输入无效或未知代码，**Then** 显示清晰错误提示且不保留旧结果。

---

### User Story 2 - Organize Funds into Lists (Priority: P2)
### 用户故事 2 - 基金列表管理（优先级 P2）

User creates lists and adds/removes funds to keep track of different categories.

用户创建列表并添加/移除基金，用于分类管理。

**Why this priority**: Organization allows users to manage multiple funds and
compare performance by groups.

**优先级原因**：组织能力让用户管理多只基金并按组对比。

**Independent Test**: Can be fully tested by creating a list, adding a fund,
preventing duplicates, and removing a fund.

**独立测试**：创建列表、添加基金、阻止重复、移除基金。

**Acceptance Scenarios**:

1. **Given** a fund is displayed, **When** the user adds it to a chosen list,
   **Then** the fund appears in that list and duplicates are prevented.
2. **Given** a list contains funds, **When** the user removes a fund,
   **Then** it is removed from that list only.

**验收场景**：

1. **Given** 已显示基金信息，**When** 添加到指定列表，**Then** 出现在列表中且不允许重复。
2. **Given** 列表包含基金，**When** 移除某基金，**Then** 仅从该列表移除。

---

### User Story 3 - Persist User Data (Priority: P3)
### 用户故事 3 - 数据持久化（优先级 P3）

User data is retained across app restarts without manual saving.

用户数据在应用重启后保留，无需手动保存。

**Why this priority**: Users expect their lists and funds to remain after closing
and reopening the app.

**优先级原因**：用户期望关闭后再打开仍保留列表与基金。

**Independent Test**: Can be fully tested by creating lists, closing the app,
reopening, and verifying data is intact.

**独立测试**：创建列表、关闭应用、重启后验证数据完整。

**Acceptance Scenarios**:

1. **Given** the user has created lists with funds, **When** the app is closed and
   reopened, **Then** all lists and memberships are restored.

**验收场景**：

1. **Given** 用户已创建含基金的列表，**When** 关闭并重新打开应用，**Then** 所有列表与成员恢复。

---

### Edge Cases
### 边界情况

- What happens when the network is unavailable during a fund query?
- How does the system handle a list name conflict?
- What happens when the user tries to add the same fund to the same list twice?
- How does the system respond to corrupted or unreadable saved data?
- What happens when list auto-refresh fails due to network errors?

- 基金查询期间网络不可用时如何处理？
- 列表名称冲突如何处理？
- 同一基金重复添加到同一列表时如何处理？
- 本地存储数据损坏或无法读取时如何处理？
- 列表自动刷新因网络失败时如何处理？

## Requirements *(mandatory)*
## 需求（必需）

### Functional Requirements
### 功能需求

- **FR-001**: System MUST accept 6-digit fund codes and return a matching fund or
  a clear error.
- **FR-002**: System MUST display fund name, current net value, change percentage,
  and update time for a successful query.
- **FR-003**: Users MUST be able to create, rename, and delete fund lists.
- **FR-004**: System MUST prevent duplicate fund entries within the same list.
- **FR-005**: Users MUST be able to add and remove funds within a list.
- **FR-006**: System MUST preserve user lists and memberships across app restarts
  without manual save actions.
- **FR-007**: System MUST provide clear, user-friendly error messages for invalid
  codes and network failures.
- **FR-008**: System MUST auto-refresh fund data in lists every N minutes
  (default 3 minutes, configurable 1–5 minutes).

- **FR-001**：系统必须接受 6 位基金代码并返回匹配基金或清晰错误。
- **FR-002**：成功查询后必须显示基金名称、当前净值、涨跌幅和更新时间。
- **FR-003**：用户必须能够创建、重命名、删除基金列表。
- **FR-004**：系统必须阻止同一列表内的重复基金。
- **FR-005**：用户必须能在列表中添加和移除基金。
- **FR-006**：系统必须在应用重启后自动恢复列表与成员数据。
- **FR-007**：无效代码或网络失败时必须给出清晰友好的错误提示。
- **FR-008**：系统必须按 N 分钟自动刷新列表基金数据（默认 3 分钟，可配置 1–5 分钟）。

### Functional Requirement Acceptance Criteria
### 功能需求验收标准

- **FR-001**: A valid code returns a matching fund; an invalid code yields a clear
  error without stale results.
- **FR-002**: Fund name, net value, change percentage, and update time are visible
  after a successful query.
- **FR-003**: Users can create, rename, and delete lists from the list interface.
- **FR-004**: Adding the same fund to the same list twice is blocked with a clear
  message.
- **FR-005**: Funds can be added to and removed from a list without affecting
  other lists.
- **FR-006**: Lists and memberships remain intact after closing and reopening the
  app.
- **FR-007**: Errors for invalid codes or network issues are shown in clear,
  user-facing language.
- **FR-008**: List fund data refreshes automatically within the configured
  interval and indicates last refresh time.

- **FR-001**：有效代码返回基金；无效代码提示清晰且不保留旧结果。
- **FR-002**：成功查询后可见基金名称、净值、涨跌幅、更新时间。
- **FR-003**：用户可在列表界面创建、重命名、删除列表。
- **FR-004**：重复添加同一基金时被阻止并提示原因。
- **FR-005**：添加/移除基金不影响其他列表。
- **FR-006**：关闭并重启后列表与成员仍完整。
- **FR-007**：无效代码或网络问题显示清晰可理解的错误。
- **FR-008**：列表基金数据按配置间隔自动刷新，并显示最近刷新时间。

### Constitution Constraints *(mandatory)*
### 宪法约束（必需）

- 数据源、隐私、跨平台一致性、测试要求等 MUST 与 `.specify/memory/constitution.md` 保持一致。
- 若某条约束无法满足，必须给出替代方案与明确理由。

- 数据源、隐私、跨平台一致性、测试要求等必须与 `.specify/memory/constitution.md` 保持一致。
- 如有无法满足的约束，必须给出替代方案与明确理由。

### Key Entities *(include if feature involves data)*
### 关键实体（涉及数据时必填）

- **Fund**: A mutual fund identified by a 6-digit code with name, net value,
  change percentage, and update time.
- **Fund List**: A user-defined collection of funds with a unique name and an
  ordered set of fund members.
- **User Data**: All lists, memberships, and preferences stored for the user.

- **Fund**：以 6 位代码标识的基金，包含名称、净值、涨跌幅、更新时间。
- **Fund List**：用户自定义列表，包含唯一名称与基金成员集合。
- **User Data**：用户保存的所有列表、成员与偏好。

## Success Criteria *(mandatory)*
## 成功标准（必需）

### Measurable Outcomes
### 可量化结果

- **SC-001**: Users can search a valid fund and see results within 3 seconds in
  95% of attempts under normal network conditions.
- **SC-002**: Users can add a fund to a list in under 10 seconds end-to-end.
- **SC-003**: 100% of saved lists and memberships are restored after app restart
  in a 30-day usage period.
- **SC-004**: Duplicate additions to the same list are prevented in 100% of
  attempts.
- **SC-005**: At least 90% of users can complete the main flow (search + add to
  list) without errors in usability testing.

- **SC-001**：正常网络下 95% 查询在 3 秒内返回。
- **SC-002**：用户可在 10 秒内完成“搜索+加入列表”。
- **SC-003**：30 天使用期内重启后 100% 恢复列表与成员。
- **SC-004**：100% 阻止同一列表内的重复添加。
- **SC-005**：可用性测试中至少 90% 用户无错误完成主流程。

## Assumptions
## 假设

- Fund codes follow the standard 6-digit format used by Chinese mutual funds.
- Users manage a moderate number of lists and funds (e.g., under 50 lists and
  under 200 funds per list) in typical use.
- The app can access an approved fund data source as defined in the project
  constitution.

- 基金代码遵循中国基金 6 位标准格式。
- 典型使用下，列表数量小于 50，每个列表基金少于 200。
- 应用可访问宪法中批准的数据源。

## Out of Scope
## 范围外

- Portfolio valuation,收益计算,或持仓分析
- 基金历史图表与技术分析
- 用户登录、云同步或多设备共享
- 基金交易、申购或赎回功能
- 推送通知或价格提醒

- 组合估值、收益计算或持仓分析
- 基金历史图表与技术分析
- 用户登录、云同步或多设备共享
- 基金交易、申购或赎回功能
- 推送通知或价格提醒
