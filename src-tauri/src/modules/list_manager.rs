use crate::errors::{AppError, AppResult};
use crate::models::{AppState, FundList};
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use std::sync::Mutex;
use std::str::FromStr;

async fn get_pool(state: &Mutex<AppState>) -> SqlitePool {
    let pool = state.lock().unwrap().pool.clone();
    pool
}

async fn list_count(pool: &SqlitePool) -> AppResult<i64> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM groups")
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
    Ok(count)
}

async fn list_name_exists(pool: &SqlitePool, name: &str, exclude_id: Option<i64>) -> AppResult<bool> {
    let row = if let Some(exclude_id) = exclude_id {
        sqlx::query("SELECT id FROM groups WHERE name = ? AND id != ? LIMIT 1")
            .bind(name)
            .bind(exclude_id)
            .fetch_optional(pool)
            .await
    } else {
        sqlx::query("SELECT id FROM groups WHERE name = ? LIMIT 1")
            .bind(name)
            .fetch_optional(pool)
            .await
    }
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    Ok(row.is_some())
}

async fn list_exists(pool: &SqlitePool, list_id: i64) -> AppResult<bool> {
    let row = sqlx::query("SELECT id FROM groups WHERE id = ? LIMIT 1")
        .bind(list_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
    Ok(row.is_some())
}

async fn max_list_position(pool: &SqlitePool) -> AppResult<i64> {
    let max_pos: Option<i64> = sqlx::query_scalar("SELECT MAX(position) FROM groups")
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
    Ok(max_pos.unwrap_or(-1))
}

async fn list_fund_count(pool: &SqlitePool, list_id: i64) -> AppResult<i64> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM group_funds WHERE group_id = ?")
        .bind(list_id)
        .fetch_one(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
    Ok(count)
}

async fn max_fund_position(pool: &SqlitePool, list_id: i64) -> AppResult<i64> {
    let max_pos: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(position) FROM group_funds WHERE group_id = ?",
    )
    .bind(list_id)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
    Ok(max_pos.unwrap_or(-1))
}

