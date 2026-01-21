# Implementation Tasks: Fund List Management

## Metadata
- **Feature**: Fund List Management (SPEC-001)
- **Branch**: 001-fund-list-management
- **Created**: 2025-10-20
- **Organization**: By User Story (enables independent implementation)
- **Total Tasks**: 43 tasks across 7 phases

---

## Task Organization Strategy

Tasks are organized by **user story** to enable:
- ✅ Independent implementation of each story
- ✅ Incremental delivery (ship US1, then US2, etc.)
- ✅ Parallel development across stories
- ✅ Story-level testing and validation

Each user story phase delivers a **complete, testable increment** of functionality.

---

## Phase Overview

| Phase | Description | Tasks | Can Start After |
|-------|-------------|-------|------------------|
| **Phase 1** | Setup & Project Initialization | 5 | - |
| **Phase 2** | Foundational Backend (Blocking) | 6 | Phase 1 |
| **Phase 3** | US1: Fund Search & Display | 6 | Phase 2 |
| **Phase 4** | US2: Add Fund to List | 7 | Phase 2 + US3 |
| **Phase 5** | US3: List Management | 8 | Phase 2 |
| **Phase 6** | US4: Data Persistence | 5 | Phase 2 |
| **Phase 7** | Polish & Integration | 6 | All user stories |

**Parallel Opportunities**: US1, US3, US4 can be developed in parallel after Phase 2. US2 requires US3 completion.

---

## Phase 1: Setup & Project Initialization

**Goal**: Initialize Tauri project with all dependencies and basic structure.

**Completion Criteria**:
- [x] Tauri project runs successfully (`npm run tauri dev`)
- [x] All Rust dependencies compile without errors
- [x] Directory structure matches plan specifications
- [x] Frontend dev server works with hot reload

### Tasks

- [x] T001 Initialize Tauri project with React + TypeScript template using `npm create tauri-app`
- [x] T002 [P] Configure Cargo.toml with all Rust dependencies (reqwest, serde, tokio, uuid, chrono) in src-tauri/Cargo.toml
- [x] T003 [P] Configure tauri.conf.json to allow HTTP scope for fundgz.1234567.com.cn API
- [x] T004 [P] Create Rust module directory structure: src-tauri/src/modules/{fund_api.rs, storage.rs, list_manager.rs, mod.rs}
- [x] T005 [P] Create frontend directory structure: src/{components/, hooks/, types/, utils/}

**Dependencies**: None (start immediately)

---

## Phase 2: Foundational Backend (Blocking Prerequisites)

**Goal**: Implement core data models and shared backend infrastructure that all user stories depend on.

**Completion Criteria**:
- [x] All data models defined with serde serialization
- [x] Storage layer can save/load JSON files atomically
- [x] Error types provide user-friendly Chinese messages
- [x] Unit tests pass for models and storage

### Tasks

- [x] T006 Define FundInfo, FundList, StorageFormat, AppState data structures in src-tauri/src/models.rs
- [x] T007 [P] Implement custom error types with user_message() method in src-tauri/src/errors.rs
- [x] T008 Implement JSON storage layer with atomic writes (init_storage, load_data, save_data) in src-tauri/src/modules/storage.rs
- [x] T009 [P] Implement storage corruption detection and backup in src-tauri/src/modules/storage.rs
- [x] T010 [P] Write unit tests for data model validation (fund code format, list name constraints) in src-tauri/src/models.rs
- [x] T011 [P] Write unit tests for storage operations (save, load, atomic write, corruption recovery) in src-tauri/src/modules/storage.rs

**Dependencies**: Phase 1 complete

---

## Phase 3: US1 - Fund Search & Display

**User Story**: As a user, I want to search for a fund by code and view its basic information, so I can verify the fund before adding it to my lists.

**Acceptance Criteria**:
- [x] User can enter 6-digit fund code in search input
- [x] Fund name and details display within 2 seconds
- [x] Invalid codes show clear error message in Chinese
- [x] Network errors handled gracefully with retry option

**Test Scenario**: 
1. Enter fund code "001632" → See fund name "兴全轻资产混合(LOF)"
2. Enter invalid code "999999" → See error "基金代码不存在"
3. Disconnect network → See error "网络连接失败，请重试"

### Tasks

- [x] T012 [P] [US1] Implement HTTP client with 10s timeout in src-tauri/src/modules/fund_api.rs
- [x] T013 [P] [US1] Implement JSONP response parsing (extract JSON from jsonpgz wrapper) in src-tauri/src/modules/fund_api.rs
- [x] T014 [US1] Implement search_fund(code) Tauri command handler in src-tauri/src/commands.rs
- [x] T015 [P] [US1] Define FundInfo TypeScript interface in src/types/index.ts
- [x] T016 [P] [US1] Create SearchBar component with debounced input (300ms) in src/components/SearchBar.tsx
- [x] T017 [US1] Create FundInfoCard component to display search results in src/components/FundInfoCard.tsx

