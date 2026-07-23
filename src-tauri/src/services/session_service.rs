use crate::db::session;
use crate::errors::{AppError, AppResult};
use crate::models::ChatSession;
use sqlx::SqlitePool;

pub async fn get_or_create_recent_session(pool: &SqlitePool) -> AppResult<ChatSession> {
    if let Some(session) = session::get_recent_session(pool).await? {
        return Ok(session);
    }
    session::create_session(pool).await
}

pub async fn create_session(pool: &SqlitePool) -> AppResult<ChatSession> {
    session::create_session(pool).await
}

pub async fn touch_session(pool: &SqlitePool, session_id: &str) -> AppResult<()> {
    session::touch_session(pool, session_id).await
}

pub async fn get_recent_session(pool: &SqlitePool) -> AppResult<Option<ChatSession>> {
    session::get_recent_session(pool).await
}

pub async fn list_sessions(pool: &SqlitePool, limit: i64) -> AppResult<Vec<ChatSession>> {
    session::list_sessions(pool, limit).await
}

pub async fn ensure_session_exists(pool: &SqlitePool, session_id: &str) -> AppResult<()> {
    let row = sqlx::query("SELECT id FROM sessions WHERE session_id = ? LIMIT 1")
        .bind(session_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    if row.is_none() {
        return Err(AppError::ChatSessionNotFound(session_id.to_string()));
    }

    Ok(())
}
