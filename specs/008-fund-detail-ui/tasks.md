---

description: "Task list for Fund Detail UI Alignment"
---

# Tasks: Fund Detail UI Alignment

**Input**: Design documents from `/Users/hitol/code/ai/leek-fund/specs/008-fund-detail-ui/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/, quickstart.md

**Tests**: Tests are optional and not requested; focus on implementation tasks.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `src-tauri/` at repository root

## Constitution-Driven Task Expectations

- Include tasks to update or add Tauri command contracts when interfaces change.
- Include tasks for SQLite migrations/backups when data shapes evolve.
- Keep UI-only tasks focused on presentation and IPC wiring; business logic stays in Rust.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Shared code surface updates needed by all stories

- [x] T001 Update fund-related TypeScript types with new summary/detail fields in `src/types/fund.ts`
- [x] T002 [P] Add new holding and refresh API invokes in `src/hooks/useTauriApi.ts`
- [x] T003 [P] Expose new holding/refresh commands in `src/hooks/useTauriCommands.ts`
- [x] T004 [P] Add formatting helpers for amounts and placeholders in `src/utils/formatters.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Backend data model and API plumbing required for all stories

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Add holding table migration SQL in `src-tauri/src/migrations/002_group_fund_positions.sql`
- [x] T006 Update migration loader to include new SQL in `src-tauri/src/migrations/mod.rs`
- [x] T007 Update migration runner to apply all migration SQL files in `src-tauri/src/modules/storage.rs`
- [x] T008 [P] Add holding/summary structs and fields in `src-tauri/src/models.rs`
- [x] T009 Update holding persistence logic to store amount+shares in `src-tauri/src/modules/position_manager.rs`
- [x] T010 Add daily change amount calculation helper in `src-tauri/src/modules/list_manager.rs`
- [x] T011 Update fund summary composition to include holding and daily change amount in `src-tauri/src/modules/fund_api.rs`
- [x] T012 Add holding CRUD commands in `src-tauri/src/commands.rs`
- [x] T013 Wire new commands into invoke handler in `src-tauri/src/main.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Three-Column Fund Browsing (Priority: P1) 🎯 MVP

**Goal**: Align the three-column layout and macOS refresh controls with the reference UI

**Independent Test**: Open the app, select a list and fund, verify layout matches the reference image and macOS menu refresh shows a checkmark on selection.

### Implementation for User Story 1

- [x] T014 [US1] Remove in-window refresh selector and prepare menu-driven refresh in `src/main.tsx`
- [x] T015 [US1] Create macOS menu items for refresh options in `src-tauri/src/main.rs`
- [x] T016 [US1] Emit refresh selection events to the UI in `src-tauri/src/main.rs`
- [x] T017 [US1] Handle refresh menu events and persist selection in `src/main.tsx`
- [x] T018 [US1] Sync initial menu check state on launch in `src/main.tsx`
- [x] T019 [US1] Align middle column header/count layout in `src/components/ListDetailView.tsx`
- [x] T020 [US1] Align list row layout to match reference spacing in `src/components/ListDetail.tsx`
- [x] T021 [US1] Align detail header/metrics layout in `src/components/FundDetailPanel.tsx`
- [x] T022 [US1] Update three-column layout and panel styles in `src/App.css`
- [x] T023 [US1] Refine shared typography and spacing styles in `src/styles.css`

**Checkpoint**: User Story 1 layout and refresh controls are fully functional

---

## Phase 4: User Story 2 - Holdings Input & Cost Calculation (Priority: P2)

**Goal**: Provide holding amount/shares inputs and auto-calculated cost price in the detail panel

**Independent Test**: Select a fund, input holding amount/shares, confirm cost price updates and shares=0 shows `--` with message, then save and clear.

### Implementation for User Story 2

- [x] T024 [US2] Add holding state and loading flow for selected list+fund in `src/App.tsx`
- [x] T025 [US2] Add holding API calls for get/save/clear in `src/hooks/useTauriApi.ts`
- [x] T026 [US2] Create holding form UI component in `src/components/HoldingForm.tsx`
- [x] T027 [US2] Render holding form and summary block in `src/components/FundDetailPanel.tsx`
- [x] T028 [US2] Add cost price calculation and zero-share placeholder logic in `src/components/HoldingForm.tsx`
- [x] T029 [US2] Wire save/clear actions to backend in `src/App.tsx`
- [x] T030 [US2] Style holding section layout, inputs, and buttons in `src/App.css`
- [x] T031 [US2] Implement holding CRUD commands in `src-tauri/src/commands.rs`
- [x] T032 [US2] Update holding validation rules for 2-decimal inputs in `src-tauri/src/modules/position_manager.rs`
- [x] T033 [US2] Include holding info and cost price in detail response in `src-tauri/src/modules/list_manager.rs`

**Checkpoint**: User Story 2 holding inputs, cost price, and persistence work independently

---

## Phase 5: User Story 3 - Sorting & Focused Add (Priority: P3)

**Goal**: Provide new sort fields/order and duplicate-add focus behavior for the middle list

**Independent Test**: Change sort field/order and confirm reorder; attempt to add a duplicate fund and verify focus behavior.

### Implementation for User Story 3

- [x] T034 [US3] Extend `FundSummary` fields for sorting in `src/types/fund.ts`
- [x] T035 [US3] Lift sort state to App level and pass to list view in `src/App.tsx`
- [x] T036 [US3] Expand sort controls to include daily change amount in `src/components/ListDetailView.tsx`
- [x] T037 [US3] Update list sorting to use backend-provided values and missing-last policy in `src/components/ListDetailView.tsx`
- [x] T038 [US3] Display daily change amount or percent consistently in `src/components/ListDetail.tsx`
- [x] T039 [US3] Implement duplicate-add focus behavior in `src/components/ListDetailView.tsx`
- [x] T040 [US3] Keep duplicate-add focus compatible with existing filter helpers in `src/utils/fundFilter.ts`

**Checkpoint**: User Story 3 sorting and focus behaviors are functional and stable

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Cleanup and verification steps across stories

- [ ] T041 [P] Update quickstart verification steps if needed in `specs/008-fund-detail-ui/quickstart.md`
- [ ] T042 Run manual verification checklist in `specs/008-fund-detail-ui/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Integrates with US1 layout but remains testable alone
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - Independent of US2 but uses shared list data

### Within Each User Story

- Models before services
- Services before UI wiring
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- T002, T003, T004 can run in parallel
- T008 can run in parallel with T005-T007 after schema approach is decided
- UI tasks within a story can run in parallel if working on different files

---

## Parallel Example: User Story 1

```bash
Task: "Align middle column header/count layout in src/components/ListDetailView.tsx"
Task: "Align detail header/metrics layout in src/components/FundDetailPanel.tsx"
Task: "Update three-column layout and panel styles in src/App.css"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Demo layout + refresh menu behavior

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Demo MVP
3. Add User Story 2 → Test independently → Demo
4. Add User Story 3 → Test independently → Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently
