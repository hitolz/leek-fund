---

description: "Task list template for feature implementation"

description_zh: "功能实现的任务清单模板"

---

# Tasks / 任务: [FEATURE NAME]

**Input**: Design documents from `/specs/[###-feature-name]/`  
**输入**：来自 `/specs/[###-feature-name]/` 的设计文档  
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/  
**前置条件**：plan.md（必需）、spec.md（用户故事必需）、research.md、data-model.md、contracts/

**Tests**: 核心模块测试为必需；如需省略，必须在 spec.md 中说明理由。  
**测试**：核心模块测试为必需；如需省略，必须在 spec.md 中说明理由。

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.  
**组织方式**：任务按用户故事分组，确保每个故事可独立实现与测试。

## Format / 格式: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies) / 可并行执行（不同文件、无依赖）
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3) / 该任务所属用户故事（如 US1、US2、US3）
- Include exact file paths in descriptions / 描述中需包含准确的文件路径

## Path Conventions / 路径规范

- **Single project**: `src/`, `tests/` at repository root / **单体项目**：仓库根目录下 `src/`、`tests/`
- **Web app**: `backend/src/`, `frontend/src/` / **Web 应用**：`backend/src/`、`frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` or `android/src/` / **移动端**：`api/src/`、`ios/src/` 或 `android/src/`
- Paths shown below assume single project - adjust based on plan.md structure / 下方路径默认单体项目，按 plan.md 实际结构调整

<!-- 
  ============================================================================
  IMPORTANT: The tasks below are SAMPLE TASKS for illustration purposes only.
  重要：以下任务仅为示例。

  The /speckit.tasks command MUST replace these with actual tasks based on:
  /speckit.tasks 命令必须基于以下内容替换为真实任务：
  - User stories from spec.md (with their priorities P1, P2, P3...)
  - spec.md 中的用户故事（含优先级 P1、P2、P3...）
  - Feature requirements from plan.md
  - plan.md 中的功能需求
  - Entities from data-model.md
  - data-model.md 中的实体
  - Endpoints from contracts/
  - contracts/ 中的接口

  Tasks MUST be organized by user story so each story can be:
  任务必须按用户故事组织，以便每个故事可以：
  - Implemented independently
  - 独立实现
  - Tested independently
  - 独立测试
  - Delivered as an MVP increment
  - 作为 MVP 增量交付

  DO NOT keep these sample tasks in the generated tasks.md file.
  生成的 tasks.md 中不得保留这些示例任务。
  ============================================================================
-->

## Phase 1: Setup (Shared Infrastructure) / 阶段 1：准备（共享基础设施）

**Purpose**: Project initialization and basic structure  
**目标**：项目初始化与基础结构

- [ ] T001 Create project structure per implementation plan / 按实施计划创建项目结构
- [ ] T002 Initialize [language] project with [framework] dependencies / 初始化 [语言] 项目并加入 [框架] 依赖
- [ ] T003 [P] Configure linting and formatting tools / 配置代码检查与格式化工具

---

## Phase 2: Foundational (Blocking Prerequisites) / 阶段 2：基础（阻塞性前置）

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented  
**目标**：在任何用户故事实现前必须完成的核心基础设施

**⚠️ CRITICAL**: No user story work can begin until this phase is complete  
**⚠️ 关键**：此阶段完成前不得开始用户故事实现

Examples of foundational tasks (adjust based on your project):  
基础任务示例（按项目实际情况调整）：

- [ ] T004 Setup database schema and migrations framework / 建立数据库结构与迁移框架
- [ ] T005 [P] Implement authentication/authorization framework / 实现认证/授权框架
- [ ] T006 [P] Setup API routing and middleware structure / 搭建 API 路由与中间件结构
- [ ] T007 Create base models/entities that all stories depend on / 创建所有故事依赖的基础模型/实体
- [ ] T008 Configure error handling and logging infrastructure / 配置错误处理与日志基础设施
- [ ] T009 Setup environment configuration management / 配置环境与配置管理

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel  
**检查点**：基础设施完成，可并行开展用户故事实现

---

## Phase 3: User Story 1 - [Title] (Priority: P1) 🎯 MVP / 阶段 3：用户故事 1 - [标题]（优先级：P1）🎯 MVP

