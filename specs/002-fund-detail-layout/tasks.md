---

description: "Task list for feature implementation"

description_zh: "功能实现的任务清单"

---

# Tasks / 任务: Fund Detail Three-Column Layout

**Input**: Design documents from `/specs/002-fund-detail-layout/`  
**输入**：来自 `/specs/002-fund-detail-layout/` 的设计文档  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/  
**前置条件**：plan.md（必需）、spec.md（用户故事必需）、research.md、data-model.md、contracts/

**Tests**: Tests are not explicitly requested; add only if later required.  
**测试**：未明确要求测试；如后续要求再补充。

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.  
**组织方式**：任务按用户故事分组，确保每个故事可独立实现与测试。

## Format / 格式: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies) / 可并行执行（不同文件、无依赖）
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3) / 该任务所属用户故事（如 US1、US2、US3）
- Include exact file paths in descriptions / 描述中需包含准确的文件路径

## Phase 1: Setup (Shared Infrastructure) / 阶段 1：准备（共享基础设施）

- [x] T001 Create UI component stubs for detail panel and chart in `src/components/FundDetailPanel.tsx` and `src/components/FundTrendChart.tsx`
- [x] T002 [P] Add fund data types for summaries/details/trend in `src/types/fund.ts`
- [x] T003 [P] Add formatting helpers for daily change and timestamps in `src/utils/formatters.ts`

---

## Phase 2: Foundational (Blocking Prerequisites) / 阶段 2：基础（阻塞性前置）

- [x] T004 Define backend data models for fund summary/detail/trend in `src-tauri/src/models.rs`
- [x] T005 Add fund detail + trend fetch logic in `src-tauri/src/modules/fund_api.rs`
- [x] T006 Update list manager to return fund summaries with daily change in `src-tauri/src/modules/list_manager.rs`
- [x] T007 Expose new Tauri commands for list funds/detail/trend in `src-tauri/src/commands.rs`
- [x] T008 Wire module exports for new commands in `src-tauri/src/modules/mod.rs`
- [x] T009 Add error variants for missing list/fund/trend data in `src-tauri/src/errors.rs`
- [x] T010 Add frontend command wrappers for list funds/detail/trend in `src/hooks/useTauriCommands.ts`

**Checkpoint**: Core data flow available for UI consumption.  
**检查点**：核心数据流可供前端使用。

---

## Phase 3: User Story 1 - Browse Lists and Funds (Priority: P1) / 阶段 3：用户故事 1 - 浏览列表与基金（优先级：P1）

**Goal**: Select a list and see only its funds in the middle column.  
**目标**：选择列表后中间列仅显示该列表基金。

**Independent Test**: Click any list and verify the middle column updates to show that list’s funds or an empty-state message.  
**独立测试**：点击任意列表，验证中间列更新为对应基金或空态提示。

- [x] T011 [US1] Add selected list state and handler wiring in `src/App.tsx`
- [x] T012 [P] [US1] Pass selected list into `src/components/ListsPanel.tsx` and emit selection changes
- [x] T013 [US1] Fetch list fund summaries on selection in `src/hooks/useTauriApi.ts`
- [x] T014 [US1] Render fund summaries in middle column in `src/components/ListDetailView.tsx`
- [x] T015 [US1] Add empty-state messaging for no lists / no funds in `src/components/ListDetailView.tsx`

**Checkpoint**: User Story 1 works independently.  
**检查点**：用户故事 1 可独立运行。

---

## Phase 4: User Story 2 - See Daily Change at a Glance (Priority: P2) / 阶段 4：用户故事 2 - 一眼看到当日涨跌（优先级：P2）

**Goal**: Show code, name, and daily change for each fund row.  
**目标**：每行基金展示代码、名称与当日涨跌。

**Independent Test**: Open a list and verify each fund row shows code, name, change value, and timestamp.  
**独立测试**：打开列表，验证每行显示代码、名称、涨跌数值与时间戳。

- [x] T016 [US2] Map fund summary fields into row props in `src/components/ListDetail.tsx`
- [x] T017 [US2] Render daily change + timestamp using formatters in `src/components/ListDetail.tsx`
- [x] T018 [US2] Add positive/negative/zero styling in `src/styles.css`