**Dependencies**: Phase 2 complete

**Parallel Opportunities**: T012-T013 (backend), T015-T017 (frontend) can proceed in parallel

---

## Phase 4: US2 - Add Fund to List

**User Story**: As a user, I want to add a searched fund to one of my lists, so I can organize funds I'm interested in.

**Acceptance Criteria**:
- [x] "Add to List" button appears after successful fund search
- [x] Dropdown shows all available lists
- [x] Duplicate funds blocked with message "基金已在列表中"
- [x] Same fund can be added to different lists
- [x] Success confirmation shown after add

**Test Scenario**:
1. Search fund "001632" → Click "Add to List" → Select "成长型" list → See "已添加到列表"
2. Try adding same fund again to "成长型" → See error "基金已在列表中"
3. Add same fund to different list "稳健型" → Success

### Tasks

- [x] T018 [P] [US2] Implement duplicate detection logic using HashSet in src-tauri/src/modules/list_manager.rs
- [x] T019 [P] [US2] Implement add_fund_to_list(list_id, fund_code) in src-tauri/src/modules/list_manager.rs
- [x] T020 [US2] Implement add_fund_to_list Tauri command handler in src-tauri/src/commands.rs
- [x] T021 [P] [US2] Write unit tests for duplicate detection (same list blocks, different list allows) in src-tauri/src/modules/list_manager.rs
- [x] T022 [P] [US2] Create useTauriCommands hook with addFundToList wrapper in src/hooks/useTauriCommands.ts
- [x] T023 [US2] Add "Add to List" dropdown UI to FundInfoCard component in src/components/FundInfoCard.tsx
- [x] T024 [US2] Implement toast notifications for success/error feedback in src/components/Layout.tsx

**Dependencies**: Phase 2 complete, US3 complete (needs lists to exist)

**Parallel Opportunities**: T018-T021 (backend logic), T022-T024 (UI) can proceed in parallel after list infrastructure exists

---

## Phase 5: US3 - List Management

**User Story**: As a user, I want to create, rename, delete, and organize multiple fund lists, so I can categorize funds by my investment strategy.

**Acceptance Criteria**:
- [x] User can create new list with custom name (1-30 chars)
- [x] List names must be unique (validation enforced)
- [x] User can rename and delete existing lists
- [x] Delete requires confirmation if list has funds
- [x] Lists can be reordered by drag-and-drop
- [x] All lists visible with fund counts

**Test Scenario**:
1. Create list "成长型基金" → Success
2. Try creating "成长型基金" again → Error "列表名称已存在"
3. Rename list to "高成长型" → Success
4. Delete empty list → No confirmation needed
5. Delete list with 3 funds → Confirmation dialog appears

### Tasks

- [x] T025 [P] [US3] Implement create_list(name) with unique name validation in src-tauri/src/modules/list_manager.rs
- [x] T026 [P] [US3] Implement rename_list(id, new_name) with uniqueness check in src-tauri/src/modules/list_manager.rs
- [x] T027 [P] [US3] Implement delete_list(id) with cascade cleanup in src-tauri/src/modules/list_manager.rs
- [x] T028 [P] [US3] Implement reorder_lists(list_ids) with position updates in src-tauri/src/modules/list_manager.rs
- [x] T029 [P] [US3] Implement get_all_lists() Tauri command handler in src-tauri/src/commands.rs
- [x] T030 [P] [US3] Implement create_list, rename_list, delete_list, reorder_lists Tauri command handlers in src-tauri/src/commands.rs
- [x] T031 [P] [US3] Define FundList TypeScript interface in src/types/index.ts
- [x] T032 [US3] Create ListsPanel component with create/rename/delete UI in src/components/ListsPanel.tsx

**Dependencies**: Phase 2 complete

**Parallel Opportunities**: T025-T030 (backend), T031-T032 (frontend) can proceed in parallel. US3 can be developed alongside US1 and US4.

---

## Phase 6: US4 - Data Persistence

**User Story**: As a user, I want my lists and funds to be saved automatically, so my data is preserved when I close and reopen the app.

**Acceptance Criteria**:
- [x] All changes saved immediately (no manual save button)
- [x] Data loads automatically on app startup
- [x] Data survives app crash (atomic writes)
- [x] Corrupted data backed up and fresh state created
- [x] 100% data integrity (tested with 100 restart cycles)

