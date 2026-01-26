# Implementation Plan: AI 对话 Tab

**Branch**: `006-ai-chat-tab` | **Date**: 2026-01-22 | **Spec**: `/Users/hitol/code/ai/leek-fund/specs/006-ai-chat-tab/spec.md`
**Input**: Feature specification from `/specs/006-ai-chat-tab/spec.md`

## Summary

新增一个 AI 对话 Tab，支持创建/继续最近会话、发送文本消息、接收流式回复，并将会话与消息持久化以便回看。

## Technical Context

**Language/Version**: Rust 1.70+, TypeScript (React 18)  
**Primary Dependencies**: Tauri 1.5, @tauri-apps/api, serde/serde_json, reqwest  
**Storage**: SQLite（本地应用数据目录）  
**Testing**: cargo test, cargo clippy, npm run tauri:dev  
**Target Platform**: 桌面端（Tauri）  
**Project Type**: 前后端同仓（src + src-tauri）  
**Performance Goals**: 流式回复在生成过程中平均每 2 秒内可见更新  
**Constraints**: 单用户本地使用、离线可用（当对话服务本地可用时）  
**Scale/Scope**: 单用户会话与消息历史持久化

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- 发现的 Constitution 文件为占位模板，未包含可执行约束；本阶段无阻塞性条款。
- Phase 1 设计后复核：未发现新增约束或冲突。

## Project Structure

### Documentation (this feature)

```text
specs/006-ai-chat-tab/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/
├── components/
├── pages/
├── services/
└── types/

src-tauri/
├── src/
└── tauri.conf.json
```

**Structure Decision**: 采用现有 Tauri 单仓结构（前端在 `src/`，后端在 `src-tauri/`），新增 AI 对话页面与对应后端接口/数据层。

## Complexity Tracking

无新增复杂度例外。
