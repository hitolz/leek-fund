use crate::db::session_chat_message;
use crate::errors::AppResult;
use crate::models::ChatMessage;
use sqlx::SqlitePool;

pub async fn save_user_message(
    pool: &SqlitePool,
    session_id: &str,
    content: &str,
) -> AppResult<ChatMessage> {
    session_chat_message::insert_message(pool, session_id, "user", content, "saved").await
}

pub async fn save_assistant_message(
    pool: &SqlitePool,
    session_id: &str,
    content: &str,
    saved_state: &str,
) -> AppResult<ChatMessage> {
    session_chat_message::insert_message(pool, session_id, "assistant", content, saved_state).await
}

pub async fn list_messages(
    pool: &SqlitePool,
    session_id: &str,
    limit: Option<i64>,
) -> AppResult<Vec<ChatMessage>> {
    session_chat_message::list_messages(pool, session_id, limit).await
}
