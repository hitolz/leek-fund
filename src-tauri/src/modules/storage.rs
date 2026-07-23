use crate::errors::{AppError, AppResult};
use crate::migrations;
use crate::models::{FundList, UserData};
use crate::modules::asset_position;
use chrono::Utc;
use serde::Deserialize;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{Row, SqlitePool};
use std::fs;
use std::path::{Path, PathBuf};

const DB_FILE_NAME: &str = "lists.sqlite";
const LEGACY_JSON_NAME: &str = "lists.json";
const STORAGE_DIR_NAME: &str = ".leek";
const SQLITE_SIDECAR_FILES: &[&str] = &["lists.sqlite-wal", "lists.sqlite-shm"];

/// 初始化存储目录并返回 SQLite 连接池
pub async fn init_storage(
    app_handle: &tauri::AppHandle,
) -> AppResult<(SqlitePool, PathBuf, PathBuf, Option<String>)> {
    let legacy_app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or_else(|| AppError::StorageError("无法获取应用数据目录".to_string()))?;
    let storage_dir = storage_dir_from_home(tauri::api::path::home_dir())?;

    fs::create_dir_all(&storage_dir)?;
    migrate_legacy_storage_files(&legacy_app_data_dir, &storage_dir)?;

    let db_path = storage_dir.join(DB_FILE_NAME);
    let legacy_json_path = storage_dir.join(LEGACY_JSON_NAME);

    let (pool, warning) = open_or_recover_db(&db_path).await?;

    run_migrations(&pool).await?;
    ensure_group_fund_positions_schema(&pool).await?;
    ensure_ai_copilot_schema(&pool).await?;

    // 初始化股票和加密货币持仓表
    asset_position::init_stock_holdings_table(&pool).await?;
    asset_position::init_crypto_holdings_table(&pool).await?;

    if should_migrate(&pool, &legacy_json_path).await? {
        migrate_from_json(&pool, &legacy_json_path).await?;
    }

    Ok((pool, db_path, legacy_json_path, warning))
}

fn storage_dir_from_home(home_dir: Option<PathBuf>) -> AppResult<PathBuf> {
    home_dir
        .map(|path| path.join(STORAGE_DIR_NAME))
        .ok_or_else(|| AppError::StorageError("无法获取用户主目录".to_string()))
}

/// 首次使用新目录时复制旧数据，原文件保留用于回退。
fn migrate_legacy_storage_files(legacy_dir: &Path, storage_dir: &Path) -> AppResult<()> {
    if legacy_dir == storage_dir {
        return Ok(());
    }

    let source_db = legacy_dir.join(DB_FILE_NAME);
    let target_db = storage_dir.join(DB_FILE_NAME);
    if target_db.exists() {
        return Ok(());
    }

    let files: Vec<&str> = if source_db.exists() {
        std::iter::once(DB_FILE_NAME)
            .chain(SQLITE_SIDECAR_FILES.iter().copied())
            .chain(std::iter::once(LEGACY_JSON_NAME))
            .collect()
    } else {
        vec![LEGACY_JSON_NAME]
    };

    for file_name in files {
        let source = legacy_dir.join(file_name);
        let target = storage_dir.join(file_name);
        if !source.exists() || target.exists() {
            continue;
        }
        fs::copy(&source, &target).map_err(|error| {
            AppError::StorageError(format!(
                "迁移存储文件 {} 到 {} 失败: {}",
                source.display(),
                target.display(),
                error
            ))
        })?;
    }

    Ok(())
}

async fn open_or_recover_db(db_path: &Path) -> AppResult<(SqlitePool, Option<String>)> {
    match open_pool(db_path).await {
        Ok(pool) => Ok((pool, None)),
        Err(_) => {
            let warning = if db_path.exists() {
                let backup = backup_corrupted_db(db_path)?;
                Some(format!(
                    "数据库文件损坏，已备份到 {}。已创建新的数据库。",
                    backup.display()
                ))
            } else {
                None
            };
            let pool = open_pool(db_path).await?;
            Ok((pool, warning))
        }
    }
}

async fn open_pool(db_path: &Path) -> AppResult<SqlitePool> {
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库连接失败: {}", e)))
}

