# Feature Specification: Fund Detail UI Alignment

**Feature Branch**: `008-fund-detail-ui`  
**Created**: 2026-01-26  
**Status**: Draft  
**Input**: User description: "按照图片 [Image #1] 修改现有页面，补充功能"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Three-Column Fund Browsing (Priority: P1)

As a user, I can browse lists, funds, and fund details in a three-column layout
that matches the provided reference image so I can understand my portfolio at a
glance.
作为用户，我可以在与参考图一致的三列布局中浏览列表、基金与详情，以便快速
理解持仓状态。

**Why this priority**: It is the primary navigation and viewing flow; without it,
the product does not meet the expected UX baseline.
**Why this priority**: 这是核心浏览流程；缺少该流程会导致体验与目标不符。

**Independent Test**: A tester can open the app, select a list, select a fund, and
confirm the layout, selected states, and detail panel match the reference image.
**Independent Test**: 测试人员可打开应用、选择列表与基金，并验证布局、选中
态与详情面板与参考图一致。

**Acceptance Scenarios**:

1. **Given** at least one list with funds, **When** the user selects a list,
   **Then** the middle column shows that list's funds with code/name and daily
   change, and the right panel shows details for the selected fund.
2. **Given** no fund is selected, **When** the list loads, **Then** an empty or
   placeholder detail state appears instead of stale data.
3. **Given** the macOS menu bar is available, **When** the user selects a refresh
   option, **Then** the selected option shows a checkmark in the menu.

---

### User Story 2 - Holdings Input & Cost Calculation (Priority: P2)

As a user, I can input holding amount and holding shares, and see the calculated
cost price so I understand my position cost basis.
作为用户，我可以输入持仓金额与持仓份额，并查看自动计算的成本价，以理解持仓成本。

**Why this priority**: Holding visibility and daily impact are core decision aids
for users managing positions.
**Why this priority**: 持仓与当日影响是用户管理仓位的关键决策信息。

**Independent Test**: A tester can input amount and shares, and verify the cost
price equals amount divided by shares while daily impact remains visible.
**Independent Test**: 测试人员可输入金额与份额，验证成本价等于金额除以份额，
同时当日涨跌影响仍可见。

**Acceptance Scenarios**:

1. **Given** a fund detail view, **When** the user enters holding amount and
   holding shares, **Then** the cost price updates as amount divided by shares.
2. **Given** existing holding values, **When** the user clears the holding,
   **Then** the holding summary resets to zero/empty and the daily impact shows
   the cleared state.

---

### User Story 3 - Sorting & Focused Add (Priority: P3)

As a user, I can sort the middle fund list by daily change amount, daily change
percentage, or holding amount, and choose descending, ascending, or no sort,
while duplicate adds focus the existing fund.
作为用户，我可以按当日涨跌额、当日涨跌幅或持仓金额对中间列表排序，并选择
降序、升序或不排序；重复添加时会聚焦已有基金。

**Why this priority**: These controls improve efficiency and prevent errors when
managing many funds.
**Why this priority**: 这些控制提升效率并避免管理多基金时出错。

**Independent Test**: A tester can choose each sort field and order, observe list
reordering, then attempt a duplicate add and see focus behavior.
**Independent Test**: 测试人员可选择每个排序字段与顺序，观察列表重排，再尝试
重复添加并确认聚焦行为。

**Acceptance Scenarios**:

1. **Given** a list with multiple funds, **When** the user selects a sort field
   and order, **Then** the list reorders accordingly and reflects the chosen
   sort controls, or returns to original order when set to no sort.
2. **Given** a fund already exists in the list, **When** the user attempts to add
   it again, **Then** the UI focuses or filters to the existing fund and prevents
   duplication.

---

### Edge Cases

- What happens when the selected list is empty or has been deleted?
  当选中的列表为空或已被删除时会发生什么？
- How does the system handle missing or delayed fund detail data?
  当基金详情数据缺失或延迟时系统如何处理？
