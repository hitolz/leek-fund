use crate::errors::{AppError, AppResult};
use crate::models::{FundInfo, GroupFundPosition};
use chrono::Utc;
use sqlx::{Row, SqlitePool};

pub async fn get_group_fund_position(
    pool: &SqlitePool,
    list_id: i64,
    fund_code: &str,
) -> AppResult<Option<GroupFundPosition>> {
    ensure_list_exists(pool, list_id).await?;
    if !FundInfo::validate_code(fund_code) {
        return Err(AppError::ValidationError("基金代码格式错误".to_string()));
    }

    let row = sqlx::query(
        "SELECT holding_amount, holding_shares, created_at, updated_at \
         FROM group_fund_positions WHERE group_id = ? AND fund_code = ?",
    )
    .bind(list_id)
    .bind(fund_code)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    Ok(row.map(|row| GroupFundPosition {
        list_id,
        fund_code: fund_code.to_string(),
        holding_amount: row.try_get("holding_amount").unwrap_or(0.0),
        holding_shares: row.try_get("holding_shares").unwrap_or(0.0),
        created_at: row.try_get("created_at").unwrap_or(0),
        updated_at: row.try_get("updated_at").unwrap_or(0),
    }))
}

pub async fn set_group_fund_position(
    pool: &SqlitePool,
    list_id: i64,
    fund_code: &str,
    holding_amount: f64,
    holding_shares: f64,
) -> AppResult<GroupFundPosition> {
    ensure_list_exists(pool, list_id).await?;
    if !FundInfo::validate_code(fund_code) {
        return Err(AppError::ValidationError("基金代码格式错误".to_string()));
    }
    let holding_amount = round_two_decimals(holding_amount);
    let holding_shares = round_two_decimals(holding_shares);

    if !holding_amount.is_finite() || holding_amount < 0.0 {
        return Err(AppError::ValidationError("持仓金额无效".to_string()));
    }
    if !holding_shares.is_finite() || holding_shares < 0.0 {
        return Err(AppError::ValidationError("持仓份额无效".to_string()));
    }

    let now = Utc::now().timestamp();
    sqlx::query(
        "INSERT INTO group_fund_positions \
        (group_id, fund_code, holding_amount, holding_shares, created_at, updated_at) \
        VALUES (?, ?, ?, ?, ?, ?) \
        ON CONFLICT(group_id, fund_code) DO UPDATE SET \
        holding_amount = excluded.holding_amount, holding_shares = excluded.holding_shares, updated_at = excluded.updated_at",
    )
    .bind(list_id)
    .bind(fund_code)
    .bind(holding_amount)
    .bind(holding_shares)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(GroupFundPosition {
        list_id,
        fund_code: fund_code.to_string(),
        holding_amount,
        holding_shares,
        created_at: now,
        updated_at: now,
    })
}

fn round_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub async fn clear_group_fund_position(
    pool: &SqlitePool,
    list_id: i64,
    fund_code: &str,
) -> AppResult<()> {
    ensure_list_exists(pool, list_id).await?;
    if !FundInfo::validate_code(fund_code) {
        return Err(AppError::ValidationError("基金代码格式错误".to_string()));
    }

    sqlx::query("DELETE FROM group_fund_positions WHERE group_id = ? AND fund_code = ?")
        .bind(list_id)
        .bind(fund_code)
        .execute(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

async fn ensure_list_exists(pool: &SqlitePool, list_id: i64) -> AppResult<()> {
    let exists = sqlx::query("SELECT id FROM groups WHERE id = ?")
        .bind(list_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;
    if exists.is_none() {
        return Err(AppError::ListNotFound(list_id.to_string()));
    }
    Ok(())
}