/// 创建新列表
pub async fn create_list(state: &Mutex<AppState>, name: String) -> AppResult<FundList> {
    // 验证列表名称
    if !FundList::validate_name(&name) {
        return Err(AppError::ValidationError(
            "列表名称不能为空且不能超过64个字符".to_string(),
        ));
    }

    let pool = get_pool(state).await;

    // 检查名称唯一性
    if list_name_exists(&pool, &name, None).await? {
        return Err(AppError::DuplicateListName(name));
    }

    // 检查列表数量限制
    if list_count(&pool).await? >= 50 {
        return Err(AppError::ValidationError(
            "已达到最大列表数量限制(50个)".to_string(),
        ));
    }

    let now = Utc::now().timestamp();
    let position = max_list_position(&pool).await? + 1;

    let result = sqlx::query(
        "INSERT INTO groups (name, position, created_at, updated_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&name)
    .bind(position)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    let id = result.last_insert_rowid();

    Ok(FundList {
        id,
        name,
        fund_codes: Vec::new(),
        created_at: now,
        updated_at: now,
        position,
    })
}

/// 重命名列表
pub async fn rename_list(state: &Mutex<AppState>, id: i64, new_name: String) -> AppResult<()> {
    // 验证新名称
    if !FundList::validate_name(&new_name) {
        return Err(AppError::ValidationError(
            "列表名称不能为空且不能超过64个字符".to_string(),
        ));
    }

    let pool = get_pool(state).await;

    if !list_exists(&pool, id).await? {
        return Err(AppError::ListNotFound(id.to_string()));
    }

    // 检查新名称是否与其他列表冲突
    if list_name_exists(&pool, &new_name, Some(id)).await? {
        return Err(AppError::DuplicateListName(new_name));
    }

    let now = Utc::now().timestamp();
    sqlx::query("UPDATE groups SET name = ?, updated_at = ? WHERE id = ?")
        .bind(&new_name)
        .bind(now)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

/// 删除列表
pub async fn delete_list(state: &Mutex<AppState>, id: i64) -> AppResult<()> {
    let pool = get_pool(state).await;

    if !list_exists(&pool, id).await? {
        return Err(AppError::ListNotFound(id.to_string()));
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    sqlx::query("DELETE FROM group_funds WHERE group_id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    sqlx::query("DELETE FROM groups WHERE id = ?")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    tx.commit()
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

/// 添加基金到列表
pub async fn add_fund_to_list(
    state: &Mutex<AppState>,
    list_id: i64,
    fund_code: String,
) -> AppResult<()> {
    // 验证基金代码
    if !crate::models::FundInfo::validate_code(&fund_code) {
        return Err(AppError::ValidationError("无效的基金代码格式".to_string()));
    }

    let pool = get_pool(state).await;

    if !list_exists(&pool, list_id).await? {
        return Err(AppError::ListNotFound(list_id.to_string()));
    }

    // 检查是否已存在（去重）
    let exists = sqlx::query("SELECT id FROM group_funds WHERE group_id = ? AND fund_code = ?")
        .bind(list_id)
        .bind(&fund_code)
        .fetch_optional(&pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
    if exists.is_some() {
        return Err(AppError::DuplicateFund(fund_code));
    }

    // 检查列表容量
    if list_fund_count(&pool, list_id).await? >= 200 {
        return Err(AppError::ValidationError(
            "列表已达到最大基金数量(200个)".to_string(),
        ));
    }

    let now = Utc::now().timestamp();
    let position = max_fund_position(&pool, list_id).await? + 1;

    // upsert fund master
    sqlx::query(
        "INSERT INTO funds (code, created_at, updated_at) VALUES (?, ?, ?) \n         ON CONFLICT(code) DO UPDATE SET updated_at = excluded.updated_at",
    )
    .bind(&fund_code)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    sqlx::query(
        "INSERT INTO group_funds (group_id, fund_code, position, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(list_id)
    .bind(&fund_code)
    .bind(position)
    .bind(now)
    .bind(now)
    .execute(&pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    sqlx::query("UPDATE groups SET updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(list_id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

/// 从列表中移除基金
pub async fn remove_fund_from_list(
    state: &Mutex<AppState>,
    list_id: i64,
    fund_code: String,
) -> AppResult<()> {
    let pool = get_pool(state).await;

    if !list_exists(&pool, list_id).await? {
        return Err(AppError::ListNotFound(list_id.to_string()));
    }

    let result = sqlx::query("DELETE FROM group_funds WHERE group_id = ? AND fund_code = ?")
        .bind(list_id)
        .bind(&fund_code)
        .execute(&pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(AppError::ValidationError("基金不在此列表中".to_string()));
    }

    let now = Utc::now().timestamp();
    sqlx::query("UPDATE groups SET updated_at = ? WHERE id = ?")
        .bind(now)
        .bind(list_id)
        .execute(&pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

/// 重新排序列表
pub async fn reorder_lists(state: &Mutex<AppState>, list_ids: Vec<i64>) -> AppResult<()> {
    let pool = get_pool(state).await;

    if list_ids.len() as i64 != list_count(&pool).await? {
        return Err(AppError::ValidationError("列表ID不完整".to_string()));
    }

    for id in &list_ids {
        if !list_exists(&pool, *id).await? {
            return Err(AppError::ValidationError(format!("无效的列表ID: {}", id)));
        }
    }

    let now = Utc::now().timestamp();
    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    for (i, id) in list_ids.iter().enumerate() {
        sqlx::query("UPDATE groups SET position = ?, updated_at = ? WHERE id = ?")
            .bind(i as i64)
            .bind(now)
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
    }

    tx.commit()
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

/// 获取所有列表（已按 position 排序）
pub async fn get_all_lists(state: &Mutex<AppState>) -> AppResult<Vec<FundList>> {
    let pool = get_pool(state).await;

    let rows = sqlx::query(
        "SELECT id, name, position, created_at, updated_at FROM groups ORDER BY position ASC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    let mut lists = Vec::new();
    for row in rows {
        let id: i64 = row
            .try_get("id")
            .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
        let name: String = row
            .try_get("name")
            .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
        let position: i64 = row
            .try_get("position")
            .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
        let created_at: i64 = row
            .try_get("created_at")
            .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
        let updated_at: i64 = row
            .try_get("updated_at")
            .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

        let fund_codes = get_list_fund_codes_internal(&pool, id).await?;

        lists.push(FundList {
            id,
            name,
            fund_codes,
            created_at,
            updated_at,
            position,
        });
    }

    Ok(lists)
}

pub fn compute_daily_change_amount(
    change_percent: &Option<String>,
    holding_amount: Option<f64>,
) -> Option<f64> {
    let holding_amount = holding_amount?;
    let raw = change_percent.as_ref()?;
    let cleaned = raw.trim().trim_end_matches('%');
    let percent = f64::from_str(cleaned).ok()?;
    Some(holding_amount * percent / 100.0)
}

async fn get_list_fund_codes_internal(pool: &SqlitePool, list_id: i64) -> AppResult<Vec<String>> {
    let rows = sqlx::query(
        "SELECT fund_code FROM group_funds WHERE group_id = ? ORDER BY position ASC",
    )
    .bind(list_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    let mut codes = Vec::new();
    for row in rows {
        let code: String = row
            .try_get("fund_code")
            .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
        codes.push(code);
    }

    Ok(codes)
}

/// 获取列表中的基金代码
pub async fn get_list_fund_codes(
    state: &Mutex<AppState>,
    list_id: i64,
) -> AppResult<Vec<String>> {
    let pool = get_pool(state).await;
    if !list_exists(&pool, list_id).await? {
        return Err(AppError::ListNotFound(list_id.to_string()));
    }
    get_list_fund_codes_internal(&pool, list_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use std::path::PathBuf;

    async fn create_test_state() -> Mutex<AppState> {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE groups (id INTEGER PRIMARY KEY AUTOINCREMENT, name VARCHAR(64) NOT NULL, position INTEGER NOT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL);",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE group_funds (id INTEGER PRIMARY KEY AUTOINCREMENT, group_id INTEGER NOT NULL, fund_code VARCHAR(64) NOT NULL, position INTEGER NOT NULL, created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL, UNIQUE(group_id, fund_code));",
        )
        .execute(&pool)
        .await
        .unwrap();
        sqlx::query(
            "CREATE TABLE funds (id INTEGER PRIMARY KEY AUTOINCREMENT, code VARCHAR(64) NOT NULL UNIQUE, name VARCHAR(64), created_at INTEGER NOT NULL, updated_at INTEGER NOT NULL);",
        )
        .execute(&pool)
        .await
        .unwrap();

        Mutex::new(AppState::new(
            pool,
            PathBuf::from("/tmp/test.sqlite"),
            PathBuf::from("/tmp/test.json"),
            None,
        ))
    }

    #[tokio::test]
    async fn test_create_list() {
        let state = create_test_state().await;
        let list = create_list(&state, "测试列表".to_string()).await.unwrap();

        assert_eq!(list.name, "测试列表");
        assert_eq!(list.fund_codes.len(), 0);
        assert_eq!(list.position, 0);

        let lists = get_all_lists(&state).await.unwrap();
        assert_eq!(lists.len(), 1);
    }
}