- What happens when a holding value is invalid or out of range?
  当持仓值无效或超出范围时会发生什么？
- What happens when holding shares are zero during cost calculation?
  当持仓份额为零导致成本计算时会发生什么？

### Assumptions & Dependencies

- The feature reuses existing fund data, lists, and holding data already stored
  by the app and does not introduce new data sources.
  本功能复用应用现有的基金、列表与持仓数据，不引入新的数据来源。
- The reference layout is authoritative for spacing, density, and panel
  placement, and minor visual tweaks are acceptable if needed for readability.
  参考图在布局结构、密度与面板位置上具有权威性，必要时允许为可读性做轻微调整。
- Sorting defaults to "no sort" until the user selects a field and order.
  排序默认不排序，直到用户选择字段与顺序。
- Sort state is global within a session and remains unchanged when switching
  between lists.
  排序状态在会话内为全局设置，切换列表时保持不变。

## Clarifications

### Session 2026-01-26

- Q: What should cost price show when holding shares are zero? → A: Show `--` with
  a clear message indicating shares are zero.
- Q: How should sorting treat missing values? → A: Always place missing values
  at the end of the list.
- Q: What input precision is required for holding amount and shares? → A: Amount
  supports 2 decimal places; shares support 2 decimal places.
- Q: How should sorting state behave across list switches? → A: Global sort state
  persists within the session and does not reset when switching lists.
- Q: Where should refresh controls live on macOS? → A: Place refresh options in
  the macOS menu bar and show a checkmark for the selected option.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST present a three-column layout with list, fund list,
  and fund detail panels matching the reference layout structure.
  系统必须提供三列布局，包含列表、基金列表与基金详情面板，并与参考布局结构一致。
- **FR-002**: The system MUST display list counts, fund codes, fund names, and
  daily change in the middle column list. Holding amount and daily change amount
  (when available) MUST be provided by the backend with the fund list payload.
  系统必须在中间列表显示列表数量、基金代码、基金名称与当日涨跌。持仓金额与当日
  涨跌额（可用时）必须由后端在基金列表数据中返回。
- **FR-003**: Users MUST be able to select a list and a fund, and the detail panel
  MUST update to the selected fund.
  用户必须能够选择列表与基金，详情面板必须随选中基金更新。
- **FR-004**: The system MUST provide holding inputs for holding amount and
  holding shares, and show cost price as an auto-calculated value. Holding
  amount MUST support 2 decimal places, and holding shares MUST support 2
  decimal places.
  系统必须提供持仓金额与持仓份额输入，并显示自动计算的成本价。持仓金额必须支持
  2 位小数，持仓份额必须支持 2 位小数。
- **FR-005**: The system MUST calculate cost price as holding amount divided by
  holding shares; when shares are zero, the cost price MUST display `--` with a
  clear message indicating zero shares.
  系统必须按“成本价 = 持仓金额 / 持仓份额”计算成本价；当持仓份额为零时，
  成本价必须显示 `--`，并明确提示“份额为零”。
- **FR-006**: The system MUST allow users to save or clear holding inputs and
  reflect the saved values in the detail summary.
  系统必须允许保存或清空持仓输入，并在详情汇总中反映已保存的值。
- **FR-007**: The system MUST show computed holding summary and daily change
  impact in the detail panel when holding data is present.
  当持仓数据存在时，系统必须在详情面板显示持仓汇总与当日涨跌影响。
- **FR-008**: The system MUST show a fund trend chart area in the detail panel and
  display a placeholder when data is unavailable.
  系统必须在详情面板显示基金趋势图区域，数据不可用时显示占位状态。
- **FR-009**: Users MUST be able to sort the fund list by daily change amount,
  daily change percentage, or holding amount, with descending, ascending, and
  no-sort modes, and see the applied sort state. Sorting fields MUST be sourced
  from backend-provided values (not computed on the frontend). Missing values
  MUST be sorted to the end of the list regardless of sort order.
  用户必须能按当日涨跌额、当日涨跌幅或持仓金额排序，支持降序、升序与不排序，
  并看到当前排序状态。排序字段必须来自后端返回值（非前端计算），缺失值必须
  在任意排序方式下排在列表末尾。
