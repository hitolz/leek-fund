---

description: "Task list for feature implementation"

description_zh: "功能实现的任务清单"

---

# Tasks / 任务: SQLite Local Persistence

**Input**: Design documents from `/specs/003-sqlite-storage/`  
**输入**：来自 `/specs/003-sqlite-storage/` 的设计文档  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/  
**前置条件**：plan.md（必需）、spec.md（用户故事必需）、research.md、data-model.md、contracts/

**Tests**: Tests are implied for storage/migration modules; add minimal unit tests for critical paths.  
**测试**：存储/迁移模块需补充最小单元测试覆盖关键路径。

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.  
**组织方式**：任务按用户故事分组，确保每个故事可独立实现与测试。

## Format / 格式: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies) / 可并行执行（不同文件、无依赖）
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3) / 该任务所属用户故事（如 US1、US2、US3）
- Include exact file paths in descriptions / 描述中需包含准确的文件路径

## Phase 1: Setup (Shared Infrastructure) / 阶段 1：准备（共享基础设施）

- [X] T001 Add sqlx SQLite dependency and features in `src-tauri/Cargo.toml`
- [X] T002 [P] Add SQLite connection configuration constants in `src-tauri/src/modules/storage.rs`
- [X] T003 [P] Create migrations folder and base schema in `src-tauri/src/migrations/001_init.sql`
- [X] T004 [P] Add migration runner helper in `src-tauri/src/migrations/mod.rs`
- [X] T005 Update storage module to initialize SQLite file path in `src-tauri/src/modules/storage.rs`

---

## Phase 2: Foundational (Blocking Prerequisites) / 阶段 2：基础（阻塞性前置）

- [X] T006 Create SQLite connection pool in `src-tauri/src/modules/storage.rs`
- [X] T007 Implement database schema creation using migrations in `src-tauri/src/modules/storage.rs`
- [ ] T008 Create repository functions for lists in `src-tauri/src/modules/storage.rs`
- [ ] T009 Create repository functions for list funds in `src-tauri/src/modules/storage.rs`
- [ ] T010 Create migration status table helpers in `src-tauri/src/modules/storage.rs`
- [ ] T011 Add new storage models for DB rows in `src-tauri/src/models.rs`
- [X] T012 Update AppState to hold SQLite pool and metadata in `src-tauri/src/models.rs`
- [X] T013 Wire storage initialization to use SQLite in `src-tauri/src/main.rs`
- [ ] T014 Update error types for database errors in `src-tauri/src/errors.rs`
- [ ] T015 Add unit tests for storage CRUD in `src-tauri/src/modules/storage_tests.rs`

**Checkpoint**: SQLite storage layer ready for list CRUD operations.  
**检查点**：SQLite 存储层完成并可执行列表 CRUD。

---

## Phase 3: User Story 1 - Persist Lists in SQLite (Priority: P1) / 阶段 3：用户故事 1 - 列表持久化到 SQLite（优先级：P1）

**Goal**: All list operations read/write to SQLite.  
**目标**：所有列表操作读写 SQLite。

**Independent Test**: Create list, add fund, restart app, verify data persists.  
**独立测试**：创建列表、添加基金、重启应用后数据仍存在。

- [X] T016 [US1] Update list_manager to use SQLite storage for list CRUD in `src-tauri/src/modules/list_manager.rs`
- [X] T017 [US1] Update list_manager to use SQLite storage for fund membership in `src-tauri/src/modules/list_manager.rs`
- [X] T018 [US1] Update command handlers to use new storage paths in `src-tauri/src/commands.rs`
- [X] T019 [US1] Ensure fund list ordering persisted in SQLite in `src-tauri/src/modules/list_manager.rs`
- [X] T020 [US1] Update storage save/load logic to no longer write JSON for lists in `src-tauri/src/modules/storage.rs`
- [X] T021 [US1] Add unit tests for list_manager SQLite paths in `src-tauri/src/modules/list_manager_tests.rs`

**Checkpoint**: Lists are persisted to SQLite and survive restart.  
**检查点**：列表数据已持久化 SQLite 且重启后保留。

---

## Phase 4: User Story 2 - Data Migration from JSON (Priority: P2) / 阶段 4：用户故事 2 - 从 JSON 迁移数据（优先级：P2）

**Goal**: Existing JSON data migrates into SQLite on first run.  
**目标**：首次运行时完成 JSON -> SQLite 迁移。

