# Feature Specification: Fund Demo UI Redesign / 基金演示页面重设计

**Feature Branch**: `009-fund-ui-demo`  
**Created**: 2026-01-27  
**Status**: Draft  
**Input**: User description: "新建一个 demo 文件夹，重新设计一下前端页面，功能与现有一致，列表可以隐藏，选中列表之后展示基金列表，基金可以添加和删除，点击基金后显示基金详情，走势图，可以设置持仓金额、持仓份额，计算成本，计算当日涨跌额，基金列表支持排序(当日涨跌幅、当日涨跌额、持仓金额)并可以 降序、升序、不排序 只生成 html 演示页面就行，"

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.
  
  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - Manage Lists and Funds / 管理列表与基金 (Priority: P1)

A user can hide or show the fund list panel, select a list, and see the corresponding funds. The user can add a fund to the selected list and remove a fund from it.  
用户可以隐藏或显示基金列表面板，选择一个列表，并查看该列表对应的基金。用户可以向已选列表添加基金，也可以从中删除基金。

**Why this priority**: Without list selection and fund management, the demo cannot represent the core workflow of organizing and maintaining tracked funds.  
**Why this priority（原因）**: 如果没有列表选择与基金管理，演示页面就无法体现“组织与维护关注基金”的核心流程。

**Independent Test**: This story can be tested by toggling the list panel, selecting a list, adding a fund, and deleting it, confirming the visible fund set updates correctly.  
**Independent Test（独立测试）**: 通过切换列表面板显隐、选择列表、添加基金、删除基金，并确认可见的基金集合正确更新，即可独立验证该故事。

**Acceptance Scenarios**:

1. **Given** the list panel is visible, **When** the user hides it and then shows it again, **Then** the selected list and its visible funds remain unchanged.  
   **Given** 列表面板可见，**When** 用户将其隐藏后再显示，**Then** 当前选中的列表与可见基金保持不变。
2. **Given** at least one list exists, **When** the user selects a different list, **Then** the fund list updates to show only funds belonging to that list.  
   **Given** 至少存在一个列表，**When** 用户选择另一个列表，**Then** 基金列表仅展示该列表下的基金。
3. **Given** a list is selected, **When** the user adds a new fund using a valid identifier, **Then** the fund appears in the selected list.  
   **Given** 已选择一个列表，**When** 用户使用有效标识添加新基金，**Then** 该基金出现在当前列表中。
4. **Given** a fund is present in the selected list, **When** the user removes it, **Then** it no longer appears in that list and is not counted in its totals.  
   **Given** 当前列表中存在某基金，**When** 用户删除它，**Then** 该基金不再显示，且不再计入该列表的汇总。

---

### User Story 2 - View Details and Holdings / 查看详情与持仓 (Priority: P2)

When a user selects a fund, the demo shows fund details and a trend chart. The user can set holding amount and holding shares, and the demo calculates cost per share and daily change amount.  
当用户选中某只基金时，演示页面显示基金详情与走势图。用户可以设置持仓金额与持仓份额，页面计算每份成本与当日涨跌额。

**Why this priority**: Fund detail review and holding calculations provide the key value of understanding position performance beyond simple list browsing.  
**Why this priority（原因）**: 相比仅浏览列表，详情查看与持仓计算能体现“理解持仓表现”的关键价值。

**Independent Test**: This story can be tested by selecting a fund, editing holding amount and shares, and verifying that cost per share and daily change amount update immediately and correctly.  
**Independent Test（独立测试）**: 选择一只基金，修改持仓金额与份额，并验证每份成本与当日涨跌额能够立即且正确更新，即可独立验证该故事。

**Acceptance Scenarios**:

1. **Given** a fund is selected, **When** the detail panel loads, **Then** the fund name, key daily indicators, and a visible trend chart are shown together.  
   **Given** 已选中某基金，**When** 详情面板加载完成，**Then** 同时展示基金名称、关键当日指标与清晰可见的走势图。
2. **Given** a selected fund has a daily change percentage, **When** the user enters holding amount and holding shares, **Then** the demo displays cost per share and daily change amount based on those inputs.  
   **Given** 已选基金包含当日涨跌幅，**When** 用户输入持仓金额与持仓份额，**Then** 页面基于这些输入展示每份成本与当日涨跌额。

---

### User Story 3 - Sort Funds / 基金排序 (Priority: P3)

A user can sort the fund list by daily change percentage, daily change amount, or holding amount, and can switch between descending, ascending, and no sorting.  
用户可以按当日涨跌幅、当日涨跌额或持仓金额对基金列表排序，并能在降序、升序与不排序之间切换。

**Why this priority**: Sorting improves decision-making but depends on the underlying list, detail, and holding calculations being available first.  
**Why this priority（原因）**: 排序能提升决策效率，但其价值建立在“列表可用、详情可看、持仓可算”的基础之上。

**Independent Test**: This story can be tested by applying each sort mode and verifying the visible order changes correctly and can be reset to the default order.  
**Independent Test（独立测试）**: 依次应用各排序方式并验证可见顺序正确变化，且能恢复为默认顺序，即可独立验证该故事。