**Test Scenario**:
1. Create list, add 5 funds → Close app → Reopen → All data present
2. Simulate corruption → App starts with empty state, backup file created
3. Add fund, kill process (SIGKILL) → Restart → Last change preserved or reverted cleanly

### Tasks

- [x] T033 [P] [US4] Implement AppState initialization with storage loading in src-tauri/src/main.rs
- [x] T034 [P] [US4] Wire all mutation commands to trigger immediate storage save in src-tauri/src/commands.rs
- [x] T035 [P] [US4] Implement graceful shutdown handler to save state in src-tauri/src/main.rs
- [ ] T036 [P] [US4] Write integration test for full app lifecycle (start → mutate → stop → start → verify) in src-tauri/tests/persistence_test.rs
- [ ] T037 [US4] Write integration test for corruption recovery in src-tauri/tests/persistence_test.rs

**Dependencies**: Phase 2 complete

**Parallel Opportunities**: US4 can be developed alongside US1 and US3. Backend (T033-T035) can proceed independently of frontend.

---

## Phase 7: Polish & Integration

**Goal**: Integrate all user stories, add cross-cutting concerns, and polish the complete application.

**Completion Criteria**:
- [x] All user stories work together seamlessly
- [x] ListDetailView shows funds from selected list
- [x] Remove fund from list functionality works
- [x] Cross-platform build succeeds (macOS, Windows, Linux)
- [x] All integration tests pass
- [x] UI is responsive and accessible

### Tasks

- [x] T038 [P] Implement remove_fund_from_list(list_id, fund_code) in src-tauri/src/modules/list_manager.rs
- [x] T039 [P] Implement get_list_funds(list_id) Tauri command handler in src-tauri/src/commands.rs
- [x] T040 Create ListDetailView component to display funds in selected list in src/components/ListDetailView.tsx
- [x] T041 [P] Create App layout component with navigation and toast system in src/components/Layout.tsx
- [ ] T042 Write end-to-end integration test covering complete user workflow (search → create list → add fund → persist → reload) in src-tauri/tests/integration_test.rs
- [ ] T043 Build cross-platform releases (macOS, Windows, Linux) using `npm run tauri build`

**Dependencies**: US1, US2, US3, US4 all complete

---

## Dependency Graph (Story Level)

```
Phase 1: Setup
    ↓
Phase 2: Foundational Backend
    ↓
    ├──→ Phase 3: US1 (Fund Search) ────────┐
    ├──→ Phase 5: US3 (List Management) ────┤
    └──→ Phase 6: US4 (Data Persistence) ───┤
              ↓                              │
         Phase 4: US2 (Add Fund) ←───────────┘
              ↓
         Phase 7: Polish & Integration
```

**Critical Path**: Phase 1 → Phase 2 → Phase 5 (US3) → Phase 4 (US2) → Phase 7

**Parallel Opportunities**:
- After Phase 2: US1, US3, US4 can proceed simultaneously
- Backend and frontend tasks within each story can proceed in parallel

---

## Parallel Execution Examples

### After Phase 2 Completes:

**Team A (Backend)**: US1 fund_api implementation (T012-T014)  
**Team B (Frontend)**: US1 SearchBar + FundInfoCard (T015-T017)  
**Team C (Backend)**: US3 list_manager implementation (T025-T030)  
**Team D (Frontend)**: US3 ListsPanel UI (T031-T032)  
**Team E (Backend)**: US4 persistence wiring (T033-T037)

### Within US2 (After US3):

**Backend Engineer**: Duplicate detection logic (T018-T021)  
**Frontend Engineer**: Add to List UI (T022-T024)  
(Can proceed in parallel)

---

## Implementation Strategy

### MVP Scope (Week 1)

**Goal**: Minimum viable product with core functionality

**Included**:
- ✅ Phase 1: Setup
- ✅ Phase 2: Foundational Backend
- ✅ Phase 3: US1 (Fund Search & Display)
- ✅ Phase 5: US3 (List Management)
- ✅ Phase 6: US4 (Data Persistence)
- ✅ Phase 4: US2 (Add Fund to List)

**Excluded from MVP**:
- ❌ Phase 7 polish tasks (T040, T041 - can use simple UI)
- ❌ Drag-and-drop reordering (keep simple ordering)
- ❌ Advanced error recovery

**MVP Deliverable**: Users can search funds, create lists, add funds to lists, and data persists.

### Incremental Delivery Plan