**Goal**: [Brief description of what this story delivers]  
**目标**：[简要说明该故事交付的价值]

**Independent Test**: [How to verify this story works on its own]  
**独立测试**：[如何验证该故事可独立运行]

### Tests for User Story 1 (OPTIONAL - only if tests requested) ⚠️ / 用户故事 1 的测试（可选 - 仅在要求测试时）⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**  
> **注意：先写测试并确保失败，再进行实现**

- [ ] T010 [P] [US1] Contract test for [endpoint] in tests/contract/test_[name].py / [US1] 在 tests/contract/test_[name].py 中为 [endpoint] 编写契约测试
- [ ] T011 [P] [US1] Integration test for [user journey] in tests/integration/test_[name].py / [US1] 在 tests/integration/test_[name].py 中为 [用户旅程] 编写集成测试

### Implementation for User Story 1 / 用户故事 1 的实现

- [ ] T012 [P] [US1] Create [Entity1] model in src/models/[entity1].py / [US1] 在 src/models/[entity1].py 中创建 [实体1] 模型
- [ ] T013 [P] [US1] Create [Entity2] model in src/models/[entity2].py / [US1] 在 src/models/[entity2].py 中创建 [实体2] 模型
- [ ] T014 [US1] Implement [Service] in src/services/[service].py (depends on T012, T013) / [US1] 在 src/services/[service].py 中实现 [服务]（依赖 T012、T013）
- [ ] T015 [US1] Implement [endpoint/feature] in src/[location]/[file].py / [US1] 在 src/[location]/[file].py 中实现 [接口/功能]
- [ ] T016 [US1] Add validation and error handling / [US1] 添加校验与错误处理
- [ ] T017 [US1] Add logging for user story 1 operations / [US1] 为用户故事 1 的操作添加日志

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently  
**检查点**：此时用户故事 1 应可独立运行并可独立测试

---

## Phase 4: User Story 2 - [Title] (Priority: P2) / 阶段 4：用户故事 2 - [标题]（优先级：P2）

**Goal**: [Brief description of what this story delivers]  
**目标**：[简要说明该故事交付的价值]

**Independent Test**: [How to verify this story works on its own]  
**独立测试**：[如何验证该故事可独立运行]

### Tests for User Story 2 (OPTIONAL - only if tests requested) ⚠️ / 用户故事 2 的测试（可选 - 仅在要求测试时）⚠️

- [ ] T018 [P] [US2] Contract test for [endpoint] in tests/contract/test_[name].py / [US2] 在 tests/contract/test_[name].py 中为 [endpoint] 编写契约测试
- [ ] T019 [P] [US2] Integration test for [user journey] in tests/integration/test_[name].py / [US2] 在 tests/integration/test_[name].py 中为 [用户旅程] 编写集成测试

### Implementation for User Story 2 / 用户故事 2 的实现

- [ ] T020 [P] [US2] Create [Entity] model in src/models/[entity].py / [US2] 在 src/models/[entity].py 中创建 [实体] 模型
- [ ] T021 [US2] Implement [Service] in src/services/[service].py / [US2] 在 src/services/[service].py 中实现 [服务]
- [ ] T022 [US2] Implement [endpoint/feature] in src/[location]/[file].py / [US2] 在 src/[location]/[file].py 中实现 [接口/功能]
- [ ] T023 [US2] Integrate with User Story 1 components (if needed) / [US2] 与用户故事 1 组件集成（如需要）

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently  
**检查点**：此时用户故事 1 和 2 均可独立运行

---

## Phase 5: User Story 3 - [Title] (Priority: P3) / 阶段 5：用户故事 3 - [标题]（优先级：P3）

**Goal**: [Brief description of what this story delivers]  
**目标**：[简要说明该故事交付的价值]

**Independent Test**: [How to verify this story works on its own]  
**独立测试**：[如何验证该故事可独立运行]

### Tests for User Story 3 (OPTIONAL - only if tests requested) ⚠️ / 用户故事 3 的测试（可选 - 仅在要求测试时）⚠️