**Acceptance Scenarios**:

1. **Given** a fund list with at least three funds, **When** the user selects “daily change percentage” and sets descending order, **Then** funds appear from the largest percentage to the smallest.  
   **Given** 基金列表至少包含三只基金，**When** 用户选择“当日涨跌幅”并设置为降序，**Then** 基金从涨幅最大到最小排列。
2. **Given** a sort is currently applied, **When** the user chooses “no sorting,” **Then** the list returns to its default order for the selected list.  
   **Given** 当前已应用排序，**When** 用户选择“不排序”，**Then** 列表恢复为该列表的默认顺序。

---

### Edge Cases

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right edge cases.
-->

- When no lists exist, the demo shows an empty-state prompt and disables fund-level actions until a list is available.  
  当没有任何列表时，页面展示空状态提示，并在存在可用列表前禁用基金层级的操作。
- When a selected list contains no funds, the fund list shows an empty state and the detail panel shows a neutral placeholder.  
  当选中列表没有基金时，基金列表展示空状态，详情面板展示中性占位内容。
- When holding shares is zero or missing, cost per share is not shown as a numeric value and the UI explains that shares must be greater than zero.  
  当持仓份额为 0 或缺失时，每份成本不展示为数值，并提示份额必须大于 0。
- When the selected fund is deleted, the selection clears or moves to the next available fund, and the detail panel updates accordingly.  
  当被选中的基金被删除时，选中状态会清空或移动到下一个可用基金，详情面板随之更新。
- When sorting is applied to data with ties, funds with equal values keep their relative default order.  
  当排序字段存在相同数值时，相同项之间保持默认顺序的相对稳定性。

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: The system MUST provide a dedicated demo page located under a new `demo` folder and present the full workflow within a single HTML page.  
  **FR-001（功能需求）**: 系统必须在新的 `demo` 文件夹下提供专用演示页面，并在单个 HTML 页面内呈现完整流程。
- **FR-002**: The system MUST allow the user to hide and show the left-side list panel without losing the current list selection.  
  **FR-002（功能需求）**: 系统必须允许用户隐藏或显示左侧列表面板，且不会丢失当前列表选择。
- **FR-003**: The system MUST display available lists and allow the user to select exactly one active list at a time.  
  **FR-003（功能需求）**: 系统必须展示可用列表，并允许用户在任意时刻仅选择一个激活列表。
- **FR-004**: When a list is selected, the system MUST show only the funds that belong to that list.  
  **FR-004（功能需求）**: 当选择某个列表时，系统必须仅展示属于该列表的基金。
- **FR-005**: The system MUST allow the user to add a fund to the currently selected list using a fund identifier and a display name.  
  **FR-005（功能需求）**: 系统必须允许用户使用基金标识与显示名称，向当前选中列表添加基金。
- **FR-006**: The system MUST allow the user to remove a fund from the currently selected list.  
  **FR-006（功能需求）**: 系统必须允许用户从当前选中列表删除基金。
- **FR-007**: The system MUST allow the user to select a fund from the visible fund list, and MUST clearly indicate which fund is selected.  
  **FR-007（功能需求）**: 系统必须允许用户从可见基金列表中选择基金，并清晰标识当前选中项。
- **FR-008**: When a fund is selected, the system MUST display a detail panel that includes fund identity, key daily indicators, and a visible trend chart area.  
  **FR-008（功能需求）**: 当基金被选中时，系统必须展示详情面板，其中包含基金身份信息、关键当日指标与可见的走势图区域。
- **FR-009**: The system MUST allow the user to enter or adjust holding amount and holding shares for the selected fund.  
  **FR-009（功能需求）**: 系统必须允许用户为当前选中基金输入或调整持仓金额与持仓份额。
- **FR-010**: The system MUST calculate and display cost per share as holding amount divided by holding shares when holding shares is greater than zero.  
  **FR-010（功能需求）**: 当持仓份额大于 0 时，系统必须按“持仓金额 ÷ 持仓份额”计算并展示每份成本。
- **FR-011**: The system MUST calculate and display daily change amount for the selected fund using the holding amount and the fund’s daily change percentage.  
  **FR-011（功能需求）**: 系统必须基于持仓金额与该基金的当日涨跌幅计算并展示当日涨跌额。
- **FR-012**: The system MUST provide sorting controls for the fund list that support three fields: daily change percentage, daily change amount, and holding amount.  
  **FR-012（功能需求）**: 系统必须为基金列表提供排序控件，支持三个字段：当日涨跌幅、当日涨跌额、持仓金额。
- **FR-013**: For each supported sorting field, the system MUST support three states: descending, ascending, and no sorting.  
  **FR-013（功能需求）**: 对每个可排序字段，系统必须支持三种状态：降序、升序、不排序。
- **FR-014**: When “no sorting” is selected, the system MUST restore the default order of funds for the active list.  
  **FR-014（功能需求）**: 当选择“不排序”时，系统必须恢复当前列表的默认基金顺序。
