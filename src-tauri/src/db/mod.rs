use crate::errors::{AppError, AppResult};
use crate::models::AppState;
use sqlx::SqlitePool;
use std::sync::Mutex;

pub mod agent;
pub mod session;
pub mod session_chat_message;

pub async fn get_pool(state: &Mutex<AppState>) -> AppResult<SqlitePool> {
    let pool = state.lock().map_err(|_| {
        AppError::StorageError("无法获取数据库连接".to_string())
    })?;
    Ok(pool.pool.clone())
}
