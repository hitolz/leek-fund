# Implementation Plan: 修复当日涨跌总金额计算中的正负号处理

**Branch**: `005-fix-amount-calculation` | **Date**: 2026-01-22 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/005-fix-amount-calculation/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

修复基金列表中当日涨跌总金额计算和显示的正负号处理问题，确保上涨基金（正值）和下跌基金（负值）正确参与总额计算，并优化显示格式的用户体验。这是一个前端计算逻辑和UI显示的修复任务，主要涉及React组件中的数值解析、计算和格式化逻辑。

## Technical Context

**Language/Version**: TypeScript 5.3+ (Frontend), Rust 2021 Edition (Backend)
**Primary Dependencies**: React 18.2+, Tauri 1.5+, Vite 5.0+
**Storage**: SQLite (via sqlx 0.7) for fund data persistence
**Testing**: NEEDS CLARIFICATION - 需要确定前端测试框架
**Target Platform**: Desktop application (Tauri cross-platform)
**Project Type**: Desktop application with Tauri (Rust backend + React frontend)
**Performance Goals**: Real-time calculation update (<100ms), UI响应时间 <1秒
**Constraints**: 精确的金额计算，浮点数精度处理，用户界面响应性
**Scale/Scope**: 个人用户级别应用，支持数十到数百只基金的计算

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Constitution file is template-only - no specific gates to check. Proceeding with standard software quality practices.

## Project Structure

### Documentation (this feature)

```text
specs/005-fix-amount-calculation/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
# Tauri Desktop Application Structure
src-tauri/
├── src/
│   ├── commands.rs      # Tauri命令接口
│   ├── models.rs        # 数据模型定义
│   └── modules/
│       ├── fund_api.rs  # 基金数据API
│       ├── storage.rs   # 数据存储
│       └── list_manager.rs # 列表管理

src/
├── components/
│   ├── ListDetailView.tsx  # 主要修改目标 - 涨跌总额计算
│   ├── FundDetailPanel.tsx
│   └── ListDetail.tsx
├── hooks/
│   ├── useTauriApi.ts
│   └── useTauriCommands.ts
├── types/
│   └── fund.ts         # TypeScript类型定义
└── App.tsx

tests/
└── [需要创建测试结构]
```

**Structure Decision**: 这是一个Tauri桌面应用程序，前端使用React+TypeScript，后端使用Rust。主要修改集中在前端的`ListDetailView.tsx`组件中的`dailyChangeTotal`计算逻辑和`formatSignedAmount`显示函数。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

无violations需要记录。