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
        "SELECT shares, unit_price, created_at, updated_at \
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
        shares: row.try_get("shares").unwrap_or(0.0),
        unit_price: row.try_get("unit_price").ok(),
        created_at: row.try_get("created_at").unwrap_or(0),
        updated_at: row.try_get("updated_at").unwrap_or(0),
    }))
}

pub async fn set_group_fund_position(
    pool: &SqlitePool,
    list_id: i64,
    fund_code: &str,
    shares: f64,
    unit_price: Option<f64>,
) -> AppResult<GroupFundPosition> {
    ensure_list_exists(pool, list_id).await?;
    if !FundInfo::validate_code(fund_code) {
        return Err(AppError::ValidationError("基金代码格式错误".to_string()));
    }
    if !shares.is_finite() || shares <= 0.0 {
        return Err(AppError::ValidationError("持仓份额无效".to_string()));
    }
    if let Some(price) = unit_price {
        if !price.is_finite() || price < 0.0 {
            return Err(AppError::ValidationError("成本价无效".to_string()));
        }
    }

    let now = Utc::now().timestamp();
    sqlx::query(
        "INSERT INTO group_fund_positions \
        (group_id, fund_code, shares, unit_price, created_at, updated_at) \
        VALUES (?, ?, ?, ?, ?, ?) \
        ON CONFLICT(group_id, fund_code) DO UPDATE SET \
        shares = excluded.shares, unit_price = excluded.unit_price, updated_at = excluded.updated_at",
    )
    .bind(list_id)
    .bind(fund_code)
    .bind(shares)
    .bind(unit_price)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(GroupFundPosition {
        list_id,
        fund_code: fund_code.to_string(),
        shares,
        unit_price,
        created_at: now,
        updated_at: now,
    })
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
