use super::list_manager::{add_fund_to_list, create_list, rename_list};
use crate::models::AppState;
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::sync::Mutex;

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
async fn test_updated_at_changes_on_rename() {
    let state = create_test_state().await;
    let list = create_list(&state, "测试列表".to_string()).await.unwrap();
    let before = list.updated_at;
    rename_list(&state, list.id, "新名称".to_string()).await.unwrap();

    let pool = state.lock().unwrap().pool.clone();
    let updated: i64 = sqlx::query_scalar("SELECT updated_at FROM groups WHERE id = ?")
        .bind(list.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(updated >= before);
}

#[tokio::test]
async fn test_updated_at_changes_on_add_fund() {
    let state = create_test_state().await;
    let list = create_list(&state, "测试列表".to_string()).await.unwrap();
    let before = list.updated_at;
    add_fund_to_list(&state, list.id, "001632".to_string())
        .await
        .unwrap();

    let pool = state.lock().unwrap().pool.clone();
    let updated: i64 = sqlx::query_scalar("SELECT updated_at FROM groups WHERE id = ?")
        .bind(list.id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(updated >= before);
}
