# Implementation Plan: Client UI Redesign From Demo / 参照演示的客户端界面重设计

**Branch**: `010-client-ui-redesign` | **Date**: 2026-01-27 | **Spec**: `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/spec.md`
**Input**: Feature specification from `/Users/hitol/code/ai/leek-fund/specs/010-client-ui-redesign/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Redesign the client UI to align with the demo page’s layout and styling while
preserving all current core behaviors (list selection, fund selection, add/remove,
holdings, and sorting). The plan focuses on UI composition and styling without
changing Rust-owned data, network, or persistence behavior.
按照 demo 页面的布局与样式重做客户端 UI，同时保留当前核心行为（列表选择、基金选择、
增删、持仓与排序）。规划聚焦 UI 组合与样式，不改变 Rust 侧数据、网络与持久化行为。

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: TypeScript (React 18), Rust 1.70+  
**Primary Dependencies**: React 18, @tauri-apps/api, existing UI component patterns in the client  
**Storage**: SQLite via Rust (no changes for this feature)  
**Testing**: Manual UI walkthrough; `npm run tauri:dev` or `npm run tauri:build`; `cargo check` for Rust compile verification  
**Target Platform**: Tauri desktop app (macOS, Windows, Linux)  
**Project Type**: Desktop app with React frontend and Rust backend  
**Performance Goals**: UI interactions remain responsive and sorting feedback is visible within 2 seconds for lists ≥30 funds  
**Constraints**: Must preserve current data ownership boundaries and behavior parity; redesign is UI-only  
**Scale/Scope**: Existing client page(s) updated to match demo layout and styling; no new user flows introduced

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Tauri Desktop Architecture: PASS. The redesign remains within the existing
  Tauri desktop app boundaries and does not add new delivery surfaces.
  Tauri 桌面架构：通过。重设计保持在现有 Tauri 桌面应用范围内，不新增交付面。
- Rust Owns Data & Network: PASS. Data fetching, parsing, and storage remain in
  Rust; the UI changes only presentation and interaction layout.
  Rust 管理数据与网络：通过。数据获取、解析与存储仍由 Rust 负责；UI 仅调整展示与交互布局。
- UI-Only Frontend: PASS. The work focuses on UI composition and styling without
  moving business rules or validation into the frontend.
  仅 UI 前端：通过。工作聚焦 UI 组合与样式，不将业务规则或校验转移到前端。
- Local-First Persistence & Recovery: PASS. No changes to SQLite persistence,
  migrations, or recovery behavior are required.
  本地优先持久化与恢复：通过。不需要更改 SQLite 持久化、迁移或恢复行为。
- Fund List Semantics & Data Integrity: PASS. List scoping, uniqueness, and
  calculation semantics remain unchanged.
  基金列表语义与数据完整性：通过。列表作用域、去重与计算语义保持不变。

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
/Users/hitol/code/ai/leek-fund/
├── demo/                         # Visual reference (already exists)
│   └── index.html
├── src/                          # React frontend
├── src-tauri/                    # Rust backend
└── specs/
    └── 010-client-ui-redesign/
        ├── plan.md
        ├── research.md
        ├── data-model.md
        ├── quickstart.md
        └── contracts/
```

**Structure Decision**: Use the existing Tauri repository layout; update React
client pages under `/Users/hitol/code/ai/leek-fund/src` to match the demo’s layout
and styling while keeping the demo as the reference artifact.
结构决策：沿用现有 Tauri 仓库结构，更新 `/Users/hitol/code/ai/leek-fund/src` 下的
React 客户端页面以匹配 demo 布局与样式，demo 页面作为参考。

## Phase 0: Research / 阶段 0：研究

Research focuses on translating the demo’s visual layout into the existing client
information architecture, mapping demo elements to current UI sections, and
identifying styling tokens that can be reused.
研究阶段聚焦将 demo 的视觉布局映射到现有客户端信息架构，明确 demo 元素与当前 UI
区域的对应关系，并识别可复用的样式规范。

## Phase 1: Design & Contracts / 阶段 1：设计与契约

Design outputs include an updated data model alignment for UI states, contracts
for any UI-to-backend interactions used by the redesigned page, and a quickstart
walkthrough for validation.
设计产物包括 UI 状态的数据模型对齐、重设计页面所用交互的契约，以及用于验证的
quickstart 走查说明。

## Phase 2: Implementation Planning / 阶段 2：实现规划

Implementation planning will sequence layout updates, component refactors, styling,
and interaction wiring while ensuring behavioral parity.
实现规划将安排布局更新、组件重构、样式调整与交互接线的顺序，同时确保行为一致性。

## Constitution Check (Post-Design) / 宪章检查（设计后复核）

- Tauri Desktop Architecture: PASS. The redesign stays within the existing
  desktop client scope and uses the current delivery pipeline.
  Tauri 桌面架构：通过。重设计保持在现有桌面客户端范围内，沿用当前交付流程。
- Rust Owns Data & Network: PASS. Contracts and UI plans do not move data/network
  responsibilities out of Rust.
  Rust 管理数据与网络：通过。契约与 UI 规划未将数据/网络职责移出 Rust。
- UI-Only Frontend: PASS. Planning focuses on layout and styling changes only.
  仅 UI 前端：通过。规划仅涉及布局与样式调整。
- Local-First Persistence & Recovery: PASS. No persistence or migration changes
  are introduced by this plan.
  本地优先持久化与恢复：通过。本规划不引入持久化或迁移变更。
- Fund List Semantics & Data Integrity: PASS. Semantics and calculation rules
  remain unchanged; only presentation is updated.
  基金列表语义与数据完整性：通过。语义与计算规则不变，仅更新展示方式。

## Complexity Tracking

No constitution violations requiring justification are identified for this
planning scope.
当前规划范围未识别出需要额外论证的宪章违例。
