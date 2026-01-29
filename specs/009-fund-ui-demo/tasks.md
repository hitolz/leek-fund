---

description: "Task list for Fund Demo UI Redesign"
---

# Tasks: Fund Demo UI Redesign / 基金演示页面重设计

**Input**: Design documents from `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/`  
**Prerequisites**: `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/plan.md`, `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/spec.md`, `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/research.md`, `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/data-model.md`, `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/contracts/fund-demo.openapi.yaml`, `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/quickstart.md`

**Tests**: No automated tests are required by the spec. Validation is performed via quickstart walkthroughs and repository compile checks.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently where feasible.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependency on incomplete tasks)
- **[Story]**: User story label ([US1], [US2], [US3])
- All file paths are absolute and point to concrete edit targets

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the demo entry point and align reviewer guidance artifacts.

- [X] T001 Create the demo folder and initial HTML scaffold in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T002 [P] Add a short “Demo Scope” note and open instructions in `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/quickstart.md`
- [X] T003 [P] Add a demo-specific scope disclaimer in `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/contracts/fund-demo.openapi.yaml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Establish shared data, state, and rendering utilities that all stories rely on.

**⚠️ CRITICAL**: No user story work should begin until this phase is complete.

- [X] T004 Define embedded sample data for lists, funds, holdings, and default order positions in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T005 Implement the base three-region layout shell (list panel, fund list, detail panel) in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T006 Implement shared state containers for active list, selected fund, holdings, and sorting preference in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T007 Implement shared helpers for stable default ordering, tri-state sorting, and numeric formatting in `/Users/hitol/code/ai/leek-fund/demo/index.html`

**Checkpoint**: Foundation complete — user stories can now be implemented.

---

## Phase 3: User Story 1 - Manage Lists and Funds / 管理列表与基金 (Priority: P1) 🎯 MVP

**Goal**: Users can toggle the list panel, select a list, and add or remove funds scoped to that list.

**Independent Test**: Execute quickstart steps 1–4 and confirm: selection is preserved across panel toggles, list switching scopes funds correctly, and add/remove changes the visible set immediately.

### Implementation for User Story 1

- [X] T008 [P] [US1] Align list and fund interaction notes with the demo scope in `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/contracts/fund-demo.openapi.yaml`
- [X] T009 [US1] Implement list panel rendering and single-active-list selection behavior in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T010 [US1] Implement list panel hide/show behavior without losing the current selection in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T011 [US1] Render the fund list strictly scoped to the active list’s default order in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T012 [US1] Implement the add-fund flow (code + name) with in-list uniqueness and append-to-default-order semantics in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T013 [US1] Implement the remove-fund flow with selection fallback to the next available fund or clear state in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T014 [US1] Implement empty states for “no lists” and “no funds in list,” including disabled actions where appropriate, in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T015 [US1] Record US1 validation notes aligned to quickstart steps 1–4 in `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/quickstart.md`

**Checkpoint**: User Story 1 is fully demoable and serves as the MVP.

---

## Phase 4: User Story 2 - View Details and Holdings / 查看详情与持仓 (Priority: P2)

**Goal**: Selecting a fund reveals details and a trend chart, and users can enter holdings to see cost per share and daily change amount.

**Independent Test**: Execute quickstart steps 5–6 and confirm: selecting a fund updates the detail panel, the trend chart renders, and derived metrics follow the documented formulas.

### Implementation for User Story 2

- [X] T016 [P] [US2] Reconcile holding derivation notes with the implemented demo formulas in `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/data-model.md`
- [X] T017 [US2] Implement the fund detail panel header and daily indicator display bound to the selected fund in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T018 [US2] Render the trend chart as inline SVG from the selected fund’s trend points in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T019 [US2] Implement holding inputs (holding amount, holding shares) with non-negative validation messaging in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T020 [US2] Implement derived metric displays for cost per share and daily change amount using the documented formulas in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T021 [US2] Wire fund selection changes to hydrate detail content, chart data, and holding state consistently in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T022 [US2] Record US2 validation notes aligned to quickstart steps 5–6 in `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/quickstart.md`

**Checkpoint**: User Stories 1 and 2 are both functional and demoable.

---

## Phase 5: User Story 3 - Sort Funds / 基金排序 (Priority: P3)

**Goal**: Users can sort by daily change percent, daily change amount, or holding amount with descending, ascending, and none states.

**Independent Test**: Execute quickstart step 7 and confirm: each field supports tri-state sorting, ties are stable, and selecting “none” restores the default order.

### Implementation for User Story 3

- [X] T023 [P] [US3] Document sorting decisions and default-order restoration rules for reviewers in `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/research.md`
- [X] T024 [US3] Implement sorting control UI for field selection and tri-state direction toggles in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T025 [US3] Implement per-fund derived sort values for daily change amount, holding amount, and daily change percent in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T026 [US3] Implement stable sorting with tie-breaker by default index and reset-to-default behavior when direction is none in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T027 [US3] Ensure sorting operates on a derived view without mutating stored default order across list switches and panel toggles in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T028 [US3] Record US3 validation notes aligned to quickstart step 7 in `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/quickstart.md`

**Checkpoint**: All user stories are functional and independently verifiable via the quickstart flows.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improve clarity, reviewer trust, and readiness for implementation tasks.

- [X] T029 [P] Refine visual states (selected, empty, gain/loss, disabled) and spacing for readability in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T030 [P] Add a demo-only disclaimer, formula legend, and sample-data note to the page footer in `/Users/hitol/code/ai/leek-fund/demo/index.html`
- [X] T031 Run a repository compile check via `cargo check` from `/Users/hitol/code/ai/leek-fund/src-tauri`
- [X] T032 Perform an end-to-end walkthrough and update any mismatched steps in `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 (Setup): No dependencies — can start immediately.
- Phase 2 (Foundational): Depends on Phase 1 — blocks all user stories.
- Phase 3 (US1): Depends on Phase 2 — MVP.
- Phase 4 (US2): Depends on Phase 2; in practice, it is best executed after US1 to ensure fund selection exists.
- Phase 5 (US3): Depends on Phase 2; in practice, it is best executed after US1 and US2 because it uses holdings and derived values.
- Phase 6 (Polish): Depends on the desired user stories being complete.

