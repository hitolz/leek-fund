# Feature Specification / 功能规格: Fund Detail Three-Column Layout

**Feature Branch**: `002-fund-detail-layout`  
**功能分支**：`002-fund-detail-layout`  
**Created**: 2026-01-21  
**创建日期**：2026-01-21  
**Status**: Draft  
**状态**：草稿  
**Input**: User description: "增加功能，调整页面展示，左侧为列表，中间一列为该列表点击后展示列表里的所有基金列表，展示 code + 名字及当天的 涨跌情况，右侧还增加一列，展示点击了某一个基金后展示该基金的具体信息，走势图"  
**输入**：用户描述：“增加功能，调整页面展示，左侧为列表，中间一列为该列表点击后展示列表里的所有基金列表，展示 code + 名字及当天的 涨跌情况，右侧还增加一列，展示点击了某一个基金后展示该基金的具体信息，走势图”

## User Scenarios & Testing *(mandatory)* / 用户场景与测试（必填）

### User Story 1 - Browse Lists and Funds (Priority: P1) / 用户故事 1 - 浏览列表与基金（优先级：P1）

As a user, I want a three-column layout where I can pick a list on the left and immediately see all funds in that list in the middle.  
作为用户，我希望通过三列布局，在左侧选择列表后，中间立即显示该列表的所有基金。

**Why this priority**: This is the primary navigation flow for finding a fund quickly.  
**优先级理由**：这是用户快速找到基金的核心路径。

**Independent Test**: Select any list and verify the middle column updates to show only that list’s funds.  
**独立测试**：选择任意列表，验证中间列仅显示该列表基金。

**Acceptance Scenarios** / **验收场景**：

1. **Given** multiple lists exist, **When** I click a list in the left column, **Then** the middle column shows only funds from that list.  
   **假设** 存在多个列表，**当** 我点击左侧某列表，**则** 中间列仅显示该列表基金。
2. **Given** a list has no funds, **When** I select it, **Then** the middle column shows an empty-state message for that list.  
   **假设** 某列表为空，**当** 我选中该列表，**则** 中间列显示该列表的空态提示。

---

### User Story 2 - See Daily Change at a Glance (Priority: P2) / 用户故事 2 - 一眼看到当日涨跌（优先级：P2）

As a user, I want each fund in the middle column to show code, name, and its daily change so I can compare performance without opening details.  
作为用户，我希望中间列每个基金显示代码、名称和当日涨跌，以便无需打开详情就能比较表现。

**Why this priority**: It enables quick scanning and comparison across many funds.  
**优先级理由**：支持快速扫视与横向对比。

**Independent Test**: Open any list and verify each fund row shows code, name, and daily change.  
**独立测试**：打开任意列表，验证每行基金显示代码、名称与当日涨跌。

**Acceptance Scenarios** / **验收场景**：

1. **Given** a list with multiple funds, **When** the middle column renders, **Then** each fund row shows code, name, and daily change for the latest available trading day.  
   **假设** 列表包含多个基金，**当** 中间列渲染时，**则** 每行显示代码、名称与最新交易日的涨跌信息。

---

### User Story 3 - View Fund Details and Trend (Priority: P3) / 用户故事 3 - 查看基金详情与走势（优先级：P3）

As a user, I want to click a fund and see its detailed information and a trend chart in the right column.  
作为用户，我希望点击基金后在右侧看到该基金的详细信息与走势图。

**Why this priority**: It supports deeper decision-making once a fund is selected.  
**优先级理由**：在选定基金后支持更深度的判断。

**Independent Test**: Click a fund in the middle column and verify the right column shows its details and chart.  
**独立测试**：点击中间列基金，验证右侧显示详情与图表。

**Acceptance Scenarios** / **验收场景**：

1. **Given** I have selected a list, **When** I click a fund in the middle column, **Then** the right column shows that fund’s details.  
   **假设** 已选择列表，**当** 我点击中间列某基金，**则** 右侧显示该基金详情。