- **FR-010**: The system MUST prevent duplicate fund additions within a list and
  provide a focused view of the existing fund when a duplicate add is attempted.
  系统必须阻止单列表内重复添加基金，并在尝试重复添加时聚焦展示该基金。
- **FR-011**: The system MUST provide clear empty states for lists, fund lists,
  and detail panels when data is absent.
  当数据为空时，系统必须为列表、基金列表与详情面板提供清晰空态。
- **FR-012**: On macOS, refresh controls MUST be available in the menu bar, and
  the selected refresh option MUST show a checkmark.
  在 macOS 上，刷新控制必须放在菜单栏中，且被选中的刷新选项必须显示打勾标记。

### Key Entities *(include if feature involves data)*

- **Fund List**: A user-defined collection of funds with a display name and order.
  **基金列表**：用户定义的基金集合，包含名称与排序。
- **Fund Item**: A fund entry with code, name, daily change metrics, holding
  amount (if set), and daily change amount (if available).
  **基金条目**：包含基金代码、名称、当日涨跌指标，以及持仓金额（如已设置）与
  当日涨跌额（如可用）。
- **Fund Detail**: The detail view data for a selected fund (latest value, daily
  change, trend series, and metadata).
  **基金详情**：选中基金的详情数据（最新净值、当日涨跌、走势序列与元数据）。
- **Holding**: User-input holding amount and shares, plus computed cost price and
  summaries for a fund.
  **持仓**：用户输入的持仓金额与份额，以及计算出的成本价与汇总。

## Constitution Alignment *(mandatory)*

- **Tauri Desktop Architecture**: The feature remains within the desktop app
  experience and does not introduce web-only flows.
  / **Tauri 桌面架构**：功能仍保持在桌面应用体验内，不引入纯 Web 流程。
- **Rust Owns Data & Network**: UI updates rely on existing data commands; no
  frontend-only data ownership is introduced.
  / **Rust 管理数据与网络**：UI 更新依赖已有数据接口，不新增前端独占数据逻辑。
- **UI-Only Frontend**: UI work is limited to layout and interaction wiring.
  / **仅 UI 前端**：UI 仅负责布局与交互接线。
- **Local-First Persistence & Recovery**: Holding edits and list changes continue
  to be stored locally with existing recovery behavior.
  / **本地优先持久化与恢复**：持仓与列表变更继续本地保存并保持原有恢复机制。
- **Fund List Semantics & Data Integrity**: List uniqueness and validation rules
  remain unchanged while UI surfaces them consistently.
  / **基金列表语义与数据完整性**：列表去重与校验规则保持不变，UI 一致呈现。

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 90% of test users can locate a fund's daily change and detail values
  within 20 seconds of opening the app.
  **SC-001**：90% 的测试用户在打开应用 20 秒内找到基金的当日涨跌与详情数值。
- **SC-002**: Users can input holding amount and shares and see the computed cost
  price within 5 seconds after input.
  **SC-002**：用户在输入持仓金额与份额后 5 秒内看到计算出的成本价。
- **SC-003**: Sort controls successfully reorder the list for all three fields
  and three order states in 100% of scripted QA checks.
  **SC-003**：在脚本化 QA 中，三类排序字段与三种排序方式均能正确生效。
- **SC-004**: Duplicate add attempts result in zero duplicate entries and provide
  a visible focus on the existing fund in 100% of tests.
  **SC-004**：重复添加尝试在 100% 测试中不产生重复条目，并能聚焦展示已有基金。
- **SC-005**: Empty-state messaging appears in all empty data conditions during
  scripted QA runs.
  **SC-005**：在脚本化 QA 测试中，所有空数据情况均展示空态提示。
