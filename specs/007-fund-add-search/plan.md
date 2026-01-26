# Implementation Plan: Fund Add Search Filter

**Branch**: `007-fund-add-search` | **Date**: 2026-01-22 | **Spec**: `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/spec.md`
**Input**: Feature specification from `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `/Users/hitol/code/ai/leek-fund/.specify/templates/commands/plan.md` for the execution workflow.

## Summary

When users click the add button for a fund that already exists in the middle list, the lower list filters to that exact fund with a clear way to return to the full list, while normal add behavior remains unchanged for new funds.  
当用户点击添加按钮且该基金已存在于中间列表时，下方列表会过滤为该基金并提供清晰的返回全量列表入口；对于新增基金，原有添加行为保持不变。

## Technical Context

**Language/Version**: Rust 1.70+, TypeScript (React 18)  
**语言/版本**：Rust 1.70+、TypeScript（React 18）

**Primary Dependencies**: Tauri 1.5, Vite 5, React 18, @tauri-apps/api, serde/serde_json, reqwes  
**主要依赖**：Tauri 1.5、Vite 5、React 18、@tauri-apps/api、serde/serde_json、reqwes

**Storage**: Local JSON file in Tauri app data directory  
**存储**：Tauri 应用数据目录中的本地 JSON 文件

**Testing**: cargo test, cargo clippy, manual UI checks via npm run tauri:dev  
**测试**：cargo test、cargo clippy、通过 npm run tauri:dev 进行手动界面验证

**Target Platform**: Desktop (Tauri)  
**目标平台**：桌面端（Tauri）

**Project Type**: Single desktop application  
**项目类型**：单体桌面应用

**Performance Goals**: Existing-fund filter result visible within 1 second for typical lists (<= 1,000 funds)  
**性能目标**：典型列表规模（<= 1,000 支基金）下，已存在基金的过滤结果在 1 秒内可见

**Constraints**: Offline-capable, no duplicate entries, non-blocking feedback  
**约束**：离线可用、不产生重复条目、提示不阻塞操作

**Scale/Scope**: Single-user desktop usage, fund lists up to a few thousand items  
**规模/范围**：单用户桌面使用场景，基金列表规模为数千条以内

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- No enforceable rules found in `/Users/hitol/code/ai/leek-fund/.specify/memory/constitution.md` (placeholders only). Gate passes.  
  在 `/Users/hitol/code/ai/leek-fund/.specify/memory/constitution.md` 中未发现可执行规则（仅占位内容），因此通过。

## Project Structure

### Documentation (this feature)

```text
/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/
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

**Structure Decision**: Use the existing Tauri + React single-app structure with UI logic in `src/` and native services in `src-tauri/`.  
**结构选择**：沿用现有的 Tauri + React 单体应用结构，界面逻辑在 `src/`，原生服务在 `src-tauri/`。

## Phase 0: Research

- Resolved fund identity matching using unique fund identifiers to prevent false positives.  
  通过基金唯一标识进行匹配，避免同名误判。
- Defined the filter UX: single-item lower list, non-blocking “already exists” hint, and a one-step clear action.  
  明确过滤体验：下方列表仅显示单项、提示不阻塞、提供一步清除。
- Confirmed empty-state behavior when the filtered fund is not present in the lower list.  
  明确当下方列表无匹配项时需显示空状态提示。

Output: `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/research.md`

## Phase 1: Design & Contracts

- Data model covers Fund, list views, and filter state.  
  数据模型覆盖基金、列表视图与过滤状态。
- Contracts define add and list/filter interactions for the UI/service boundary.  
  合同定义添加与列表过滤交互，约束界面与服务边界。
- Quickstart documents the manual verification flow.  
  Quickstart 记录手动验证流程。

Outputs:
- `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/data-model.md`
- `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/contracts/fund-list.openapi.yaml`
- `/Users/hitol/code/ai/leek-fund/specs/007-fund-add-search/quickstart.md`

## Phase 1b: Agent Context Update

- Run `/Users/hitol/code/ai/leek-fund/.specify/scripts/bash/update-agent-context.sh codex` after plan completion.  
  计划完成后运行上述脚本以更新代理上下文。

## Phase 2: Task Planning

- Not created by `/speckit.plan`. Use `/speckit.tasks` next.  
  此阶段不由 `/speckit.plan` 生成，请使用 `/speckit.tasks`。

## Constitution Check (Post-Design)

- No constitution gates defined; no violations detected.  
  未定义宪法门禁，无违规项。

## Complexity Tracking

No constitution violations to justify.  
无需复杂度例外说明。
