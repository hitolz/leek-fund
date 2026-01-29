# Feature Specification: Client UI Redesign From Demo / 参照演示的客户端界面重设计

**Feature Branch**: `010-client-ui-redesign`  
**Created**: 2026-01-27  
**Status**: Draft  
**Input**: User description: "用当前 010 分支，按照 demo 里的页面及样式，重做客户端页面"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Demo-Style Core Flow / 演示风格主流程 (Priority: P1)

A fund-tracking user can complete the primary workflow in the redesigned client page that matches the demo’s layout and styling: select a list, view its funds, select a fund, and see its details.  
基金跟踪用户可以在符合 demo 布局与样式的重设计客户端页面中完成主流程：选择列表、查看该列表基金、选择基金并查看详情。

**Why this priority**: The redesign is only valuable if the core workflow remains usable and clear in the new visual structure.  
**Why this priority（原因）**: 只有当主流程在新视觉结构中依旧清晰可用时，重设计才有价值。

**Independent Test**: Launch the client, select two different lists and two different funds, and verify the fund list and detail panel update correctly in the demo-style layout.  
**Independent Test（独立测试）**: 启动客户端，选择两个不同列表与两只不同基金，确认基金列表与详情面板在 demo 风格布局下正确更新。

**Acceptance Scenarios**:

1. **Given** the user is on the redesigned page with at least one list and fund, **When** the user selects a list and then a fund, **Then** the list panel, fund list, and detail panel show consistent selection state.  
   **Given** 用户处于重设计页面且至少存在一个列表与基金，**When** 用户选择列表再选择基金，**Then** 列表面板、基金列表与详情面板的选择状态保持一致。
2. **Given** a list and fund are selected, **When** the user hides and shows the list panel, **Then** the current selections are preserved.  
   **Given** 已选中某列表与基金，**When** 用户隐藏再显示列表面板，**Then** 当前选择保持不变。

---

### User Story 2 - Manage Funds and Holdings / 管理基金与持仓 (Priority: P2)

A user can add and remove funds, edit holding amount and holding shares, and see derived cost and daily change amount while using the redesigned demo-style UI.  
用户可以在重设计的 demo 风格界面中添加与删除基金、编辑持仓金额与份额，并查看每份成本与当日涨跌额等推导结果。

**Why this priority**: Fund management and holding calculations are the most important interactions after the core navigation flow and must remain accurate.  
**Why this priority（原因）**: 基金管理与持仓计算是主流程之后最重要的交互，必须保持准确。

**Independent Test**: Add a fund, remove a fund, update holding amount and shares for a selected fund, and confirm derived values update correctly.  
**Independent Test（独立测试）**: 添加基金、删除基金，并更新选中基金的持仓金额与份额，确认推导值正确更新。

**Acceptance Scenarios**:

1. **Given** a list is active, **When** the user adds a valid fund and then removes it, **Then** the fund appears and disappears only in that list.  
   **Given** 某列表已激活，**When** 用户添加一个有效基金并删除，**Then** 该基金仅在该列表中出现与消失。
2. **Given** a fund is selected, **When** the user edits holding amount and holding shares, **Then** cost per share and daily change amount update to match the documented formulas.  
   **Given** 已选中基金，**When** 用户编辑持仓金额与份额，**Then** 每份成本与当日涨跌额更新为与文档公式一致的结果。

---

### User Story 3 - Sorting and Comparison / 排序与对比 (Priority: P3)

A user can sort the fund list by daily change percent, daily change amount, or holding amount, cycle between descending, ascending, and no sorting, and clearly understand the current sorting state in the redesigned UI.  
用户可以在重设计界面中按当日涨跌幅、当日涨跌额或持仓金额排序，在降序、升序与不排序之间切换，并清晰理解当前排序状态。

**Why this priority**: Sorting improves decision-making but depends on the stability of list data and holding calculations.  
**Why this priority（原因）**: 排序能提升决策效率，但依赖列表数据与持仓计算的稳定性。

**Independent Test**: Apply each sort field across all three sort states and confirm the order changes correctly and resets to default order when sorting is off.  
**Independent Test（独立测试）**: 对每个排序字段依次应用三种排序状态，确认顺序变化正确，且不排序时恢复默认顺序。

**Acceptance Scenarios**:

1. **Given** a list with multiple funds, **When** the user cycles a sort field through descending and ascending, **Then** the visible order matches the selected direction.  
   **Given** 列表包含多只基金，**When** 用户将某排序字段在降序与升序之间切换，**Then** 可见顺序与所选方向一致。
