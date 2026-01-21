---

description: "Task list for fund tracker client implementation"
---

# Tasks: Fund List Management
# 任务：基金列表管理

**Input**: Design documents from `/specs/001-fund-tracker-client/`
**输入**：来自 `/specs/001-fund-tracker-client/` 的设计文档

**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/
**前置条件**：plan.md（必需）、spec.md（用户故事必需）、research.md、data-model.md、contracts/

**Tests**: 核心模块测试为必需；如需省略，必须在 spec.md 中说明理由。
**测试**：核心模块测试为必需；如需省略，必须在 spec.md 中说明理由。

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.
**组织方式**：任务按用户故事分组，确保每个故事可独立实现与测试。

## Format: `[ID] [P?] [Story] Description`
## 格式：`[ID] [P?] [Story] 描述`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[P]**：可并行执行（不同文件、无依赖）
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- **[Story]**：任务所属用户故事（如 US1、US2、US3）
- Include exact file paths in descriptions
- 描述中必须包含准确文件路径

## Path Conventions
## 路径约定

- **Single project**: `src/`, `tests/` at repository root
- **单体项目**：仓库根目录的 `src/`、`tests/`
- **Web app**: `backend/src/`, `frontend/src/`
- **Web 应用**：`backend/src/`、`frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` or `android/src/`
- **移动端**：`api/src/`、`ios/src/` 或 `android/src/`
- Paths shown below assume single project - adjust based on plan.md structure
- 以下路径默认单体结构，按 plan.md 调整

## Phase 1: Setup (Shared Infrastructure)
## 阶段 1：搭建（共享基础设施）

**Purpose**: Project initialization and basic structure
**目的**：项目初始化与基础结构准备

- [X] T001 Confirm Tauri + React dev workflow runs in this repo (`package.json`, `src-tauri/Cargo.toml`)
中文：确认本仓库可运行 Tauri + React 开发流程（`package.json`、`src-tauri/Cargo.toml`）。
- [X] T002 [P] Define TypeScript types for fund/list data in `src/types/fund.ts`
中文：在 `src/types/fund.ts` 中定义基金/列表相关 TypeScript 类型。
- [X] T003 [P] Create frontend API wrapper for Tauri invokes in `src/hooks/useTauriApi.ts`
中文：在 `src/hooks/useTauriApi.ts` 中封装 Tauri 调用 API。

---

## Phase 2: Foundational (Blocking Prerequisites)
## 阶段 2：基础设施（阻塞性前置）

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented
**目的**：所有用户故事实施前必须完成的核心基础设施

**⚠️ CRITICAL**: No user story work can begin until this phase is complete
**⚠️ 关键**：此阶段完成前不得开始任何用户故事

- [X] T004 Define Rust data models for Fund/FundList/UserData in `src-tauri/src/models.rs`
中文：在 `src-tauri/src/models.rs` 中定义 Fund/FundList/UserData 的 Rust 模型。
- [X] T005 Implement storage load/save + migration guard in `src-tauri/src/modules/storage.rs`
中文：在 `src-tauri/src/modules/storage.rs` 中实现存储读写与迁移保护。
- [X] T006 Implement list management operations in `src-tauri/src/modules/list_manager.rs`
中文：在 `src-tauri/src/modules/list_manager.rs` 中实现列表管理操作。
- [X] T007 Implement fund data fetch/parsing in `src-tauri/src/modules/fund_api.rs`
中文：在 `src-tauri/src/modules/fund_api.rs` 中实现基金数据获取与解析。
- [X] T008 Define error types and user-facing messages in `src-tauri/src/errors.rs`
中文：在 `src-tauri/src/errors.rs` 中定义错误类型与用户提示。
- [X] T009 Wire Tauri commands to modules in `src-tauri/src/commands.rs`
中文：在 `src-tauri/src/commands.rs` 中绑定 Tauri 命令与模块。
- [X] T010 Add core module unit tests for fund API parsing in `src-tauri/src/modules/fund_api_tests.rs`
中文：在 `src-tauri/src/modules/fund_api_tests.rs` 添加基金 API 解析单元测试。
- [X] T011 Add core module unit tests for storage load/save in `src-tauri/src/modules/storage_tests.rs`
中文：在 `src-tauri/src/modules/storage_tests.rs` 添加存储读写单元测试。
- [X] T012 Add core module unit tests for list manager rules in `src-tauri/src/modules/list_manager_tests.rs`
中文：在 `src-tauri/src/modules/list_manager_tests.rs` 添加列表规则单元测试。

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel
**检查点**：基础设施完成后，可并行推进用户故事

