# Tasks / 任务: Holding Amount in Fund Detail Panel

**Input**: Design documents from `/specs/004-holding-amount/`  
**输入**：来自 `/specs/004-holding-amount/` 的设计文档  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/  
**前置条件**：plan.md、spec.md、research.md、data-model.md、contracts/

**Tests**: Core module tests required (storage/list_manager).  
**测试**：核心模块测试必需（storage/list_manager）。

**Organization**: Tasks grouped by user story for independent delivery.  
**组织方式**：任务按用户故事分组，确保可独立交付。

## Phase 1: Setup (Shared Infrastructure) / 阶段 1：准备（共享基础设施）

**Purpose**: Align on current code paths and extension points.  
**目标**：对齐当前代码路径与扩展点。

- [x] T001 Review current storage schema in `src-tauri/src/models.rs` and note where to add positions
- [x] T002 Review storage read/write flow in `src-tauri/src/modules/storage.rs` for migration insertion points
- [x] T003 Review fund detail UI composition in `src/components/FundDetailPanel.tsx`
- [x] T004 Review Tauri command layer in `src-tauri/src/commands.rs` for new position commands
- [x] T005 Review frontend Tauri hooks in `src/hooks/useTauriCommands.ts`

---

## Phase 2: Foundational (Blocking Prerequisites) / 阶段 2：基础（阻塞性前置）

**Purpose**: Shared data model + storage + command scaffolding.  
**目标**：共享数据模型、存储与命令基础。

- [x] T006 Add `GroupFundPosition` type in `src/types/fund.ts`
- [x] T007 Add storage model fields for positions in `src-tauri/src/models.rs`
- [x] T008 Implement storage migration defaults for missing positions in `src-tauri/src/modules/storage.rs`
- [x] T009 Implement storage helpers for position CRUD in `src-tauri/src/modules/storage.rs`
- [x] T010 Implement list-scoped position lookup in `src-tauri/src/modules/list_manager.rs`
- [x] T011 Add unit tests for position migration and CRUD in `src-tauri/src/modules/storage_tests.rs`
- [x] T012 Add unit tests for list-scoped position behavior in `src-tauri/src/modules/list_manager_tests.rs`
- [x] T013 Add Tauri command handlers for `get_group_fund_position` in `src-tauri/src/commands.rs`
- [x] T014 Add Tauri command handlers for `set_group_fund_position` in `src-tauri/src/commands.rs`
- [x] T015 Add Tauri command handlers for `clear_group_fund_position` in `src-tauri/src/commands.rs`
- [x] T016 Wire command exports in `src-tauri/src/main.rs`
- [x] T017 Add command typings/clients in `src/hooks/useTauriCommands.ts`
- [x] T018 Add command wrappers in `src/hooks/useTauriApi.ts`

**Checkpoint**: Storage + commands available for UI usage.  
**检查点**：存储与命令可被 UI 使用。

---

## Phase 3: User Story 1 - Set Holding Amount Per Group (Priority: P1) 🎯 MVP / 阶段 3：用户故事 1 - 按分组设置持仓金额（优先级：P1）🎯 MVP

**Goal**: Allow entering holding info in the right detail panel, scoped by group + fund.  
**目标**：在右侧详情面板为分组+基金设置持仓信息。

**Independent Test**: Select a group and fund, set shares + cost price, save, and see holding amount updated for that group-fund pair.  
**独立测试**：选择分组与基金，设置份额与成本价并保存，持仓金额显示更新。

### Implementation for User Story 1 / 用户故事 1 的实现

- [x] T019 [US1] Add position state model in `src/components/FundDetailPanel.tsx`
- [x] T020 [US1] Add shares and unit price input UI in `src/components/FundDetailPanel.tsx`
- [x] T021 [US1] Add computed holding amount display in `src/components/FundDetailPanel.tsx`
- [x] T022 [US1] Load existing position on selection in `src/components/FundDetailPanel.tsx`
- [x] T023 [US1] Implement save handler to call `set_group_fund_position` in `src/components/FundDetailPanel.tsx`
- [x] T024 [US1] Add input validation messages (shares/unit price) in `src/components/FundDetailPanel.tsx`
- [x] T025 [US1] Add formatting helpers for holding amount in `src/components/FundDetailPanel.tsx`
- [x] T026 [US1] Add CSS styles for holding editor in `src/App.css`