**Checkpoint**: User Story 2 works independently (with US1 list selection).  
**检查点**：用户故事 2 在 US1 基础上可独立验证。

---

## Phase 5: User Story 3 - View Fund Details and Trend (Priority: P3) / 阶段 5：用户故事 3 - 查看基金详情与走势（优先级：P3）

**Goal**: Selecting a fund shows details and a trend chart in the right column.  
**目标**：选择基金后右侧显示详情与走势图。

**Independent Test**: Click a fund and verify detail panel renders data or a no-data message for trend.  
**独立测试**：点击基金，验证右侧渲染详情与走势或无数据提示。

- [x] T019 [US3] Add selected fund state and handler wiring in `src/App.tsx`
- [x] T020 [P] [US3] Fetch fund detail on selection in `src/hooks/useTauriApi.ts`
- [x] T021 [P] [US3] Fetch fund trend on selection in `src/hooks/useTauriApi.ts`
- [x] T022 [US3] Implement detail panel layout in `src/components/FundDetailPanel.tsx`
- [x] T023 [US3] Implement trend chart rendering in `src/components/FundTrendChart.tsx`
- [x] T024 [US3] Add empty/loading/error states in `src/components/FundDetailPanel.tsx`

**Checkpoint**: User Story 3 works independently (with US1/US2 in place).  
**检查点**：用户故事 3 在 US1/US2 基础上可独立验证。

---

## Phase 6: Polish & Cross-Cutting Concerns / 阶段 6：完善与横切关注点

- [x] T025 Update three-column layout styling and responsive behavior in `src/components/Layout.tsx`
- [x] T026 [P] Refine column spacing, scroll behavior, and empty-state visuals in `src/styles.css`
- [x] T027 Update quickstart verification steps if UI behavior changes in `specs/002-fund-detail-layout/quickstart.md`

---

## Dependencies & Execution Order / 依赖与执行顺序

### Phase Dependencies / 阶段依赖

- **Setup (Phase 1)**: No dependencies - can start immediately / **准备（阶段 1）**：无依赖，可立即开始
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories / **基础（阶段 2）**：依赖准备完成，阻塞所有用户故事
- **User Stories (Phase 3+)**: Depend on Foundational phase completion / **用户故事（阶段 3+）**：依赖基础阶段完成
- **Polish (Phase 6)**: Depends on all user stories / **完善（阶段 6）**：依赖所有用户故事完成

### User Story Dependencies / 用户故事依赖

- **US1** is required before US2 and US3 (selection context and list data).  
  **US1** 是 US2、US3 的前置（选择上下文与列表数据）。
- **US2** can proceed after US1 and shares the same middle-column list.  
  **US2** 在 US1 之后进行，复用中间列列表。
- **US3** depends on list + fund selection from US1/US2.  
  **US3** 依赖 US1/US2 的列表与基金选择。

---

## Parallel Opportunities / 并行机会

- T002, T003, T004 can run in parallel.  
  T002、T003、T004 可并行。
- T020 and T021 can run in parallel once selection state exists.  
  T020 与 T021 在选择状态完成后可并行。
- UI layout tasks in T025 and styling in T026 can run in parallel.  
  T025 与 T026 可并行。

---

## Implementation Strategy / 实施策略

- **MVP**: Complete Phase 1–3 to deliver list selection + fund list view.  
  **MVP**：完成阶段 1–3，交付列表选择与基金列表展示。
- **Increment 1**: Add daily change rendering (Phase 4).  
  **增量 1**：增加当日涨跌展示（阶段 4）。
- **Increment 2**: Add detail panel and trend chart (Phase 5).  
  **增量 2**：增加详情面板与走势图（阶段 5）。
- **Polish**: Final layout and UX refinements (Phase 6).  
  **完善**：最终布局与体验优化（阶段 6）。

---

## Parallel Example: User Story 1 / 并行示例：用户故事 1

```bash
T012, T013, T014 can run in parallel for US1 after T011
US1 中 T012、T013、T014 可在 T011 之后并行
```