### User Story Dependencies

- US1 (P1): No dependency on other stories once foundation is complete.
- US2 (P2): Functionally independent after foundation, but relies on fund selection surfaces typically delivered in US1.
- US3 (P3): Relies on list scoping from US1 and derived holding values defined in US2 for full fidelity.

### Suggested Completion Order

- MVP: US1 → validate → demo.
- Incremental: US1 → US2 → US3 → Polish.

---

## Parallel Opportunities

- T002 and T003 can run in parallel (different documents).
- T008, T016, and T023 can run in parallel with implementation work (documentation in separate files).
- T029 and T032 can run in parallel near the end (demo page vs. quickstart doc).

---

## Parallel Example: User Story 1 (US1)

```bash
Task: "Align list and fund interaction notes in /Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/contracts/fund-demo.openapi.yaml"
Task: "Implement list panel and selection behavior in /Users/hitol/code/ai/leek-fund/demo/index.html"
```

---

## Parallel Example: User Story 2 (US2)

```bash
Task: "Reconcile holding derivation notes in /Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/data-model.md"
Task: "Implement detail panel, holdings inputs, and chart in /Users/hitol/code/ai/leek-fund/demo/index.html"
```

---

## Parallel Example: User Story 3 (US3)

```bash
Task: "Document sorting rules in /Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/research.md"
Task: "Implement tri-state sorting controls and stable ordering in /Users/hitol/code/ai/leek-fund/demo/index.html"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

- Complete Phase 1 and Phase 2.
- Deliver US1 (Phase 3) and validate using quickstart steps 1–4.
- Demo value: reviewers can manage lists and funds immediately.

### Incremental Delivery

- Add US2 for details, charts, and holdings calculations.
- Add US3 for sorting across derived and base values.
- Finish with Polish tasks to improve clarity and reviewer confidence.
