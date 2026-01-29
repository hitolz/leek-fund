---

description: "Task list for Client UI Redesign From Demo"
---

# Tasks: Client UI Redesign From Demo / 参照演示的客户端界面重设计

**Input**: Design documents from `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/`  
**Prerequisites**: `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/plan.md`, `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/spec.md`, `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/research.md`, `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/data-model.md`, `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/contracts/client-ui.openapi.yaml`, `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/quickstart.md`

**Tests**: No automated tests requested. Validation uses quickstart walkthrough and existing build commands.

**Organization**: Tasks are grouped by user story so each story is independently demonstrable.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependency)
- **[Story]**: User story label ([US1], [US2], [US3])
- All tasks include exact file paths

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare UI alignment references and mapping notes.

- [X] T001 Create a demo-to-client layout mapping note in `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/research.md`
- [X] T002 [P] Capture reusable style tokens (colors, spacing, typography) in `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/research.md`
- [X] T003 [P] Add a UI alignment checklist section to `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/quickstart.md`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared UI scaffolding and styling foundations before story work.

**⚠️ CRITICAL**: No user story work should begin until this phase is complete.

- [X] T004 Identify the current client page entry point and primary layout file in `/Users/hitol/code/ai/leek-fund/src`
- [X] T005 Create or update global style tokens to match the demo palette in `/Users/hitol/code/ai/leek-fund/src`
- [X] T006 Implement a shared three-column layout scaffold matching the demo structure in `/Users/hitol/code/ai/leek-fund/src`
- [X] T007 Implement shared state indicators (selected, disabled, up/down/flat) in `/Users/hitol/code/ai/leek-fund/src`

**Checkpoint**: Foundation complete — user stories can now be implemented.

---

## Phase 3: User Story 1 - Demo-Style Core Flow / 演示风格主流程 (Priority: P1) 🎯 MVP

**Goal**: Deliver the redesigned layout with list selection, fund selection, and detail view parity.

**Independent Test**: Run the quickstart walkthrough steps 1–2 and verify list/fund selection updates the detail panel within the demo-style layout.

### Implementation for User Story 1

- [X] T008 [US1] Refactor the list panel UI to match demo layout and styling in `/Users/hitol/code/ai/leek-fund/src`
- [X] T009 [US1] Refactor the fund list UI to match demo layout and styling in `/Users/hitol/code/ai/leek-fund/src`
- [X] T010 [US1] Refactor the detail panel layout and header to match demo styling in `/Users/hitol/code/ai/leek-fund/src`
- [X] T011 [US1] Ensure list panel toggle retains current list and fund selection in `/Users/hitol/code/ai/leek-fund/src`
- [X] T012 [US1] Update selection indicators (active list, selected fund) to align with demo state styles in `/Users/hitol/code/ai/leek-fund/src`

**Checkpoint**: User Story 1 is demoable as MVP.

---

## Phase 4: User Story 2 - Manage Funds and Holdings / 管理基金与持仓 (Priority: P2)

**Goal**: Maintain fund add/remove and holding edits within the redesigned UI.

**Independent Test**: Run quickstart step 3–4 and confirm add/remove and holding edits behave correctly in the new layout.

### Implementation for User Story 2

- [X] T013 [US2] Restyle add/remove fund controls to match demo interaction patterns in `/Users/hitol/code/ai/leek-fund/src`
- [X] T014 [US2] Restyle holding amount and holding shares inputs in the detail panel in `/Users/hitol/code/ai/leek-fund/src`
- [X] T015 [US2] Align derived metric presentation (cost per share, daily change amount) with demo visual cues in `/Users/hitol/code/ai/leek-fund/src`
- [X] T016 [US2] Add empty/disabled states for no lists or no funds in the redesigned UI in `/Users/hitol/code/ai/leek-fund/src`

**Checkpoint**: User Stories 1 and 2 are both functional and visually aligned.

---

## Phase 5: User Story 3 - Sorting and Comparison / 排序与对比 (Priority: P3)

**Goal**: Present tri-state sorting controls and indicators in the demo style without altering logic.

**Independent Test**: Run quickstart step 5 and verify sorting controls, indicators, and default-order restoration.

### Implementation for User Story 3

- [X] T017 [US3] Restyle sorting controls to match demo visual hierarchy in `/Users/hitol/code/ai/leek-fund/src`
- [X] T018 [US3] Add clear sorting state indicators (descending/ascending/none) consistent with demo style in `/Users/hitol/code/ai/leek-fund/src`
- [X] T019 [US3] Ensure sorting state display remains consistent across list switches in `/Users/hitol/code/ai/leek-fund/src`

**Checkpoint**: All user stories are complete and aligned to the demo styling.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final visual refinements and validation.

- [X] T020 [P] Apply responsive tweaks to preserve usability on narrow widths in `/Users/hitol/code/ai/leek-fund/src`
- [X] T021 [P] Audit spacing, typography, and colors against demo reference in `/Users/hitol/code/ai/leek-fund/src`
- [X] T022 Run `npm run tauri:build` from `/Users/hitol/code/ai/leek-fund`
- [X] T023 Perform end-to-end walkthrough and update `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/quickstart.md` if needed

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 (Setup): No dependencies
- Phase 2 (Foundational): Depends on Phase 1 and blocks all user stories
- Phase 3 (US1): Depends on Phase 2 and serves as MVP
- Phase 4 (US2): Depends on Phase 2 and should follow US1 for layout stability
- Phase 5 (US3): Depends on Phase 2 and should follow US1/US2 for state indicators
- Phase 6 (Polish): Depends on desired stories being completed

### User Story Dependencies

- US1 (P1): No dependency on other stories after foundation
- US2 (P2): Builds on US1 layout but preserves independent core behaviors
- US3 (P3): Builds on US1/US2 for consistent state presentation

---

## Parallel Opportunities

- T001, T002, and T003 can run in parallel (docs only).
- T020 and T021 can run in parallel near the end (styling checks).

---

## Parallel Example: User Story 1 (US1)

```bash
Task: "Refactor list panel UI in /Users/hitol/code/ai/leek-fund/src"
Task: "Refactor fund list UI in /Users/hitol/code/ai/leek-fund/src"
```

---

## Parallel Example: User Story 2 (US2)

```bash
Task: "Restyle add/remove controls in /Users/hitol/code/ai/leek-fund/src"
Task: "Restyle holding inputs and metrics in /Users/hitol/code/ai/leek-fund/src"
```

---

## Parallel Example: User Story 3 (US3)

```bash
Task: "Restyle sorting controls in /Users/hitol/code/ai/leek-fund/src"
Task: "Add sorting state indicators in /Users/hitol/code/ai/leek-fund/src"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

- Complete Phase 1 and Phase 2.
- Deliver US1 and validate via quickstart steps 1–2.

### Incremental Delivery

- Add US2 for management and holdings.
- Add US3 for sorting clarity.
- Finish with polish and build validation.