2. **Given** a sort is active, **When** the user switches to no sorting, **Then** the fund list restores the preserved default order.  
   **Given** 当前已应用排序，**When** 用户切换为不排序，**Then** 基金列表恢复保留的默认顺序。

### Edge Cases

- When no lists exist, the page shows a clear empty state and disables fund-level actions.  
  当不存在任何列表时，页面展示清晰空状态并禁用基金层级操作。
- When a selected list has no funds, the fund list shows an empty state and the detail panel shows a neutral placeholder.  
  当选中列表没有基金时，基金列表展示空状态，详情面板展示中性占位。
- When a selected fund is removed, selection moves to the next available fund in the list or clears if none remain.  
  当已选基金被删除时，选择会移动到该列表下一个可用基金，若无则清空。
- When holding shares is zero or missing, cost per share is not displayed as a numeric value and the UI explains why.  
  当持仓份额为 0 或缺失时，每份成本不显示为数值，并在界面中解释原因。
- When sorting produces tied values, the order remains stable relative to the default order.  
  当排序出现相同值时，顺序相对默认顺序保持稳定。
- When the window is narrow, the primary workflow remains accessible without hiding critical controls.  
  当窗口变窄时，主流程仍可完成且不会隐藏关键控件。

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST deliver a redesigned client page whose layout and visual style align with the demo page in the `demo` folder.  
  **FR-001（功能需求）**: 系统必须交付一个与 `demo` 文件夹中的演示页面在布局与视觉风格上保持一致的客户端重设计页面。
- **FR-002**: The redesign MUST preserve existing core behaviors: list selection, fund selection, add/remove, holding inputs, derived metrics, and sorting.  
  **FR-002（功能需求）**: 重设计必须保留既有核心行为：列表选择、基金选择、增删、持仓输入、推导指标与排序。
- **FR-003**: The system MUST allow the list panel to be hidden or shown without losing current selections.  
  **FR-003（功能需求）**: 系统必须允许列表面板显隐切换且不丢失当前选择。
- **FR-004**: The system MUST allow exactly one active list at a time and show only funds in that list.  
  **FR-004（功能需求）**: 系统必须保证任意时刻只有一个激活列表，并仅展示该列表的基金。
- **FR-005**: The system MUST allow selecting a fund and clearly indicate the selected item in the redesigned list.  
  **FR-005（功能需求）**: 系统必须允许选择基金，并在重设计列表中清晰标识选中项。
- **FR-006**: The system MUST display a detail view that includes fund identity, key daily indicators, and a visible trend chart area.  
  **FR-006（功能需求）**: 系统必须展示包含基金身份、关键当日指标与可见走势图区域的详情视图。
- **FR-007**: The system MUST allow adding a fund to the active list using a valid fund code and display name, and MUST prevent duplicates within a list.  
  **FR-007（功能需求）**: 系统必须允许使用有效基金代码与名称添加基金，并在同一列表内防止重复。
- **FR-008**: The system MUST allow removing a fund from the active list and MUST update selection safely after removal.  
  **FR-008（功能需求）**: 系统必须允许从激活列表删除基金，并在删除后安全更新选中状态。
- **FR-009**: The system MUST allow entering holding amount and holding shares for a selected fund.  
  **FR-009（功能需求）**: 系统必须允许为选中基金输入持仓金额与持仓份额。
- **FR-010**: The system MUST show cost per share and daily change amount derived from holdings and daily change percent, consistent with current product rules.  
  **FR-010（功能需求）**: 系统必须展示由持仓与当日涨跌幅推导的每份成本与当日涨跌额，并与当前产品规则一致。
- **FR-011**: The system MUST provide sorting controls for daily change percent, daily change amount, and holding amount.  
  **FR-011（功能需求）**: 系统必须提供当日涨跌幅、当日涨跌额与持仓金额的排序控件。
- **FR-012**: The system MUST support descending, ascending, and no-sorting states for each sorting field.  
  **FR-012（功能需求）**: 系统必须为每个排序字段支持降序、升序与不排序三种状态。
- **FR-013**: When no sorting is selected, the system MUST restore the preserved default order.  
  **FR-013（功能需求）**: 当不排序时，系统必须恢复保留的默认顺序。
- **FR-014**: Sorting MUST be stable for tied values and MUST keep relative order consistent with the default order.  
  **FR-014（功能需求）**: 排序在相同值时必须保持稳定，并维持与默认顺序一致的相对顺序。
