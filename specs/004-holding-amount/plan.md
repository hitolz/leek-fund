# Implementation Plan / 实施计划: Holding Amount in Fund Detail Panel

**Branch**: `004-holding-amount` | **Date**: 2026-01-21 | **Spec**: /Users/hitol/code/ai/leek-fund/specs/004-holding-amount/spec.md  
**分支**：`004-holding-amount` | **日期**：2026-01-21 | **规格**：/Users/hitol/code/ai/leek-fund/specs/004-holding-amount/spec.md  
**Input**: Feature specification from `/specs/004-holding-amount/spec.md`  
**输入**：来自 `/specs/004-holding-amount/spec.md` 的功能规格

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.  
**说明**：本模板由 `/speckit.plan` 命令生成，执行流程见 `.specify/templates/commands/plan.md`。

## Summary / 概要

Add a holding section in the right fund detail panel that is scoped to group + fund, persists locally, and shows computed holding amount plus daily change amount.  
在右侧基金详情面板增加持仓区域，按分组+基金绑定，本地持久化，并展示计算出的持仓金额与当日涨跌金额。

## Technical Context / 技术背景

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
  需要填写：用项目的技术细节替换本节内容。本结构用于指导迭代过程。
-->

**Language/Version**: Rust 1.70+, TypeScript (React 18) / **语言/版本**：Rust 1.70+，TypeScript（React 18）  
**Primary Dependencies**: Tauri 1.5, Vite 5, @tauri-apps/api / **主要依赖**：Tauri 1.5，Vite 5，@tauri-apps/api  
**Storage**: Local JSON file in Tauri app data directory / **存储**：Tauri 应用数据目录中的本地 JSON 文件  
**Testing**: cargo test, cargo clippy / **测试**：cargo test、cargo clippy  
**Target Platform**: macOS, Windows, Linux (desktop) / **目标平台**：macOS、Windows、Linux（桌面）  
**Project Type**: Desktop app (Tauri + React) / **项目类型**：桌面应用（Tauri + React）  
**Performance Goals**: UI remains responsive and smooth (60 fps target) / **性能目标**：界面响应流畅（目标 60fps）  
**Constraints**: No blocking I/O on UI thread; local-only persistence / **约束**：UI 线程不阻塞 I/O；仅本地持久化  
**Scale/Scope**: Single-user local app, typical list sizes (<= 50 lists, <= 200 funds per list) / **规模/范围**：单用户本地应用，典型数据规模（<= 50 列表，每列表 <= 200 基金）

## Constitution Check / 宪法检查

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*  
*关卡：在第 0 阶段调研前必须通过，第 1 阶段设计后需复检。*

- Real-time data from `fundgz.1234567.com.cn`, history from `fund.eastmoney.com/pingzhongdata/{code}.js`, and display data timestamp / 实时数据来自 `fundgz.1234567.com.cn`，历史数据来自 `fund.eastmoney.com/pingzhongdata/{code}.js`，并标注数据时间戳
- Local persistence and migration strategy defined, no loss of historical data / 本地持久化与迁移策略已定义，不丢失历史数据
- macOS/Windows/Linux behavior consistent, main thread non-blocking / macOS/Windows/Linux 行为一致，主线程不阻塞
- Only approved external services, no telemetry/data upload / 仅访问批准的外部服务，无遥测/数据上传
- Core module test plan defined, Tauri command contracts aligned / 核心模块测试计划明确，Tauri 命令契约已对齐

## Project Structure / 项目结构

### Documentation (this feature) / 文档（本功能）

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root) / 源码（仓库根目录）
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
  需要填写：用本功能的实际目录结构替换占位树。删除不用的选项，
  并扩展选中结构到真实路径（例如 apps/admin、packages/something）。
  最终交付的计划中不得包含 Option 标签。
-->

```text
src/
├── components/
├── hooks/
└── types/

src-tauri/
└── src/
    ├── commands.rs
    ├── models.rs
    └── modules/

tests/
```

**Structure Decision**: Single desktop app with shared frontend (`src/`) and Tauri backend (`src-tauri/`).  
**结构选择**：单体桌面应用，前端位于 `src/`，Tauri 后端位于 `src-tauri/`。

## Complexity Tracking / 复杂度跟踪

> **Fill ONLY if Constitution Check has violations that must be justified**  
> **仅当宪法检查存在需要说明的违规项时填写**

| Violation / 违反项 | Why Needed / 必要原因 | Simpler Alternative Rejected Because / 为什么拒绝更简单方案 |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] / [例如：第 4 个项目] | [current need] / [当前需求] | [why 3 projects insufficient] / [为何 3 个项目不足] |
| [e.g., Repository pattern] / [例如：仓储模式] | [specific problem] / [具体问题] | [why direct DB access insufficient] / [为何直接访问数据库不足] |