**Independent Test**: Start with JSON data, verify SQLite contains same lists/funds.  
**独立测试**：带 JSON 数据启动，SQLite 内数据一致。

- [X] T022 [US2] Read legacy JSON file and parse user data in `src-tauri/src/modules/storage.rs`
- [X] T023 [US2] Implement JSON-to-SQLite migration logic in `src-tauri/src/modules/storage.rs`
- [ ] T024 [US2] Add migration completion flag in SQLite in `src-tauri/src/modules/storage.rs`
- [X] T025 [US2] Ensure migration is idempotent and safe to re-run in `src-tauri/src/modules/storage.rs`
- [ ] T026 [US2] Add migration unit tests using sample JSON in `src-tauri/src/modules/storage_tests.rs`

**Checkpoint**: Migration runs once and is safe.  
**检查点**：迁移仅执行一次且可重复安全运行。

---

## Phase 5: User Story 3 - Database Recovery Handling (Priority: P3) / 阶段 5：用户故事 3 - 数据库恢复处理（优先级：P3）

**Goal**: Provide recovery guidance when database is missing or corrupted.  
**目标**：数据库缺失/损坏时提供恢复指引。

**Independent Test**: Corrupt/remove DB file, restart app, see recovery guidance.  
**独立测试**：损坏/删除数据库文件，重启后看到恢复提示。

- [X] T027 [US3] Detect DB corruption and rename old file in `src-tauri/src/modules/storage.rs`
- [X] T028 [US3] Create fresh DB after corruption in `src-tauri/src/modules/storage.rs`
- [X] T029 [US3] Expose recovery warning message from backend in `src-tauri/src/commands.rs`
- [X] T030 [US3] Display recovery guidance in UI in `src/App.tsx`
- [ ] T031 [US3] Add recovery test cases in `src-tauri/src/modules/storage_tests.rs`

**Checkpoint**: Recovery flow works and message is shown.  
**检查点**：恢复流程可用且提示可见。

---

## Phase 6: Polish & Cross-Cutting Concerns / 阶段 6：完善与横切关注点

- [X] T032 Add migration notes to quickstart in `specs/003-sqlite-storage/quickstart.md`
- [X] T033 Update contracts if command signatures change in `specs/003-sqlite-storage/contracts/openapi.yaml`
- [ ] T034 [P] Update docs or README if storage path changes in `README.md`

---

## Dependencies & Execution Order / 依赖与执行顺序

### Phase Dependencies / 阶段依赖

- **Setup (Phase 1)**: No dependencies - can start immediately / **准备（阶段 1）**：无依赖，可立即开始
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories / **基础（阶段 2）**：依赖准备完成，阻塞所有用户故事
- **User Stories (Phase 3+)**: Depend on Foundational phase completion / **用户故事（阶段 3+）**：依赖基础阶段完成
- **Polish (Phase 6)**: Depends on all user stories / **完善（阶段 6）**：依赖所有用户故事完成

### User Story Dependencies / 用户故事依赖

- **US1** is required before US2 and US3 (SQLite storage base).  
  **US1** 是 US2、US3 的前置（SQLite 存储基础）。
- **US2** depends on US1 storage layer to write migrated data.  
  **US2** 依赖 US1 存储层写入迁移数据。
- **US3** depends on storage initialization and error handling.  
  **US3** 依赖存储初始化与错误处理。

---

## Parallel Opportunities / 并行机会

- T002, T003, T004 can run in parallel.  
  T002、T003、T004 可并行。
- T008 and T009 can run in parallel after T006.  
  T008 与 T009 在 T006 后可并行。
- T027 and T028 can run in parallel once detection strategy is defined.  
  T027 与 T028 在确定检测策略后可并行。

---

## Implementation Strategy / 实施策略

- **MVP**: Complete Phase 1–3 to deliver SQLite persistence for lists.  
  **MVP**：完成阶段 1–3，交付列表 SQLite 持久化。
- **Increment 1**: Add JSON migration (Phase 4).  
  **增量 1**：增加 JSON 迁移（阶段 4）。
- **Increment 2**: Add recovery guidance (Phase 5).  
  **增量 2**：增加恢复指引（阶段 5）。
- **Polish**: Documentation and contract updates (Phase 6).  
  **完善**：文档与契约更新（阶段 6）。

---

## Parallel Example: User Story 1 / 并行示例：用户故事 1

```bash
T016, T017 can run in parallel for US1 after T006
US1 中 T016、T017 可在 T006 之后并行
```