2. **Given** a fund has no trend data, **When** I open its details, **Then** the right column shows a clear message that trend data is unavailable.  
   **假设** 某基金无走势数据，**当** 我打开详情，**则** 右侧显示走势数据不可用的提示。

### Edge Cases / 边界情况

- What happens when there are no lists at all? / 如果没有任何列表会怎样？
- How does the system handle missing daily-change data for a fund? / 若基金缺少当日涨跌数据如何处理？
- What happens when the selected list is deleted while it is active? / 正在查看的列表被删除时如何处理？

## Requirements *(mandatory)* / 需求（必填）

### Functional Requirements / 功能性需求

- **FR-001**: The system MUST present a three-column layout: lists on the left, list funds in the middle, and fund details on the right.  
  **FR-001**：系统必须提供三列布局：左侧列表、中间基金列表、右侧基金详情。
- **FR-002**: Selecting a list MUST update the middle column to show only the funds in that list.  
  **FR-002**：选择列表后必须更新中间列，仅显示该列表的基金。
- **FR-003**: Each fund row in the middle column MUST display the fund’s code, name, and daily change for the latest available trading day.  
  **FR-003**：中间列每个基金行必须显示代码、名称与最新交易日的涨跌信息。
- **FR-004**: Selecting a fund MUST update the right column to show that fund’s detailed information and a trend chart.  
  **FR-004**：选择基金后必须更新右侧显示基金详情与走势图。
- **FR-005**: The system MUST provide empty-state messaging when there are no lists, when a list has no funds, or when a fund has no trend data.  
  **FR-005**：在无列表、列表无基金或基金无走势数据时必须提供空态提示。
- **FR-006**: If the currently selected list becomes unavailable, the system MUST reset the middle and right columns to a safe default state.  
  **FR-006**：当前列表不可用时，系统必须将中间列与右侧列重置到安全默认状态。

### Constitution Constraints *(mandatory)* / 宪法约束（必填）

- 数据源、隐私、跨平台一致性、测试要求等 MUST 与 `.specify/memory/constitution.md` 保持一致。  
  Data source, privacy, cross-platform consistency, and testing requirements MUST align with `.specify/memory/constitution.md`.
- 若某条约束无法满足，必须给出替代方案与明确理由。  
  If any constraint cannot be met, an alternative with explicit rationale MUST be provided.

### Key Entities *(include if feature involves data)* / 关键实体（如涉及数据）

- **Fund List**: A user-defined collection of funds that can be selected to scope the middle column.  
  **基金列表**：用户定义的基金集合，用于限定中间列范围。
- **Fund**: An individual fund with code, name, daily change, and detail information.  
  **基金**：包含代码、名称、当日涨跌与详情信息的单个基金。
- **Fund Trend**: A recent performance series used to render the right-column chart.  
  **基金走势**：用于右侧图表的近期表现序列。

### Assumptions / 假设

- The daily change value reflects the latest available trading day.  
  当日涨跌值以最新可用交易日为准。
- The trend chart displays recent performance over a standard recent window (e.g., last 30 trading days) when data exists.  
  走势图展示固定近期窗口（例如最近 30 个交易日），有数据时才显示。

## Success Criteria *(mandatory)* / 成功标准（必填）

### Measurable Outcomes / 可衡量结果

- **SC-001**: Users can navigate from list selection to viewing a fund’s details in 30 seconds or less.  
  **SC-001**：用户在 30 秒内可从选择列表切换到查看某基金详情。
- **SC-002**: At least 90% of users can identify the daily change of a fund without opening its details.  
  **SC-002**：至少 90% 的用户无需打开详情即可识别基金当日涨跌。
- **SC-003**: The middle column updates to the selected list within 1 second in typical usage.  
  **SC-003**：典型使用场景下中间列在 1 秒内更新为所选列表。
- **SC-004**: Fewer than 5% of sessions encounter a blank right column when a fund is selected.  
  **SC-004**：基金被选中时，出现右侧空白面板的会话占比低于 5%。
