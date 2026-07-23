use crate::errors::AppResult;

pub fn load_migration_sqls() -> AppResult<Vec<String>> {
    Ok(vec![
        include_str!("001_init.sql").to_string(),
        include_str!("002_group_fund_positions.sql").to_string(),
        include_str!("003_ai_copilot.sql").to_string(),
        include_str!("004_operations.sql").to_string(),
        include_str!("005_daily_history.sql").to_string(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[tokio::test]
    async fn migration_003_is_idempotent_and_creates_p0_tables() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        for _ in 0..2 {
            for sql in load_migration_sqls().unwrap() {
                for statement in sql
                    .split(';')
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                {
                    sqlx::query(statement).execute(&pool).await.unwrap();
                }
            }
        }
        for table in [
            "portfolio_snapshots",
            "ai_reports",
            "ai_findings",
            "sessions",
            "session_chat_messages",
            "agents",
            "ai_request_audits",
        ] {
            let count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?",
            )
            .bind(table)
            .fetch_one(&pool)
            .await
            .unwrap();
            assert_eq!(count, 1, "missing table {table}");
        }
    }
}