---

## Phase 3: User Story 1 - Search and View Fund Info (Priority: P1) 🎯 MVP
## 阶段 3：用户故事 1 - 搜索并查看基金信息（优先级 P1）🎯 MVP

**Goal**: Users can search by fund code and see current fund information.
**目标**：用户可通过基金代码搜索并查看当前基金信息。

**Independent Test**: Enter valid/invalid fund codes and verify results + errors.
**独立测试**：输入有效/无效基金代码，验证结果与错误提示。

### Implementation for User Story 1
### 用户故事 1 的实现

- [X] T013 [P] [US1] Build search input + result card UI in `src/components/FundSearch.tsx`
中文：在 `src/components/FundSearch.tsx` 中实现搜索输入与结果卡片 UI。
- [X] T014 [US1] Wire search action to `search_fund` command in `src/components/FundSearch.tsx`
中文：在 `src/components/FundSearch.tsx` 中将搜索操作接入 `search_fund` 命令。
- [X] T015 [US1] Add error/empty/loading states for search in `src/components/FundSearch.tsx`
中文：在 `src/components/FundSearch.tsx` 中补充错误/空态/加载状态。
- [X] T016 [US1] Render update time and change percent in `src/components/FundSearch.tsx`
中文：在 `src/components/FundSearch.tsx` 中展示更新时间与涨跌幅。

**Checkpoint**: User Story 1 is fully functional and testable independently
**检查点**：用户故事 1 可独立运行与测试

---

## Phase 4: User Story 2 - Organize Funds into Lists (Priority: P2)
## 阶段 4：用户故事 2 - 基金列表管理（优先级 P2）

**Goal**: Users can create lists and add/remove funds without duplicates.
**目标**：用户可创建列表并添加/移除基金，且无重复。

**Independent Test**: Create list, add fund, block duplicate, remove fund.
**独立测试**：创建列表、添加基金、阻止重复、移除基金。

### Implementation for User Story 2
### 用户故事 2 的实现

- [X] T017 [P] [US2] Build list sidebar UI (create/rename/delete) in `src/components/FundLists.tsx`
中文：在 `src/components/FundLists.tsx` 中实现列表侧栏 UI（新增/改名/删除）。
- [X] T018 [P] [US2] Build list detail view with fund members in `src/components/ListDetail.tsx`
中文：在 `src/components/ListDetail.tsx` 中实现列表详情与成员展示。
- [X] T019 [US2] Wire list CRUD commands in `src/components/FundLists.tsx`
中文：在 `src/components/FundLists.tsx` 中接入列表增删改命令。
- [X] T020 [US2] Wire add/remove fund actions in `src/components/ListDetail.tsx`
中文：在 `src/components/ListDetail.tsx` 中接入基金添加/移除操作。
- [X] T021 [US2] Enforce duplicate handling and user messages in `src/components/ListDetail.tsx`
中文：在 `src/components/ListDetail.tsx` 中处理重复添加与提示信息。

**Checkpoint**: User Story 2 works independently (list operations + membership)
**检查点**：用户故事 2 可独立运行（列表与成员操作）

---

## Phase 5: User Story 3 - Persist User Data (Priority: P3)
## 阶段 5：用户故事 3 - 数据持久化（优先级 P3）

**Goal**: Lists and memberships persist across restarts.
**目标**：列表与成员在重启后仍保留。

**Independent Test**: Create lists, close app, reopen, verify data restored.
**独立测试**：创建列表、关闭应用、重新打开并验证数据恢复。

### Implementation for User Story 3
### 用户故事 3 的实现

- [X] T022 [US3] Load initial lists on app startup in `src/App.tsx`
中文：在 `src/App.tsx` 中实现启动时加载列表。
- [X] T023 [US3] Refresh UI state after list mutations in `src/App.tsx`
中文：在 `src/App.tsx` 中实现列表变更后的 UI 刷新。
- [X] T024 [US3] Add data corruption recovery messaging in `src/App.tsx`
中文：在 `src/App.tsx` 中加入数据损坏的恢复提示。