**Checkpoint**: Holding info can be set and saved for the current group + fund.  
**检查点**：可为当前分组+基金设置并保存持仓信息。

---

## Phase 4: User Story 2 - Clear Holding Amount (Priority: P2) / 阶段 4：用户故事 2 - 清空持仓金额（优先级：P2）

**Goal**: Clear holding info and show placeholder state.  
**目标**：清空持仓信息并显示占位状态。

**Independent Test**: Clear holding info for a group-fund pair and see the placeholder.  
**独立测试**：清空某分组-基金持仓后显示占位。

### Implementation for User Story 2 / 用户故事 2 的实现

- [x] T027 [US2] Add clear button and confirm dialog in `src/components/FundDetailPanel.tsx`
- [x] T028 [US2] Implement clear handler to call `clear_group_fund_position` in `src/components/FundDetailPanel.tsx`
- [x] T029 [US2] Reset local UI state after clear in `src/components/FundDetailPanel.tsx`
- [x] T030 [US2] Show empty placeholder when no position exists in `src/components/FundDetailPanel.tsx`

**Checkpoint**: Clearing removes the position for the selected group + fund.  
**检查点**：清空后该分组+基金不再有持仓记录。

---

## Phase 5: User Story 3 - See Daily Change Amount (Priority: P3) / 阶段 5：用户故事 3 - 查看当日涨跌金额（优先级：P3）

**Goal**: Display daily change amount based on holding info and latest daily change data.  
**目标**：基于持仓信息与最新涨跌数据展示当日涨跌金额。

**Independent Test**: With holding info and daily change data, daily change amount is shown; otherwise show unavailable state.  
**独立测试**：有持仓与涨跌数据时显示金额，缺失时显示不可用。

### Implementation for User Story 3 / 用户故事 3 的实现

- [x] T031 [US3] Add daily change amount computation helper in `src/components/FundDetailPanel.tsx`
- [x] T032 [US3] Render daily change amount row with state handling in `src/components/FundDetailPanel.tsx`
- [x] T033 [US3] Add formatting and color styling for daily change amount in `src/App.css`
- [x] T034 [US3] Add fallback text for missing daily change data in `src/components/FundDetailPanel.tsx`

**Checkpoint**: Daily change amount appears when data is available.  
**检查点**：当日涨跌金额在数据可用时显示。

---

## Phase 6: Polish & Cross-Cutting Concerns / 阶段 6：完善与横切关注点

**Purpose**: Documentation and validation.  
**目标**：文档与验证。

- [x] T035 [P] Update feature docs if needed in `specs/004-holding-amount/quickstart.md`
- [ ] T036 Run quickstart checklist against UI in `specs/004-holding-amount/quickstart.md`
- [ ] T037 Run core tests `cargo test` with focus on storage/list_manager changes (record results)

---

## Dependencies & Execution Order / 依赖与执行顺序

### Phase Dependencies / 阶段依赖

- **Setup (Phase 1)**: No dependencies - can start immediately  
  **准备（阶段 1）**：无依赖，可立即开始
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories  
  **基础（阶段 2）**：依赖准备完成，阻塞所有用户故事
- **User Stories (Phase 3+)**: Depend on Foundational completion  
  **用户故事（阶段 3+）**：依赖基础阶段完成
- **Polish (Phase 6)**: Depends on all desired user stories being complete  
  **完善（阶段 6）**：依赖所需用户故事完成

### User Story Dependencies / 用户故事依赖

- **User Story 1 (P1)**: Starts after Foundational (Phase 2)  
- **User Story 2 (P2)**: Starts after Foundational; independent of US1 state beyond shared UI  
- **User Story 3 (P3)**: Starts after Foundational; uses US1 holding data when available

### Parallel Execution Examples / 并行执行示例

- **US1**: T019, T020, T021 can proceed in parallel; T022–T024 follow once base UI is in place.  
- **US2**: T027 and T030 can proceed in parallel; T028–T029 follow after command wiring.  
- **US3**: T031 and T033 can proceed in parallel; T032–T034 follow after computation helper.

---

## Implementation Strategy / 实施策略

- **MVP**: Deliver Phase 3 (US1) to enable setting holdings per group + fund.  
- **Incremental**: Add clearing (US2), then daily change amount display (US3).  
- **Stabilization**: Run tests and quickstart verification in Phase 6.