- **FR-015**: The redesigned page MUST provide clear empty, loading, and error states that guide the next user action.  
  **FR-015（功能需求）**: 重设计页面必须提供清晰的空、加载与错误状态，指引用户下一步操作。
- **FR-016**: The redesign MUST not require changes to data ownership boundaries or persistence behavior.  
  **FR-016（功能需求）**: 重设计不得要求改变数据归属边界或持久化行为。

## Assumptions / 假设

- The demo page is the visual and interaction reference; pixel-perfect matching is not required, but key structure and styling cues must be recognizable.  
  demo 页面是视觉与交互参考；不要求像素级一致，但关键结构与样式线索必须可辨识。
- Existing backend rules, data fetching, and persistence remain the source of truth; the redesign focuses on client presentation and interaction flow.  
  既有后端规则、数据获取与持久化仍是事实来源；重设计仅聚焦客户端展示与交互流程。
- Calculation formulas and validation rules are reused from current behavior and are not redefined by this redesign.  
  计算公式与校验规则沿用当前行为，本次重设计不重新定义。
- The redesign is considered successful if users can complete the same workflows with equal or better clarity compared to the demo reference.  
  当用户能以不低于 demo 参考的清晰度完成同样流程时，即视为成功。

### Key Entities *(include if feature involves data)*

- **Fund List / 基金列表**: A named collection that defines the active context and preserves default order.  
  **Fund List / 基金列表（实体说明）**: 定义激活上下文并保留默认顺序的命名集合。
- **Fund / 基金**: A tracked fund with code, name, daily change percent, and trend data for details.  
  **Fund / 基金（实体说明）**: 包含代码、名称、当日涨跌幅与详情趋势数据的被跟踪基金。
- **Holding / 持仓**: User-provided holding amount and shares with derived cost and daily change amount.  
  **Holding / 持仓（实体说明）**: 用户输入的持仓金额与份额，并可推导成本与当日涨跌额。
- **Sort Preference / 排序偏好**: The current sort field and direction applied to the visible fund list.  
  **Sort Preference / 排序偏好（实体说明）**: 当前应用于可见基金列表的排序字段与方向。
- **View Layout State / 视图布局状态**: The UI state for list panel visibility and current list/fund selection.  
  **View Layout State / 视图布局状态（实体说明）**: 列表面板显隐与当前列表/基金选择的界面状态。

## Constitution Alignment *(mandatory)*

- **Tauri Desktop Architecture**: The redesign stays within the existing desktop application scope and only updates the client UI.  
  / **Tauri 桌面架构**: 重设计保持在既有桌面应用范围内，仅更新客户端界面。
- **Rust Owns Data & Network**: Data and network ownership remain unchanged; the UI redesign does not move these responsibilities.  
  / **Rust 管理数据与网络**: 数据与网络归属保持不变，UI 重设计不转移这些职责。
- **UI-Only Frontend**: The feature focuses on presentation and interaction while respecting current business rules and validation sources.  
  / **仅 UI 前端**: 功能聚焦展示与交互，同时尊重当前业务规则与校验来源。
- **Local-First Persistence & Recovery**: No changes to persistence paths, migrations, or recovery expectations are required.  
  / **本地优先持久化与恢复**: 不要求改变持久化路径、迁移或恢复预期。
- **Fund List Semantics & Data Integrity**: List scoping, uniqueness, and deterministic calculations remain consistent with existing behavior.  
  / **基金列表语义与数据完整性**: 列表作用域、去重与确定性计算保持与现有行为一致。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can complete the primary workflow in the redesigned page within 60 seconds during a guided walkthrough.  
  **SC-001（可衡量结果）**: 在引导式走查中，用户可在 60 秒内完成主流程。
- **SC-002**: In review sessions using representative data, all core behaviors from the current client are present and functional in the redesigned page.  
  **SC-002（可衡量结果）**: 在使用代表性数据的评审中，当前客户端的所有核心行为在重设计页面中可见且可用。
- **SC-003**: For lists with at least 30 funds, sorting completes with visibly correct order and stable no-sorting restoration within 2 seconds.  
  **SC-003（可衡量结果）**: 对于至少 30 只基金的列表，排序在 2 秒内完成并呈现正确顺序，且不排序时稳定恢复默认顺序。
- **SC-004**: In stakeholder evaluation, the redesigned page is rated at least as clear as the demo reference for understanding selection and change direction in 90% of tasks.  
  **SC-004（可衡量结果）**: 在干系人评估中，重设计页面在 90% 的任务上被评为在理解选择与涨跌方向方面不逊于 demo 参考。
