# Feature Specification / 功能规格: SQLite Local Persistence

**Feature Branch**: `003-sqlite-storage`  
**功能分支**：`003-sqlite-storage`  
**Created**: 2026-01-21  
**创建日期**：2026-01-21  
**Status**: Draft  
**状态**：草稿  
**Input**: User description: "在 002 的基础上，将数据保存到数据库，先用 sqllite 数据库，由 rust 写入数据到本地 sqllite db文件。"  
**输入**：用户描述：“在 002 的基础上，将数据保存到数据库，先用 sqllite 数据库，由 rust 写入数据到本地 sqllite db文件。”

## User Scenarios & Testing *(mandatory)* / 用户场景与测试（必填）

### User Story 1 - Persist Lists in SQLite (Priority: P1) / 用户故事 1 - 列表持久化到 SQLite（优先级：P1）

As a user, I want my lists and fund selections to be stored in a local SQLite database so they are retained across app restarts.  
作为用户，我希望我的列表与基金选择保存到本地 SQLite 数据库，应用重启后仍能保留。

**Why this priority**: Persistence is the core requirement of this change.  
**优先级理由**：持久化是本次变更的核心目标。

**Independent Test**: Create a list and add funds, restart the app, and verify the data remains.  
**独立测试**：创建列表并添加基金，重启应用后确认数据仍存在。

**Acceptance Scenarios** / **验收场景**：

1. **Given** I created a list and added funds, **When** I restart the app, **Then** the list and its funds remain intact.  
   **假设** 已创建列表并添加基金，**当** 重启应用，**则** 列表与基金仍完整保留。
2. **Given** I rename or delete a list, **When** I restart the app, **Then** the changes persist.  
   **假设** 重命名或删除列表，**当** 重启应用，**则** 变更保持不变。

---

### User Story 2 - Data Migration from JSON (Priority: P2) / 用户故事 2 - 从 JSON 迁移数据（优先级：P2）

As a user with existing data, I want my local JSON data to migrate to SQLite automatically without losing anything.  
作为已有数据的用户，我希望本地 JSON 数据自动迁移到 SQLite，且不丢失。

**Why this priority**: Existing users must keep their data with minimal disruption.  
**优先级理由**：已有用户必须无感迁移且不丢数据。

**Independent Test**: Start the app with existing JSON data and confirm SQLite has identical lists and funds.  
**独立测试**：使用已有 JSON 数据启动应用，确认 SQLite 中数据一致。

**Acceptance Scenarios** / **验收场景**：

1. **Given** JSON data exists, **When** the app starts, **Then** data is migrated to SQLite and remains available.  
   **假设** 存在 JSON 数据，**当** 应用启动，**则** 数据迁移到 SQLite 且可用。
2. **Given** migration completes, **When** the app restarts, **Then** it reads from SQLite instead of JSON.  
   **假设** 迁移完成，**当** 应用重启，**则** 从 SQLite 读取而非 JSON。

---

### User Story 3 - Database Recovery Handling (Priority: P3) / 用户故事 3 - 数据库恢复处理（优先级：P3）

As a user, I want clear recovery guidance if the database file is missing or corrupted.  
作为用户，我希望在数据库文件丢失或损坏时能得到明确的恢复指引。

**Why this priority**: Data issues must be recoverable and clearly communicated.  
**优先级理由**：数据问题必须可恢复且清晰提示。

**Independent Test**: Corrupt or remove the database file and verify the app shows recovery guidance.  
**独立测试**：模拟数据库损坏或删除，验证应用给出恢复指引。

**Acceptance Scenarios** / **验收场景**：

1. **Given** the SQLite file is missing or unreadable, **When** the app starts, **Then** a recovery message is shown.  
   **假设** SQLite 文件缺失或不可读，**当** 应用启动，**则** 显示恢复提示。

### Edge Cases / 边界情况

- What happens when migration is interrupted midway? / 迁移中断时如何处理？
- How does the system handle partial data in SQLite? / SQLite 中数据部分写入如何处理？
- What happens if JSON and SQLite both exist but conflict? / JSON 与 SQLite 同时存在且冲突时如何处理？

## Requirements *(mandatory)* / 需求（必填）

### Functional Requirements / 功能性需求

- **FR-001**: The system MUST store lists and fund selections in a local SQLite database.  
  **FR-001**：系统必须将列表与基金选择保存到本地 SQLite 数据库。
- **FR-002**: The system MUST load list data from SQLite on startup.  
  **FR-002**：系统启动时必须从 SQLite 加载列表数据。
- **FR-003**: The system MUST migrate existing JSON data into SQLite on first run after upgrade.  
  **FR-003**：系统升级后首次启动必须将现有 JSON 数据迁移到 SQLite。
- **FR-004**: The system MUST avoid data loss during migration and report failures.  
  **FR-004**：迁移过程中必须避免数据丢失并在失败时报告。
- **FR-005**: The system MUST provide user-visible recovery guidance when the SQLite database is missing or corrupted.  
  **FR-005**：SQLite 数据库缺失或损坏时必须给出用户可见的恢复指引。
- **FR-006**: The system MUST keep data storage local and not upload to external services.  
  **FR-006**：系统必须保持数据仅存本地，不上传外部服务。

### Constitution Constraints *(mandatory)* / 宪法约束（必填）

- Data source, privacy, cross-platform consistency, and testing requirements MUST align with `.specify/memory/constitution.md`.  
  数据源、隐私、跨平台一致性、测试要求等 MUST 与 `.specify/memory/constitution.md` 保持一致。
- If any constraint cannot be met, an alternative with explicit rationale MUST be provided.  
  若某条约束无法满足，必须给出替代方案与明确理由。

### Key Entities *(include if feature involves data)* / 关键实体（如涉及数据）

- **Fund List**: User-defined list with name and ordered fund codes.  
  **基金列表**：包含名称与有序基金代码的用户列表。
- **Fund Entry**: Fund code associated with a list.  
  **基金条目**：与列表关联的基金代码。
- **Migration Record**: State indicating whether JSON-to-SQLite migration has completed.  
  **迁移记录**：表示 JSON 向 SQLite 迁移是否完成的状态。

### Assumptions / 假设

- SQLite database is stored in the same app data directory used previously for JSON.  
  SQLite 数据库与原 JSON 使用同一应用数据目录。
- Data volumes remain within local SQLite performance expectations.  
  数据规模在本地 SQLite 可接受范围内。

## Success Criteria *(mandatory)* / 成功标准（必填）

### Measurable Outcomes / 可衡量结果

- **SC-001**: 100% of existing users retain their lists and fund selections after upgrade.  
  **SC-001**：升级后 100% 的已有用户保留列表与基金选择。
- **SC-002**: App startup completes within 2 seconds for typical data sizes after migration.  
  **SC-002**：迁移后典型数据量下应用启动在 2 秒内完成。
- **SC-003**: Migration completes successfully on first run in at least 99% of cases.  
  **SC-003**：首次运行迁移成功率至少 99%。
- **SC-004**: Recovery guidance appears within 2 seconds when the database is missing or corrupted.  
  **SC-004**：数据库缺失或损坏时 2 秒内显示恢复指引。