- **FR-015**: The demo MUST preserve functional parity with the current application behavior for list selection, fund selection, add/remove, and holding calculations, while allowing visual redesign.  
  **FR-015（功能需求）**: 演示页面必须在“列表选择、基金选择、增删、持仓计算”等行为上与现有应用保持功能一致，同时允许视觉重设计。

## Assumptions / 假设

- The demo uses representative sample data embedded in the page and does not require live data connections.  
  演示页面使用内嵌的代表性示例数据，不依赖实时数据连接。
- The demo focuses on interaction clarity and calculation correctness rather than persistence across restarts.  
  演示页面更关注交互清晰与计算正确性，而非跨重启持久化。
- Daily change amount is derived from the current holding amount multiplied by the daily change percentage shown for the fund.  
  当日涨跌额由“当前持仓金额 × 基金展示的当日涨跌幅”推导得到。
- The default order for “no sorting” is the order in which funds appear in the selected list before any sort is applied.  
  “不排序”的默认顺序指该列表在未应用排序前的展示顺序。

### Key Entities *(include if feature involves data)*

- **Fund List / 基金列表**: A named collection that groups funds and can be selected as the active context. Key attributes include list name, visibility state, and an ordered set of funds.  
  **Fund List / 基金列表（实体说明）**: 用于分组基金的命名集合，可作为当前激活上下文。关键属性包括列表名称、显隐状态与有序基金集合。
- **Fund / 基金**: An individual tracked fund within a list. Key attributes include identifier, display name, daily change percentage, and calculated daily change amount.  
  **Fund / 基金（实体说明）**: 列表中的单只被跟踪基金。关键属性包括基金标识、显示名称、当日涨跌幅与计算得到的当日涨跌额。
- **Holding / 持仓**: User-provided position inputs associated with a selected fund. Key attributes include holding amount, holding shares, and derived cost per share.  
  **Holding / 持仓（实体说明）**: 与选中基金关联的用户输入持仓信息。关键属性包括持仓金额、持仓份额与推导出的每份成本。
- **Sort Preference / 排序偏好**: The current sorting configuration applied to the visible fund list. Key attributes include sort field, sort direction, and whether sorting is active.  
  **Sort Preference / 排序偏好（实体说明）**: 当前应用在可见基金列表上的排序配置。关键属性包括排序字段、排序方向与排序是否启用。

## Constitution Alignment *(mandatory)*

<!--
  ACTION REQUIRED: Explicitly state how this feature complies with each
  constitution principle, or document justified violations with migration plans.
-->

- **Tauri Desktop Architecture**: The feature is a demo-facing specification that remains within the desktop application scope and does not introduce external surfaces.  
  / **Tauri 桌面架构**: 该功能是面向演示的规范，保持在桌面应用范围内，不引入额外的外部暴露面。
- **Rust Owns Data & Network**: The demo does not redefine data or network ownership and is compatible with existing patterns where core data handling remains outside the UI.  
  / **Rust 管理数据与网络**: 演示页面不重新定义数据与网络的归属，兼容现有“核心数据处理不在 UI 内部”的模式。
- **UI-Only Frontend**: The demo emphasizes presentation and interaction design while keeping rules explicit and reviewable in the specification.  
  / **仅 UI 前端**: 演示页面强调展示与交互设计，并在规范中明确可审查的规则边界。
- **Local-First Persistence & Recovery**: The demo does not require persistence changes and can operate with local sample data without affecting recovery expectations.  
  / **本地优先持久化与恢复**: 演示页面不要求持久化变更，可使用本地示例数据运行，不影响恢复相关预期。
- **Fund List Semantics & Data Integrity**: The specification preserves existing list semantics (single active list, scoped funds) and makes calculations and sorting rules explicit and testable.  
  / **基金列表语义与数据完整性**: 该规范保持既有列表语义（单一激活列表、基金作用域清晰），并将计算与排序规则明确为可测试条目。

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: In a usability walkthrough, a user can switch lists, add a fund, select it, and view its details within 60 seconds without guidance.  
  **SC-001（可衡量结果）**: 在可用性走查中，用户无需引导即可在 60 秒内完成切换列表、添加基金、选中基金并查看详情。
- **SC-002**: For a list of at least 20 funds, applying any supported sort completes with a visibly correct order in under 2 seconds.  
  **SC-002（可衡量结果）**: 在至少 20 只基金的列表上，应用任一排序方式后，2 秒内可见顺序正确。
- **SC-003**: Holding calculations display consistent results such that cost per share and daily change amount match the documented formulas in 100% of tested cases.  
  **SC-003（可衡量结果）**: 持仓计算结果保持一致，在所有测试用例中，每份成本与当日涨跌额均 100% 符合文档公式。
- **SC-004**: In stakeholder review, the demo is judged to cover all current core behaviors (list selection, fund selection, add/remove, holding calculations, sorting) with no missing flows.  
  **SC-004（可衡量结果）**: 在干系人评审中，演示页面被确认覆盖全部当前核心行为（列表选择、基金选择、增删、持仓计算、排序），不存在缺失流程。
