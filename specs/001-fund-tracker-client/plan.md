# Implementation Plan: Fund List Management
# 实施计划：基金列表管理

**Branch**: `001-fund-tracker-client` | **Date**: 2026-01-21 | **Spec**: /Users/hitol/code/ai/leek-fund/specs/001-fund-tracker-client/spec.md
**分支**：`001-fund-tracker-client` | **日期**：2026-01-21 | **规格**：/Users/hitol/code/ai/leek-fund/specs/001-fund-tracker-client/spec.md

**Input**: Feature specification from `/specs/001-fund-tracker-client/spec.md`
**输入**：来自 `/specs/001-fund-tracker-client/spec.md` 的功能规格

**Note**: This template is filled in by the `/speckit.plan` command.
**说明**：本模板由 `/speckit.plan` 命令生成。

## Summary
## 概要

Build a Tauri desktop client that lets users search 6-digit fund codes, view
current fund info (net value, change, update time), auto-refresh list data on a
configurable interval, organize funds into multiple lists, and persist data
locally across restarts.

构建一个 Tauri 桌面客户端，支持 6 位基金代码搜索、展示净值/涨跌/更新时间、
列表数据按可配置间隔自动刷新、多列表管理，并在重启后本地持久化数据。

## Technical Context
## 技术背景

**Language/Version**: Rust 1.70+, TypeScript (React 18)  
**Primary Dependencies**: Tauri 1.5, React 18, Vite, serde/serde_json, reqwest  
**Storage**: Local JSON file in Tauri app data directory  
**Testing**: `cargo test` (Rust), `npm test` (frontend, if configured)  
**Target Platform**: macOS (dev), Windows/Linux supported  
**Project Type**: Single desktop app (Tauri frontend + backend)  
**Performance Goals**: Fund search results within 3s (95%); 60fps UI target  
**Constraints**: Only fundgz data source; local-only data; async I/O; no telemetry  
**Scale/Scope**: Single-user desktop app; <=50 lists; <=200 funds/list

**语言/版本**：Rust 1.70+，TypeScript（React 18）  
**主要依赖**：Tauri 1.5、React 18、Vite、serde/serde_json、reqwest  
**存储**：Tauri 应用数据目录中的本地 JSON 文件  
**测试**：`cargo test`（Rust），`npm test`（前端，如配置）  
**目标平台**：macOS（开发），Windows/Linux 支持  
**项目类型**：单体桌面应用（Tauri 前后端）  
**性能目标**：基金查询 95% 低于 3s；UI 目标 60fps  
**约束**：仅 fundgz 数据源；本地存储；异步 I/O；无遥测  
**规模范围**：单用户；<=50 列表；<=200 基金/列表

## Constitution Check
## 宪法检查

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

*门禁：进入 Phase 0 研究前必须通过，Phase 1 设计后复检。*

- 数据源限定为 `fundgz.1234567.com.cn`，并标注数据时间戳
- 本地持久化与迁移策略已定义，不丢失历史数据
- macOS/Windows/Linux 行为一致，主线程不阻塞
- 仅访问批准的外部服务，无遥测/数据上传
- 核心模块测试计划明确，Tauri 命令契约已对齐

- Data source limited to `fundgz.1234567.com.cn` with visible timestamps
- Local persistence and migration guard defined
- Cross-platform consistency with non-blocking UI
- No unapproved external services or telemetry
- Core module tests and Tauri command contracts aligned

**Post-design re-check**: Passed (data model, contracts, and quickstart align).
**设计后复检**：通过（数据模型、契约与 quickstart 一致）。

## Project Structure
## 项目结构

### Documentation (this feature)
### 文档（本功能）

```text
specs/001-fund-tracker-client/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
└── tasks.md
```

### Source Code (repository root)
### 源码结构（仓库根目录）

```text
src/
├── components/
├── hooks/
├── types/
├── App.tsx
└── main.tsx

src-tauri/
├── src/
│   ├── modules/
│   │   ├── fund_api.rs
│   │   ├── storage.rs
│   │   └── list_manager.rs
│   ├── commands.rs
│   ├── models.rs
│   ├── errors.rs
│   └── main.rs
└── Cargo.toml
```

**Structure Decision**: Single Tauri desktop app with frontend in `src/` and
backend in `src-tauri/`.

**结构选择**：单体 Tauri 桌面应用，前端在 `src/`，后端在 `src-tauri/`。

## Complexity Tracking
## 复杂度跟踪

No constitution violations detected; no complexity justifications required.

未发现宪法违规；无需复杂度说明。
