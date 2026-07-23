use crate::errors::{AppError, AppResult};
use crate::models::ChatMessage;
use chrono::Utc;
use sqlx::{Row, SqlitePool};

pub async fn insert_message(
    pool: &SqlitePool,
    session_id: &str,
    role: &str,
    content: &str,
    saved_state: &str,
    snapshot_id: Option<&str>,
    context_json: Option<&str>,
) -> AppResult<ChatMessage> {
    let now = Utc::now().timestamp();
    let result = sqlx::query(
        "INSERT INTO session_chat_messages \
         (session_id, role, content, saved_state, snapshot_id, context_json, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(session_id)
    .bind(role)
    .bind(content)
    .bind(saved_state)
    .bind(snapshot_id)
    .bind(context_json)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(ChatMessage {
        id: result.last_insert_rowid(),
        session_id: session_id.to_string(),
        role: role.to_string(),
        content: content.to_string(),
        saved_state: Some(saved_state.to_string()),
        created_at: now,
        updated_at: now,
    })
}

pub async fn list_messages(
    pool: &SqlitePool,
    session_id: &str,
    limit: Option<i64>,
) -> AppResult<Vec<ChatMessage>> {
    let sql = if limit.is_some() {
        "SELECT id, session_id, role, content, saved_state, created_at, updated_at \
         FROM session_chat_messages WHERE session_id = ? ORDER BY id ASC LIMIT ?"
    } else {
        "SELECT id, session_id, role, content, saved_state, created_at, updated_at \
         FROM session_chat_messages WHERE session_id = ? ORDER BY id ASC"
    };

    let mut query = sqlx::query(sql).bind(session_id);
    if let Some(l) = limit {
        query = query.bind(l);
    }

    let rows = query
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    let messages = rows
        .into_iter()
        .map(|row| ChatMessage {
            id: row.get::<i64, _>("id"),
            session_id: row.get::<String, _>("session_id"),
            role: row.get::<String, _>("role"),
            content: row.get::<String, _>("content"),
            saved_state: row.get::<Option<String>, _>("saved_state"),
            created_at: row.get::<i64, _>("created_at"),
            updated_at: row.get::<i64, _>("updated_at"),
        })
        .collect();

    Ok(messages)
}