**Checkpoint**: User Story 3 is independently testable (data persists across restarts)
**检查点**：用户故事 3 可独立测试（重启后数据持久化）

---

## Phase 6: Polish & Cross-Cutting Concerns
## 阶段 6：打磨与横切关注点

**Purpose**: Improvements that affect multiple user stories
**目的**：影响多个用户故事的收尾与改进

- [X] T025 [P] Update quickstart verification steps in `specs/001-fund-tracker-client/quickstart.md`
中文：更新 `specs/001-fund-tracker-client/quickstart.md` 的验证步骤。
- [X] T026 Add UI copy polish and empty states in `src/components/FundSearch.tsx`
中文：在 `src/components/FundSearch.tsx` 中优化文案与空态。
- [ ] T027 Run quickstart flow and confirm behavior matches `specs/001-fund-tracker-client/quickstart.md`
中文：执行 quickstart 流程并确认与 `specs/001-fund-tracker-client/quickstart.md` 一致。

---

## Dependencies & Execution Order
## 依赖与执行顺序

### Phase Dependencies
### 阶段依赖

- **Setup (Phase 1)**: No dependencies - can start immediately
- **搭建（阶段 1）**：无依赖，可立即开始
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **基础设施（阶段 2）**：依赖阶段 1，阻塞所有用户故事
- **User Stories (Phase 3+)**: Depend on Foundational completion
- **用户故事（阶段 3+）**：依赖阶段 2 完成
- **Polish (Final Phase)**: Depends on all user stories being complete
- **打磨（最终阶段）**：依赖所有用户故事完成

### User Story Dependencies
### 用户故事依赖

- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories
- **用户故事 1（P1）**：基础设施完成后可开始，无其他故事依赖
- **User Story 2 (P2)**: Can start after Foundational - Uses list/fund modules
- **用户故事 2（P2）**：基础设施完成后可开始，依赖列表/基金模块
- **User Story 3 (P3)**: Can start after Foundational - Depends on storage loading
- **用户故事 3（P3）**：基础设施完成后可开始，依赖存储加载

### Within Each User Story
### 每个用户故事内的顺序

- Models before services/commands
- 模型先于服务/命令
- Commands before frontend wiring
- 命令先于前端接线
- Core implementation before UX polish
- 核心实现先于体验打磨

### Parallel Opportunities
### 可并行项

- T002 and T003 can run in parallel
- T002 与 T003 可并行
- T010, T011, T012 can run in parallel
- T010、T011、T012 可并行
- T017 and T018 can run in parallel
- T017 与 T018 可并行

---

## Parallel Example: User Story 2
## 并行示例：用户故事 2

```bash
Task: "Build list sidebar UI in src/components/FundLists.tsx"
Task: "Build list detail view with fund members in src/components/ListDetail.tsx"
```

```bash
任务："在 src/components/FundLists.tsx 中实现列表侧栏 UI"
任务："在 src/components/ListDetail.tsx 中实现列表详情视图"
```

---

## Implementation Strategy
## 实施策略

### MVP First (User Story 1 Only)
### MVP 优先（仅用户故事 1）

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. STOP and validate User Story 1 independently

1. 完成阶段 1：搭建
2. 完成阶段 2：基础设施
3. 完成阶段 3：用户故事 1
4. 停止并独立验证用户故事 1

### Incremental Delivery
### 逐步交付

1. Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Demo MVP
3. Add User Story 2 → Test independently → Demo
4. Add User Story 3 → Test independently → Demo
5. Polish + quickstart validation

1. 搭建 + 基础设施 → 准备完成
2. 加入用户故事 1 → 独立测试 → MVP 演示
3. 加入用户故事 2 → 独立测试 → 演示
4. 加入用户故事 3 → 独立测试 → 演示
5. 打磨 + quickstart 验证

---

## Notes
## 备注

- [P] tasks = different files, no dependencies
- [P] 任务表示不同文件、无依赖
- Each user story is independently completable and testable
- 每个用户故事可独立完成与测试
- Core module tests are required per constitution
- 宪法要求核心模块必须有测试
