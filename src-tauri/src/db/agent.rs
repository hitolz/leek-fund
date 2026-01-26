use crate::errors::{AppError, AppResult};
use crate::models::ChatAgentInfo;
use chrono::Utc;
use sqlx::{Row, SqlitePool};

pub async fn ensure_default_agent(pool: &SqlitePool, name: &str, description: &str) -> AppResult<ChatAgentInfo> {
    if let Some(agent) = get_agent_by_name(pool, name).await? {
        return Ok(agent);
    }

    let now = Utc::now().timestamp();
    let result = sqlx::query(
        "INSERT INTO agents (name, description, created_at, updated_at) VALUES (?, ?, ?, ?)",
    )
    .bind(name)
    .bind(description)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(ChatAgentInfo {
        id: result.last_insert_rowid(),
        name: name.to_string(),
        description: Some(description.to_string()),
        created_at: now,
        updated_at: now,
    })
}

pub async fn get_agent_by_name(pool: &SqlitePool, name: &str) -> AppResult<Option<ChatAgentInfo>> {
    let row = sqlx::query(
        "SELECT id, name, description, created_at, updated_at FROM agents WHERE name = ? LIMIT 1",
    )
    .bind(name)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    Ok(row.map(|row| ChatAgentInfo {
        id: row.get::<i64, _>("id"),
        name: row.get::<String, _>("name"),
        description: row.get::<Option<String>, _>("description"),
        created_at: row.get::<i64, _>("created_at"),
        updated_at: row.get::<i64, _>("updated_at"),
    }))
}