- [ ] T024 [P] [US3] Contract test for [endpoint] in tests/contract/test_[name].py / [US3] 在 tests/contract/test_[name].py 中为 [endpoint] 编写契约测试
- [ ] T025 [P] [US3] Integration test for [user journey] in tests/integration/test_[name].py / [US3] 在 tests/integration/test_[name].py 中为 [用户旅程] 编写集成测试

### Implementation for User Story 3 / 用户故事 3 的实现

- [ ] T026 [P] [US3] Create [Entity] model in src/models/[entity].py / [US3] 在 src/models/[entity].py 中创建 [实体] 模型
- [ ] T027 [US3] Implement [Service] in src/services/[service].py / [US3] 在 src/services/[service].py 中实现 [服务]
- [ ] T028 [US3] Implement [endpoint/feature] in src/[location]/[file].py / [US3] 在 src/[location]/[file].py 中实现 [接口/功能]

**Checkpoint**: All user stories should now be independently functional  
**检查点**：所有用户故事应可独立运行

---

[Add more user story phases as needed, following the same pattern]  
[按需添加更多用户故事阶段，遵循相同格式]

---

## Phase N: Polish & Cross-Cutting Concerns / 阶段 N：完善与横切关注点

**Purpose**: Improvements that affect multiple user stories  
**目标**：影响多个用户故事的改进事项

- [ ] TXXX [P] Documentation updates in docs/ / 在 docs/ 中更新文档
- [ ] TXXX Code cleanup and refactoring / 代码清理与重构
- [ ] TXXX Performance optimization across all stories / 所有故事的性能优化
- [ ] TXXX [P] Additional unit tests (if requested) in tests/unit/ / 在 tests/unit/ 中补充单元测试（如要求）
- [ ] TXXX Security hardening / 安全加固
- [ ] TXXX Run quickstart.md validation / 运行 quickstart.md 验证

---

## Dependencies & Execution Order / 依赖与执行顺序

### Phase Dependencies / 阶段依赖

- **Setup (Phase 1)**: No dependencies - can start immediately / **准备（阶段 1）**：无依赖，可立即开始
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories / **基础（阶段 2）**：依赖准备完成，阻塞所有用户故事
- **User Stories (Phase 3+)**: All depend on Foundational phase completion / **用户故事（阶段 3+）**：依赖基础阶段完成
  - User stories can then proceed in parallel (if staffed) / 用户故事可并行开展（如有人员）
  - Or sequentially in priority order (P1 → P2 → P3) / 或按优先级顺序推进（P1 → P2 → P3）
- **Polish (Final Phase)**: Depends on all desired user stories being complete / **完善（最终阶段）**：依赖所需用户故事完成

### User Story Dependencies / 用户故事依赖

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories / **用户故事 1（P1）**：基础阶段完成后可开始，不依赖其他故事
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable / **用户故事 2（P2）**：基础阶段完成后可开始，可能集成 US1 但需可独立测试
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable / **用户故事 3（P3）**：基础阶段完成后可开始，可能集成 US1/US2 但需可独立测试

### Within Each User Story / 每个用户故事内

- Tests (if included) MUST be written and FAIL before implementation / 测试（如包含）必须先写且失败，再实现
- Models before services / 先模型后服务
- Services before endpoints / 先服务后接口
- Core implementation before integration / 先核心实现后集成
- Story complete before moving to next priority / 完成本故事后再推进下一优先级

### Parallel Opportunities / 并行机会

- All Setup tasks marked [P] can run in parallel / 所有标记 [P] 的准备任务可并行
- All Foundational tasks marked [P] can run in parallel (within Phase 2) / 基础阶段标记 [P] 的任务可并行
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows) / 基础阶段完成后，用户故事可并行开展（如团队资源允许）
- All tests for a user story marked [P] can run in parallel / 同一用户故事中标记 [P] 的测试可并行
- Models within a story marked [P] can run in parallel / 同一故事中标记 [P] 的模型可并行
- Different user stories can be worked on in parallel by different team members / 不同用户故事可由不同成员并行推进

---

## Parallel Example: User Story 1 / 并行示例：用户故事 1

```bash
T010, T011, T012, T013 can run in parallel for US1
US1 可并行执行示例：T010、T011、T012、T013
```
