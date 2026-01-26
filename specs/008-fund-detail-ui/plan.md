# Implementation Plan: Fund Detail UI Alignment

**Branch**: `008-fund-detail-ui` | **Date**: 2026-01-26 | **Spec**: `/Users/hitol/code/ai/leek-fund/specs/008-fund-detail-ui/spec.md`
**Input**: Feature specification from `/Users/hitol/code/ai/leek-fund/specs/008-fund-detail-ui/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `/Users/hitol/code/ai/leek-fund/.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Align the three-column fund detail UI to the reference image, add sorting options
for the middle list, and complete the holding inputs with cost price calculation
and backend-provided sort fields, plus macOS menu bar refresh controls.  
对齐三列基金详情 UI 与参考图，补齐中间列表排序、持仓输入与成本价计算（排序字段由后端返回），
并在 macOS 菜单栏提供刷新选项与勾选状态。

## Technical Context

**Language/Version**: Rust 1.70+, TypeScript (React 18)  
**语言/版本**：Rust 1.70+、TypeScript（React 18）

**Primary Dependencies**: Tauri 1.5, Vite 5, React 18, @tauri-apps/api, serde/serde_json, reqwest, SQLx (SQLite)  
**主要依赖**：Tauri 1.5、Vite 5、React 18、@tauri-apps/api、serde/serde_json、reqwest、SQLx（SQLite）

**Storage**: Local SQLite in Tauri app data directory  
**存储**：Tauri 应用数据目录中的本地 SQLite

**Testing**: cargo test, cargo clippy, manual UI checks via npm run tauri:dev  
**测试**：cargo test、cargo clippy、通过 npm run tauri:dev 进行手动界面验证

**Target Platform**: Desktop (macOS, Windows, Linux)  
**目标平台**：桌面端（macOS、Windows、Linux）

**Project Type**: Single desktop application (React UI + Tauri Rust backend)  
**项目类型**：单体桌面应用（React UI + Tauri Rust 后端）

**Performance Goals**: List render and resort visible within 1 second for lists up to 1,000 funds  
**性能目标**：在列表规模不超过 1,000 支基金时，列表渲染与重新排序在 1 秒内可见

**Constraints**: Offline-capable for existing data, no duplicate funds per list, backend-provided sort fields  
**约束**：已缓存数据离线可用、单列表内不重复、排序字段由后端返回

**Scale/Scope**: Single-user desktop usage, lists with up to a few thousand items  
**规模/范围**：单用户桌面场景，列表规模为数千条以内

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Tauri Desktop Architecture: feature remains a Tauri + Rust + React app targeting
  macOS/Windows/Linux. Tauri 桌面架构：功能仍为 Tauri + Rust + React 应用，
  覆盖 macOS/Windows/Linux。
- Rust Owns Data & Network: all network, parsing, storage, and migrations handled
  in Rust; frontend only calls typed Tauri commands. Rust 管理数据与网络：所有网络、
  解析、存储与迁移在 Rust 中完成，前端仅调用类型化 Tauri 命令。
- UI-Only Frontend: no business logic or validation bypass in UI layers. 仅 UI 前端：
  UI 层不得包含业务逻辑或绕过校验。
- Local-First Persistence & Recovery: SQLite storage, migration/backup behavior
  documented and preserved for this feature. 本地优先持久化与恢复：SQLite 存储，
  迁移与备份行为需文档化并保持一致。
- Fund List Semantics & Data Integrity: list uniqueness, fund code validation,
  and deterministic calculations upheld. 基金列表语义与数据完整性：保持列表去重、
  基金码校验与确定性计算。

Gate status: PASS. No violations detected.  
门禁状态：通过，无违规项。

## Project Structure

### Documentation (this feature)

```text
/Users/hitol/code/ai/leek-fund/specs/008-fund-detail-ui/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
└── tasks.md
```

### Source Code (repository root)

```text
/Users/hitol/code/ai/leek-fund/
├── src/
│   ├── components/
│   ├── hooks/
│   ├── pages/
│   ├── services/
│   ├── types/
│   └── utils/
├── src-tauri/
│   └── src/
└── tests/
```

**Structure Decision**: Use the existing Tauri + React single-app structure with UI
logic in `src/` and data/network/storage logic in `src-tauri/`.  
**结构选择**：沿用现有的 Tauri + React 单体应用结构，界面逻辑在 `src/`，
数据/网络/存储逻辑在 `src-tauri/`。

## Phase 0: Research

- Confirmed holding input and daily change amount behavior by aligning with
  holding and amount-calculation specs (004/005).  
  结合 004/005 规范确认持仓输入与当日涨跌额行为。
- Standardized sort behavior: backend-provided values, missing values sorted to
  the end, global sort state per session.  
  标准化排序行为：字段来自后端、缺失值置底、会话内全局排序状态。
- Defined cost price edge case handling (zero shares shows `--` with message).  
  明确成本价边界处理（份额为零时显示 `--` 并提示）。
- Confirmed macOS refresh controls live in menu bar with checkmark state.  
  确认 macOS 刷新控制位于菜单栏并显示勾选状态。

Output: `/Users/hitol/code/ai/leek-fund/specs/008-fund-detail-ui/research.md`

## Phase 1: Design & Contracts

- Data model expanded to include holding amount/shares, cost price, and list
  summary fields (holding amount, daily change amount) provided by the backend.  
  数据模型补齐持仓金额/份额、成本价，以及列表摘要字段（持仓金额、当日涨跌额）。
- Contracts updated to expose list summary fields and holding CRUD operations.  
  合同更新以暴露列表摘要字段与持仓读写操作。
- Quickstart documents manual verification for sorting, cost price, and macOS
  refresh menu behavior.  
  Quickstart 记录排序、成本价与菜单栏刷新的手动验证流程。

Outputs:
- `/Users/hitol/code/ai/leek-fund/specs/008-fund-detail-ui/data-model.md`
- `/Users/hitol/code/ai/leek-fund/specs/008-fund-detail-ui/contracts/fund-detail-ui.openapi.yaml`
- `/Users/hitol/code/ai/leek-fund/specs/008-fund-detail-ui/quickstart.md`

## Phase 1b: Agent Context Update

- Run `/Users/hitol/code/ai/leek-fund/.specify/scripts/bash/update-agent-context.sh codex` after Phase 1 outputs.  
  Phase 1 产出后运行该脚本更新代理上下文。

## Phase 2: Task Planning

- Not created by `/speckit.plan`. Use `/speckit.tasks` next.  
  此阶段不由 `/speckit.plan` 生成，请使用 `/speckit.tasks`。

## Constitution Check (Post-Design)

- All principles satisfied; no deviations introduced.  
  已满足所有原则，无新增偏离。

## Complexity Tracking

No constitution violations to justify.  
无需复杂度例外说明。
