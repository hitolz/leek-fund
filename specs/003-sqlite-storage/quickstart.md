# Quickstart / 快速开始

## Goal / 目标

Verify SQLite persistence, JSON migration, and recovery messaging.  
验证 SQLite 持久化、JSON 迁移与恢复提示。

## Prerequisites / 前置条件

- Existing JSON data available (for migration test).  
- 有现存 JSON 数据（用于迁移测试）。

## Steps / 步骤

1. Start the app with existing JSON data.  
   使用现有 JSON 数据启动应用。

2. Create a new list and add a fund.  
   新建列表并添加基金。

3. Restart the app.  
   重启应用。

4. Verify the list and fund remain intact.  
   验证列表与基金仍完整保留。

5. Check that a SQLite database file (e.g., `lists.sqlite`) exists in the app data directory.  
   检查应用数据目录中存在 SQLite 数据库文件（如 `lists.sqlite`）。

6. Rename or delete a list, restart the app, and verify changes persist.  
   重命名或删除列表后重启应用，验证变更仍保留。

7. Corrupt or rename the SQLite file, then restart the app.  
   损坏或重命名 SQLite 文件后重启应用。

8. Verify a recovery message appears and a new SQLite file is created.  
   验证恢复提示出现且创建新的 SQLite 文件。

## Expected Results / 预期结果

- Data is migrated from JSON into SQLite on first run after upgrade.  
- 升级后首次运行完成 JSON 到 SQLite 的迁移。

- Lists and fund selections persist across restarts.  
- 列表与基金选择在重启后仍保留。

- Recovery guidance appears when the SQLite file is missing or corrupted.  
- SQLite 文件缺失或损坏时显示恢复指引。
