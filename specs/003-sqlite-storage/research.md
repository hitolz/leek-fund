# Research / 调研

## Decision 1: SQLite Driver Choice / 决策 1：SQLite 驱动选择

**Decision**: Use `sqlx` with SQLite for local database access in the Rust backend.  
**决策**：后端使用 `sqlx` 操作 SQLite。

**Rationale**: Async-friendly integration with the existing runtime and strong type mapping.  
**理由**：与现有异步运行时更契合，并提供良好的类型映射能力。

**Alternatives considered**: `rusqlite` (sync), custom file storage.  
**备选方案**：`rusqlite`（同步）、自定义文件存储。

## Decision 2: Migration Strategy / 决策 2：迁移策略

**Decision**: On first run after upgrade, read JSON data and write to SQLite in a single migration step, then mark migration as completed.  
**决策**：升级后首次启动读取 JSON 并一次性写入 SQLite，然后标记迁移完成。

**Rationale**: Minimizes user disruption while ensuring no data loss.  
**理由**：最小化用户感知，并保证不丢数据。

**Alternatives considered**: Manual migration tool, multi-step background migration.  
**备选方案**：手动迁移工具、多阶段后台迁移。

## Decision 3: Corruption Recovery / 决策 3：损坏恢复

**Decision**: If the SQLite file is missing or corrupted, preserve the file (rename with timestamp) and start with a fresh database, showing recovery guidance.  
**决策**：SQLite 文件缺失或损坏时保留原文件（带时间戳改名），创建新库并提示恢复。

**Rationale**: Avoids data loss while keeping the app usable.  
**理由**：避免数据丢失，并保证应用可用。

**Alternatives considered**: Hard fail without recovery; auto-delete corrupted file.  
**备选方案**：直接失败不恢复；自动删除损坏文件。

## Decision 4: Storage Location / 决策 4：存储位置

**Decision**: Store the SQLite file in the Tauri app data directory alongside existing data files.  
**决策**：SQLite 文件存放在 Tauri 应用数据目录，与现有数据文件同目录。

**Rationale**: Aligns with platform conventions and existing storage path logic.  
**理由**：符合平台标准路径，与现有存储路径逻辑一致。

**Alternatives considered**: User-selected folder; project root storage.  
**备选方案**：用户自选路径；项目根目录存储。
