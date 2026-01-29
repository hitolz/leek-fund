# Implementation Plan: Fund Demo UI Redesign / 基金演示页面重设计

**Branch**: `009-fund-ui-demo` | **Date**: 2026-01-27 | **Spec**: `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/spec.md`
**Input**: Feature specification from `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Deliver a non-shipping demo artifact in a new `demo/` folder as a single HTML
page that preserves current core behaviors (list toggle, list selection, fund
add/remove, detail view, holdings calculations, and tri-state sorting) using
embedded sample data and deterministic UI-side formulas for illustration.
在新的 `demo/` 目录中交付一个不随产品发布的演示产物（单页 HTML），使用内嵌
示例数据与确定性公式展示当前核心行为（列表显隐、列表选择、基金增删、详情、
持仓计算与三态排序）。

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: HTML5, CSS3, JavaScript (ES2020) for demo artifact; Rust 1.70+ and React 18 + TypeScript remain unchanged  
**Primary Dependencies**: None required beyond browser capabilities; chart rendered via inline SVG  
**Storage**: N/A for demo persistence; sample data embedded in the page and held in memory  
**Testing**: Manual acceptance walkthrough against spec scenarios; repository compile check via `cargo check` in `/Users/hitol/code/ai/leek-fund/src-tauri`  
**Target Platform**: Desktop browsers and Tauri WebView equivalents on macOS, Windows, Linux  
**Project Type**: Tauri desktop app with a documentation/demo artifact under repository root  
**Performance Goals**: Sorting and selection interactions remain visibly correct within 2 seconds for lists up to 20 funds  
**Constraints**: Single HTML page in `demo/`; no live network calls; deterministic calculations; demo does not alter production data paths  
**Scale/Scope**: 1 demo page; 1 list panel; 1 fund list; 1 detail panel; representative sample data with 3 lists and at least 20 funds total

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Tauri Desktop Architecture: PASS. The demo is a repository artifact that does
  not change the product architecture and remains aligned with desktop targets.
  Tauri 桌面架构：通过。演示为仓库内产物，不改变产品架构，仍面向桌面端。
- Rust Owns Data & Network: PASS with scope note. The demo uses embedded sample
  data and does not introduce any new data/network pathways; production ownership
  remains in Rust.
  Rust 管理数据与网络：通过（附范围说明）。演示使用内嵌示例数据，不引入新的
  数据/网络路径；生产环境的数据归属仍在 Rust。
- UI-Only Frontend: PASS with documentation exception. Any calculations shown in
  the demo are illustrative for the HTML prototype and do not replace Rust
  business rules.
  仅 UI 前端：通过（文档例外说明）。演示中的计算仅用于 HTML 原型展示，不替代
  Rust 业务规则。
- Local-First Persistence & Recovery: PASS. No persistence behavior, storage
  paths, or migrations are changed by this demo artifact.
  本地优先持久化与恢复：通过。该演示产物不改变持久化行为、存储路径或迁移策略。
- Fund List Semantics & Data Integrity: PASS. The plan preserves single active
  list semantics, in-list uniqueness assumptions, and explicit formulas for
  deterministic calculations in the demo documentation.
  基金列表语义与数据完整性：通过。计划保持单一激活列表语义、列表内去重假设，
  并在演示文档中明确确定性计算公式。

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
├── demo/
│   └── index.html                 # Single-page HTML demo artifact
├── specs/
│   └── 009-fund-ui-demo/
│       ├── plan.md
│       ├── research.md
│       ├── data-model.md
│       ├── quickstart.md
│       └── contracts/
├── src/
└── src-tauri/
```

**Structure Decision**: Use the existing Tauri repository layout and add a
root-level `demo/` folder for the single HTML prototype, while keeping all
planning artifacts under `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo`.
结构决策：沿用现有 Tauri 仓库结构，在根目录新增 `demo/` 放置单页 HTML 原型，
并将所有规划产物放在 `/Users/hitol/code/ai/leek-fund/specs/009-fund-ui-demo`。

## Phase 0: Research / 阶段 0：研究

Research focuses on demo-safe decisions that avoid architecture violations and
resolve technical unknowns for tri-state sorting, default order restoration, and
deterministic calculation display rules.
研究阶段聚焦于不会破坏架构约束的演示决策，解决三态排序、默认顺序恢复与确定性
计算展示规则等技术不确定项。

## Phase 1: Design & Contracts / 阶段 1：设计与契约

Design outputs include a data model describing lists, funds, holdings, and sort
preference; a contract document describing demo data shapes and user actions; and
a quickstart that guides reviewers through acceptance scenarios.
设计产物包括描述列表、基金、持仓与排序偏好的数据模型；描述演示数据结构与用户
动作的契约文档；以及引导评审完成验收场景的 quickstart。

## Phase 2: Implementation Planning / 阶段 2：实现规划

Implementation planning will sequence demo layout, interaction wiring, calculation
rules, sorting behavior, and acceptance verification, without changing production
Rust ownership of data and network behavior.
实现规划将安排演示布局、交互联动、计算规则、排序行为与验收验证的顺序，同时不
改变生产环境中 Rust 对数据与网络行为的归属。

## Constitution Check (Post-Design) / 宪章检查（设计后复核）

- Tauri Desktop Architecture: PASS. Artifacts are documentation/demo only and do
  not change desktop delivery requirements.
  Tauri 桌面架构：通过。产物为文档/演示用途，不改变桌面交付要求。
- Rust Owns Data & Network: PASS. Contracts and models describe demo data shapes
  without introducing UI-side network or persistence responsibilities.
  Rust 管理数据与网络：通过。契约与模型仅描述演示数据结构，不引入 UI 侧网络或持久化职责。
- UI-Only Frontend: PASS with scope note. Any demo-side calculations are explicitly
  documented as prototype behavior and do not revise production Rust rules.
  仅 UI 前端：通过（附范围说明）。演示侧计算明确标注为原型行为，不修改生产 Rust 规则。
- Local-First Persistence & Recovery: PASS. No changes to storage paths, migrations,
  or backup behaviors are introduced by planning artifacts.
  本地优先持久化与恢复：通过。规划产物未引入存储路径、迁移或备份行为的变更。
- Fund List Semantics & Data Integrity: PASS. Data model and contracts preserve list
  scoping, explicit formulas, and stable ordering rules.
  基金列表语义与数据完整性：通过。数据模型与契约保持列表作用域、明确公式与稳定排序规则。

## Complexity Tracking

No constitution violations requiring justification are identified for this
planning scope.
当前规划范围未识别出需要额外论证的宪章违例。