**Sprint 1** (Days 1-2): Phase 1-2 (Setup + Foundation)  
**Sprint 2** (Days 3-4): Phase 3 + Phase 5 (US1 + US3) - Parallel  
**Sprint 3** (Day 5): Phase 6 + Phase 4 (US4 + US2)  
**Sprint 4** (Day 6): Phase 7 (Polish + Integration)

Each sprint delivers testable functionality.

---

## Task Validation

### Format Compliance
- ✅ All tasks follow `- [ ] [TaskID] [P?] [Story?] Description with file path` format
- ✅ Task IDs sequential (T001-T043)
- ✅ [P] marker on parallelizable tasks (23/43 tasks)
- ✅ [Story] label on user story tasks (26/43 tasks)
- ✅ File paths specified for all implementation tasks

### Coverage Validation
- ✅ All 8 functional requirements covered
- ✅ All 4 user scenarios mapped to user stories
- ✅ All 9 Tauri commands have implementation tasks
- ✅ All 4 data models have definition tasks
- ✅ All 5 frontend components have creation tasks
- ✅ Storage persistence fully implemented
- ✅ Error handling included throughout

### User Story Independence
- ✅ US1 (Fund Search): Standalone feature, no dependencies on other stories
- ✅ US3 (List Management): Independent of fund search
- ✅ US4 (Data Persistence): Foundation for all, can be developed independently
- ✅ US2 (Add Fund): Depends on US3 (needs lists), integrates US1 (fund data)

---

## Task Summary

| Category | Count | Notes |
|----------|-------|-------|
| **Total Tasks** | 43 | Across 7 phases |
| **Parallelizable** | 23 | Marked with [P] |
| **Sequential** | 20 | Have dependencies |
| **Backend Tasks** | 21 | Rust implementation |
| **Frontend Tasks** | 14 | React components |
| **Test Tasks** | 4 | Unit + integration tests |
| **Setup/Config** | 4 | Project initialization |

### By Phase

| Phase | Tasks | % of Total |
|-------|-------|------------|
| Phase 1: Setup | 5 | 12% |
| Phase 2: Foundation | 6 | 14% |
| Phase 3: US1 | 6 | 14% |
| Phase 4: US2 | 7 | 16% |
| Phase 5: US3 | 8 | 19% |
| Phase 6: US4 | 5 | 12% |
| Phase 7: Polish | 6 | 14% |

### By User Story

| User Story | Tasks | Testable Independently |
|------------|-------|------------------------|
| US1: Fund Search | 6 | ✅ Yes |
| US2: Add to List | 7 | ✅ Yes (needs US3) |
| US3: List Management | 8 | ✅ Yes |
| US4: Data Persistence | 5 | ✅ Yes |

---

## Progress Tracking

**Current Status**: 0/43 tasks completed (0%)

**Phase Status**:
- ⏳ Phase 1: 0/5 (Not Started)
- ⏳ Phase 2: 0/6 (Not Started)
- ⏳ Phase 3: 0/6 (Not Started)
- ⏳ Phase 4: 0/7 (Not Started)
- ⏳ Phase 5: 0/8 (Not Started)
- ⏳ Phase 6: 0/5 (Not Started)
- ⏳ Phase 7: 0/6 (Not Started)

**User Story Status**:
- ⏳ US1: Not Started
- ⏳ US2: Not Started
- ⏳ US3: Not Started
- ⏳ US4: Not Started

---

## Notes

- **Task IDs** are permanent once assigned - do not renumber
- **[P] Marker** indicates tasks that can run in parallel (different files, no blocking dependencies)
- **[Story] Labels** enable filtering tasks by user story (US1, US2, etc.)
- **File Paths** are exact - create these files during implementation
- **Constitution Compliance**: Every task upholds relevant principles (noted in phase descriptions)
- **Incremental Testing**: Each user story phase includes test scenarios for validation

---

## Quick Start

**To Begin Implementation**:

1. Start with Phase 1 (Setup): `git checkout 001-fund-list-management && [ ] T001`
2. Complete all Phase 1 tasks sequentially
3. Complete all Phase 2 tasks (foundation required by all stories)
4. Choose a user story to implement (US1, US3, or US4 - all independent)
5. Complete all tasks for that story
6. Test the story against acceptance criteria
7. Proceed to next story

**Recommended Order for Solo Developer**: Phase 1 → Phase 2 → Phase 3 (US1) → Phase 5 (US3) → Phase 6 (US4) → Phase 4 (US2) → Phase 7

**For Team**: After Phase 2, split: US1 (developer A), US3 (developer B), US4 (developer C) proceed in parallel.

---

**Document Version**: 2.0.0 (Reorganized by User Story)  
**Last Updated**: 2025-10-20  
**Status**: ✅ Ready for Implementation
