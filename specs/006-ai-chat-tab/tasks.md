---
description: "Task list template for feature implementation"
---

# Tasks: AI 对话 Tab

**Input**: Design documents from `/specs/006-ai-chat-tab/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Tests are OPTIONAL - not requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 确认新增 AI 对话 Tab 的路由/菜单挂载点并记录在 docs（如有）
- [X] T002 [P] 新建前端页面目录结构 `src/pages/ai-chat/`
- [X] T003 [P] 新建前端组件目录结构 `src/components/ai-chat/`
- [X] T004 [P] 新建前端服务目录结构 `src/services/ai-chat/`
- [X] T005 [P] 新建前端类型定义文件 `src/types/ai-chat.ts`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

- [X] T006 定义 SQLite 表结构迁移/初始化逻辑，包含 `session` 与 `session_chat_messages`（位置：`src-tauri/src/` 下数据库初始化模块）
- [X] T007 [P] 创建后端数据访问层：Session 数据模型与 CRUD（`src-tauri/src/` 下 db/session.rs）
- [X] T008 [P] 创建后端数据访问层：SessionChatMessage 数据模型与 CRUD（`src-tauri/src/` 下 db/session_chat_message.rs）
- [X] T009 [P] 创建后端数据访问层：Agent 元数据结构（`src-tauri/src/` 下 db/agent.rs）
- [X] T010 建立后端统一错误类型与错误映射（`src-tauri/src/errors.rs`）
- [X] T011 建立后端应用数据目录与 SQLite 连接管理（`src-tauri/src/db/mod.rs`）
- [X] T012 创建后端会话服务封装（创建/查找最近会话/加载历史）（`src-tauri/src/services/session_service.rs`）
- [X] T013 创建后端消息服务封装（保存消息/标记未保存）（`src-tauri/src/services/message_service.rs`）
- [X] T014 创建后端 Agent 选择器（默认对话代理）（`src-tauri/src/services/agent_router.rs`）

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - 在新 Tab 中发起 AI 对话 (Priority: P1) 🎯 MVP

**Goal**: 用户在新 Tab 中发送文本并接收流式回复

**Independent Test**: 打开 AI 对话 Tab，发送文本并看到流式回复与历史加载

### Implementation for User Story 1

- [X] T015 [US1] 新增 AI 对话 Tab 入口并挂载页面 `src/pages/ai-chat/index.tsx`
- [X] T016 [US1] 实现聊天页面基础布局（消息区 + 底部输入框）`src/pages/ai-chat/index.tsx`
- [X] T017 [P] [US1] 实现消息列表组件 `src/components/ai-chat/MessageList.tsx`
- [X] T018 [P] [US1] 实现消息气泡组件 `src/components/ai-chat/MessageItem.tsx`
- [X] T019 [P] [US1] 实现聊天输入组件 `src/components/ai-chat/MessageInput.tsx`
- [X] T020 [US1] 维护前端会话状态与消息状态 `src/pages/ai-chat/useChatState.ts`
- [X] T021 [US1] 前端创建/加载最近会话调用封装 `src/services/ai-chat/sessionApi.ts`
- [X] T022 [US1] 前端获取历史消息调用封装 `src/services/ai-chat/messageApi.ts`
- [X] T023 [US1] 前端发送消息并接收 SSE 流式回复逻辑 `src/services/ai-chat/streamApi.ts`
- [X] T024 [US1] 前端接收流式回复并逐步拼接显示 `src/pages/ai-chat/useStreamReply.ts`
- [X] T025 [US1] 前端消息校验（空白消息拦截）`src/components/ai-chat/MessageInput.tsx`
- [X] T026 [US1] 后端新增创建会话接口（创建或返回最近会话）`src-tauri/src/api/session.rs`
- [X] T027 [US1] 后端新增查询最近会话接口 `src-tauri/src/api/session.rs`
- [X] T028 [US1] 后端新增查询会话消息接口 `src-tauri/src/api/message.rs`
- [X] T029 [US1] 后端新增 SSE 流式回复接口（创建用户消息 + 流式回复）`src-tauri/src/api/stream.rs`
- [X] T030 [US1] 后端对接 LLM 代理（对话代理）并生成流式片段 `src-tauri/src/services/chat_agent.rs`
- [X] T031 [US1] 后端流式过程中保存 assistant 消息 `src-tauri/src/services/message_service.rs`
- [X] T032 [US1] 前端样式与滚动行为处理 `src/pages/ai-chat/ai-chat.css`

**Checkpoint**: User Story 1 should be fully functional and independently testable

---

## Phase 4: User Story 2 - 会话可持续保存与查看 (Priority: P2)

**Goal**: 用户可在重新进入时查看历史消息

**Independent Test**: 发送消息后重启应用，历史消息可加载显示

### Implementation for User Story 2

- [X] T033 [US2] 前端应用启动时加载最近会话并渲染历史 `src/pages/ai-chat/index.tsx`
- [X] T034 [US2] 前端历史消息分页/批量加载策略（初版可全量）`src/services/ai-chat/messageApi.ts`
- [X] T035 [US2] 后端会话/消息加载接口增加排序与限制参数 `src-tauri/src/api/message.rs`
- [X] T036 [US2] 后端保存 `updated_at` 触发逻辑 `src-tauri/src/db/session.rs`

**Checkpoint**: User Story 2 should be functional and independently testable

---

## Phase 5: User Story 3 - 对话不可用时的提示 (Priority: P3)

**Goal**: 对话创建或回复失败时给出清晰提示

**Independent Test**: 模拟后端错误，前端展示错误提示并保留已显示内容

### Implementation for User Story 3

- [X] T037 [US3] 前端显示创建会话失败提示 `src/pages/ai-chat/index.tsx`
- [X] T038 [US3] 前端显示流式回复中断提示（可重试）`src/pages/ai-chat/index.tsx`
- [X] T039 [US3] 前端标记消息未保存状态 `src/components/ai-chat/MessageItem.tsx`
- [X] T040 [US3] 后端持久化失败时返回可区分错误码 `src-tauri/src/errors.rs`
- [X] T041 [US3] 后端流式中断时返回终止事件 `src-tauri/src/api/stream.rs`

**Checkpoint**: User Story 3 should be functional and independently testable

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T042 [P] 更新功能文档与截图指引（如需）`README.md`
- [ ] T043 清理未使用代码并统一命名 `src/` 与 `src-tauri/src/`
- [ ] T044 运行 quickstart 步骤自检并记录结果 `specs/006-ai-chat-tab/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational - Builds on US1 data paths
- **User Story 3 (P3)**: Can start after Foundational - Adds failure handling across US1/US2

### Parallel Opportunities

- Phase 1 tasks marked [P] can run in parallel
- Phase 2 data access layer tasks (T007–T009) can run in parallel
- US1 UI components (T017–T019) can run in parallel

---

## Parallel Example: User Story 1

```bash
Task: "实现消息列表组件 src/components/ai-chat/MessageList.tsx"
Task: "实现消息气泡组件 src/components/ai-chat/MessageItem.tsx"
Task: "实现聊天输入组件 src/components/ai-chat/MessageInput.tsx"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. STOP and validate User Story 1 independently

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Demo
3. Add User Story 2 → Test independently → Demo
4. Add User Story 3 → Test independently → Demo
