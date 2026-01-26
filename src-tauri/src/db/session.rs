use crate::errors::{AppError, AppResult};
use crate::models::ChatSession;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub async fn create_session(pool: &SqlitePool) -> AppResult<ChatSession> {
    let session_id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp();

    let result = sqlx::query(
        "INSERT INTO sessions (session_id, title, created_at, updated_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&session_id)
    .bind(Option::<String>::None)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(ChatSession {
        id: result.last_insert_rowid(),
        session_id,
        title: None,
        created_at: now,
        updated_at: now,
    })
}

pub async fn get_recent_session(pool: &SqlitePool) -> AppResult<Option<ChatSession>> {
    let row = sqlx::query(
        "SELECT id, session_id, title, created_at, updated_at FROM sessions ORDER BY updated_at DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    Ok(row.map(|row| ChatSession {
        id: row.get::<i64, _>("id"),
        session_id: row.get::<String, _>("session_id"),
        title: row.get::<Option<String>, _>("title"),
        created_at: row.get::<i64, _>("created_at"),
        updated_at: row.get::<i64, _>("updated_at"),
    }))
}

pub async fn touch_session(pool: &SqlitePool, session_id: &str) -> AppResult<()> {
    let now = Utc::now().timestamp();
    sqlx::query("UPDATE sessions SET updated_at = ? WHERE session_id = ?")
        .bind(now)
        .bind(session_id)
        .execute(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
    Ok(())
}
