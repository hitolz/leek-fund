# Tasks: Fund Add Search Filter

**Input**: Design documents from `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not requested in spec; no test tasks included.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Define shared types used across stories

- [X] T001 Add FundFilterState type and export in `src/types/fund.ts` and `src/types/index.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared helpers required by multiple stories

- [X] T002 Add fund filter helpers (exists check, filtered list) in `src/utils/fundFilter.ts`

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Filter existing fund on add (Priority: P1) 🎯 MVP

**Goal**: When adding a fund that already exists in the middle list, filter the lower list to that fund and show a non-blocking hint.

**Independent Test**: Add an existing fund and confirm the lower list only shows that fund with a hint, without creating a duplicate.

### Implementation for User Story 1

- [X] T003 [US1] Add filter state to the list view and detect existing fund on add in `src/components/ListDetailView.tsx`
- [X] T004 [US1] Apply filter to the rendered fund list and display “已存在” hint in `src/components/ListDetailView.tsx`
- [X] T005 [US1] Show empty-state message when filter active but fund not present in list in `src/components/ListDetailView.tsx`
- [X] T006 [P] [US1] Add styles for filter hint and empty-state in `src/App.css`

**Checkpoint**: User Story 1 is independently functional and testable

---

## Phase 4: User Story 2 - Add a new fund normally (Priority: P2)

**Goal**: Adding a new fund behaves exactly as before and does not trigger filtering.

**Independent Test**: Add a fund not in the list and confirm the lower list remains unfiltered and the fund is added normally.

### Implementation for User Story 2

- [X] T007 [US2] Ensure add flow bypasses filter when fund is not in list in `src/components/ListDetailView.tsx`
- [X] T008 [US2] Clear any prior filter state on successful new add in `src/components/ListDetailView.tsx`

**Checkpoint**: User Story 2 is independently functional and testable

---

## Phase 5: User Story 3 - Return to full list after filtering (Priority: P3)

**Goal**: Provide a one-step clear action to return to the full lower list view after filtering.

**Independent Test**: Trigger the filter and use the clear action to restore the full list.

### Implementation for User Story 3

- [X] T009 [US3] Add clear-filter control and reset filter state in `src/components/ListDetailView.tsx`
- [X] T010 [P] [US3] Add styles for the clear-filter control in `src/App.css`

**Checkpoint**: User Story 3 is independently functional and testable

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Ensure documentation reflects UI changes

- [X] T011 Update usage notes for the clear-filter control in `specs/007-fund-add-search/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - blocks user stories
- **User Stories (Phase 3+)**: Depend on Foundational completion
- **Polish (Phase 6)**: Depends on user stories completion

### User Story Dependencies

- **User Story 1 (P1)**: No dependencies after Foundational
- **User Story 2 (P2)**: No dependencies after Foundational
- **User Story 3 (P3)**: No dependencies after Foundational

### Parallel Opportunities

- T006 and T010 can run in parallel with other story tasks (CSS-only changes)
- Story phases can proceed in parallel after Phase 2 if staffed

---

## Parallel Example: User Story 1

```bash
Task: "Add filter state to the list view and detect existing fund on add in src/components/ListDetailView.tsx"
Task: "Add styles for filter hint and empty-state in src/App.css"
```

---

## Parallel Example: User Story 3

```bash
Task: "Add clear-filter control and reset filter state in src/components/ListDetailView.tsx"
Task: "Add styles for the clear-filter control in src/App.css"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate User Story 1 independently

### Incremental Delivery

1. Setup + Foundational
2. User Story 1 → Validate
3. User Story 2 → Validate
4. User Story 3 → Validate
5. Polish documentation