async fn run_migrations(pool: &SqlitePool) -> AppResult<()> {
    let migration_sqls = migrations::load_migration_sqls()?;

    for sql in migration_sqls {
        for statement in sql.split(';') {
            let stmt = statement.trim();
            if stmt.is_empty() {
                continue;
            }
            sqlx::query(stmt)
                .execute(pool)
                .await
                .map_err(|e| AppError::StorageError(format!("迁移执行失败: {}", e)))?;
        }
    }

    Ok(())
}

async fn ensure_group_fund_positions_schema(pool: &SqlitePool) -> AppResult<()> {
    let exists: Option<String> = sqlx::query_scalar(
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'group_fund_positions'",
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
    if exists.is_none() {
        return Ok(());
    }

    let columns: Vec<String> = sqlx::query("PRAGMA table_info('group_fund_positions')")
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>("name").ok())
        .collect();

    if !columns.iter().any(|name| name == "holding_amount") {
        sqlx::query("ALTER TABLE group_fund_positions ADD COLUMN holding_amount REAL")
            .execute(pool)
            .await
            .map_err(|e| AppError::StorageError(format!("迁移执行失败: {}", e)))?;
    }

    if !columns.iter().any(|name| name == "holding_shares") {
        sqlx::query("ALTER TABLE group_fund_positions ADD COLUMN holding_shares REAL")
            .execute(pool)
            .await
            .map_err(|e| AppError::StorageError(format!("迁移执行失败: {}", e)))?;
    }

    if columns.iter().any(|name| name == "shares") {
        sqlx::query(
            "UPDATE group_fund_positions \
             SET holding_shares = COALESCE(holding_shares, shares), \
                 holding_amount = COALESCE(holding_amount, shares * unit_price) \
             WHERE holding_shares IS NULL OR holding_amount IS NULL",
        )
        .execute(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
    }

    sqlx::query(
        "UPDATE group_fund_positions \
         SET holding_shares = COALESCE(holding_shares, 0), \
             holding_amount = COALESCE(holding_amount, 0) \
         WHERE holding_shares IS NULL OR holding_amount IS NULL",
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

async fn ensure_ai_copilot_schema(pool: &SqlitePool) -> AppResult<()> {
    let columns: Vec<String> = sqlx::query("PRAGMA table_info('session_chat_messages')")
        .fetch_all(pool)
        .await
        .map_err(|error| AppError::StorageError(format!("数据库读取失败: {error}")))?
        .into_iter()
        .filter_map(|row| row.try_get::<String, _>("name").ok())
        .collect();
    for (column, definition) in [("snapshot_id", "TEXT"), ("context_json", "TEXT")] {
        if !columns.iter().any(|name| name == column) {
            sqlx::query(&format!(
                "ALTER TABLE session_chat_messages ADD COLUMN {column} {definition}"
            ))
            .execute(pool)
            .await
            .map_err(|error| AppError::StorageError(format!("迁移执行失败: {error}")))?;
        }
    }
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_session_messages_snapshot \
         ON session_chat_messages(snapshot_id)",
    )
    .execute(pool)
    .await
    .map_err(|error| AppError::StorageError(format!("迁移执行失败: {error}")))?;
    Ok(())
}

async fn should_migrate(pool: &SqlitePool, legacy_json_path: &Path) -> AppResult<bool> {
    if !legacy_json_path.exists() {
        return Ok(false);
    }

    let count: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM groups")
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
    Ok(count == 0)
}

async fn migrate_from_json(pool: &SqlitePool, legacy_json_path: &Path) -> AppResult<()> {
    let data = load_legacy_json(legacy_json_path)?;
    validate_storage_format(&data)?;

    let now = Utc::now().timestamp();

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    for (index, list) in data.lists.iter().enumerate() {
        let position = index as i64;
        let result = sqlx::query(
            "INSERT INTO groups (name, position, created_at, updated_at) VALUES (?, ?, ?, ?)",
        )
        .bind(&list.name)
        .bind(position)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

        let group_id = result.last_insert_rowid();

        for (fund_index, fund_code) in list.fund_codes.iter().enumerate() {
            sqlx::query(
                "INSERT INTO funds (code, created_at, updated_at) VALUES (?, ?, ?) \n                 ON CONFLICT(code) DO UPDATE SET updated_at = excluded.updated_at",
            )
            .bind(fund_code)
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

            sqlx::query(
                "INSERT INTO group_funds (group_id, fund_code, position, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            )
            .bind(group_id)
            .bind(fund_code)
            .bind(fund_index as i64)
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
        }
    }

    tx.commit()
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    let backup_path = legacy_json_path.with_extension("migrated.json");
    let _ = fs::rename(legacy_json_path, &backup_path);

    Ok(())
}

/// 从旧 JSON 文件加载数据
pub fn load_legacy_json(path: &Path) -> AppResult<UserData> {
    if !path.exists() {
        return Ok(UserData::new());
    }

    let content = fs::read_to_string(path)
        .map_err(|e| AppError::StorageError(format!("读取文件失败: {}", e)))?;

    if let Ok(data) = serde_json::from_str::<UserData>(&content) {
        if data.schema_version != 1 {
            return Err(AppError::StorageError(format!(
                "不支持的数据格式版本: {}",
                data.schema_version
            )));
        }
        return Ok(data);
    }

    if let Ok(legacy) = serde_json::from_str::<LegacyUserData>(&content) {
        if legacy.schema_version != 1 {
            return Err(AppError::StorageError(format!(
                "不支持的数据格式版本: {}",
                legacy.schema_version
            )));
        }

        let lists = legacy
            .lists
            .into_iter()
            .enumerate()
            .map(|(index, list)| FundList {
                id: (index + 1) as i64,
                name: list.name,
                fund_codes: list.fund_codes,
                created_at: list.created_at,
                updated_at: list.updated_at,
                position: list.position,
            })
            .collect();

        let now = Utc::now().timestamp();
        return Ok(UserData {
            schema_version: legacy.schema_version,
            lists,
            created_at: now,
            last_modified: now,
            last_migrated_at: None,
            preferences: None,
        });
    }

    let backup_path = get_backup_path(path);
    let _ = fs::rename(path, &backup_path);
    eprintln!("数据文件损坏，已备份到: {}", backup_path.display());

    Err(AppError::StorageError(
        "数据文件损坏，已备份。使用新的空数据。".to_string(),
    ))
}

#[derive(Deserialize)]
struct LegacyUserData {
    #[serde(default)]
    schema_version: u32,
    #[serde(default)]
    lists: Vec<LegacyFundList>,
}

#[derive(Deserialize)]
struct LegacyFundList {
    #[serde(default)]
    name: String,
    #[serde(default)]
    fund_codes: Vec<String>,
    #[serde(default)]
    created_at: i64,
    #[serde(default)]
    updated_at: i64,
    #[serde(default)]
    position: i64,
    #[serde(default)]
    id: Option<String>,
}

/// 获取备份文件路径
fn get_backup_path(path: &Path) -> PathBuf {
    let timestamp = chrono::Utc::now().timestamp();
    path.with_extension(format!("backup.{}.json", timestamp))
}

fn backup_corrupted_db(path: &Path) -> AppResult<PathBuf> {
    let timestamp = chrono::Utc::now().timestamp();
    let backup_path = path.with_extension(format!("backup.{}.sqlite", timestamp));
    fs::rename(path, &backup_path)
        .map_err(|e| AppError::StorageError(format!("备份数据库失败: {}", e)))?;
    Ok(backup_path)
}

/// 验证存储格式
pub fn validate_storage_format(data: &UserData) -> AppResult<()> {
    if data.schema_version != 1 {
        return Err(AppError::ValidationError(format!(
            "不支持的版本: {}",
            data.schema_version
        )));
    }

    if data.lists.len() > 50 {
        return Err(AppError::ValidationError(
            "列表数量超过限制(50个)".to_string(),
        ));
    }

    let mut names = std::collections::HashSet::new();
    for list in &data.lists {
        if !names.insert(&list.name) {
            return Err(AppError::ValidationError(format!(
                "列表名称重复: {}",
                list.name
            )));
        }
    }

    for list in &data.lists {
        if list.fund_codes.len() > 200 {
            return Err(AppError::ValidationError(format!(
                "列表 '{}' 的基金数量超过限制(200个)",
                list.name
            )));
        }

        let mut codes = std::collections::HashSet::new();
        for code in &list.fund_codes {
            if !codes.insert(code) {
                return Err(AppError::ValidationError(format!(
                    "列表 '{}' 中基金代码重复: {}",
                    list.name, code
                )));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn storage_directory_is_hidden_folder_under_home() {
        let home = PathBuf::from("/users/tester");
        assert_eq!(
            storage_dir_from_home(Some(home.clone())).unwrap(),
            home.join(".leek")
        );
        assert!(storage_dir_from_home(None).is_err());
    }

    #[test]
    fn legacy_storage_files_are_copied_without_overwriting_new_data() {
        let temp_dir = TempDir::new().unwrap();
        let legacy_dir = temp_dir.path().join("legacy");
        let storage_dir = temp_dir.path().join("home").join(".leek");
        fs::create_dir_all(&legacy_dir).unwrap();
        fs::create_dir_all(&storage_dir).unwrap();

        fs::write(legacy_dir.join(DB_FILE_NAME), b"old database").unwrap();
        fs::write(legacy_dir.join("lists.sqlite-wal"), b"old wal").unwrap();
        fs::write(legacy_dir.join("lists.sqlite-shm"), b"old shm").unwrap();
        fs::write(legacy_dir.join(LEGACY_JSON_NAME), b"old json").unwrap();

        migrate_legacy_storage_files(&legacy_dir, &storage_dir).unwrap();

        assert_eq!(
            fs::read(storage_dir.join(DB_FILE_NAME)).unwrap(),
            b"old database"
        );
        assert_eq!(
            fs::read(storage_dir.join("lists.sqlite-wal")).unwrap(),
            b"old wal"
        );
        assert_eq!(
            fs::read(storage_dir.join("lists.sqlite-shm")).unwrap(),
            b"old shm"
        );
        assert_eq!(
            fs::read(storage_dir.join(LEGACY_JSON_NAME)).unwrap(),
            b"old json"
        );

        fs::write(storage_dir.join(DB_FILE_NAME), b"new database").unwrap();
        fs::remove_file(storage_dir.join("lists.sqlite-wal")).unwrap();
        migrate_legacy_storage_files(&legacy_dir, &storage_dir).unwrap();
        assert_eq!(
            fs::read(storage_dir.join(DB_FILE_NAME)).unwrap(),
            b"new database"
        );
        assert!(!storage_dir.join("lists.sqlite-wal").exists());
    }

    #[test]
    fn test_load_legacy_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("nonexistent.json");

        let data = load_legacy_json(&path).unwrap();
        assert_eq!(data.lists.len(), 0);
    }

    #[test]
    fn test_corrupted_file_backup() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("corrupted.json");

        fs::write(&path, "invalid json content").unwrap();

        let result = load_legacy_json(&path);
        assert!(result.is_err());

        assert!(!path.exists());

        let backup_files: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .unwrap()
                    .starts_with("corrupted.backup")
            })
            .collect();
        assert!(!backup_files.is_empty());
    }

    #[tokio::test]
    async fn ai_schema_upgrade_adds_context_columns_to_legacy_table() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE session_chat_messages (\
             id INTEGER PRIMARY KEY, session_id TEXT NOT NULL, role TEXT NOT NULL, \
             content TEXT NOT NULL, saved_state TEXT, created_at INTEGER NOT NULL, \
             updated_at INTEGER NOT NULL)",
        )
        .execute(&pool)
        .await
        .unwrap();
        ensure_ai_copilot_schema(&pool).await.unwrap();
        ensure_ai_copilot_schema(&pool).await.unwrap();
        let columns: Vec<String> = sqlx::query("PRAGMA table_info('session_chat_messages')")
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.get("name"))
            .collect();
        assert!(columns.iter().any(|name| name == "snapshot_id"));
        assert!(columns.iter().any(|name| name == "context_json"));
    }

    #[tokio::test]
    async fn existing_database_runs_ai_migration_before_context_index() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        ensure_ai_copilot_schema(&pool).await.unwrap();

        let columns: Vec<String> = sqlx::query("PRAGMA table_info('session_chat_messages')")
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.get("name"))
            .collect();
        assert!(columns.iter().any(|name| name == "snapshot_id"));
        assert!(columns.iter().any(|name| name == "context_json"));

        let index_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sqlite_master \
             WHERE type = 'index' AND name = 'idx_session_messages_snapshot'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(index_count, 1);
    }
}
