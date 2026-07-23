use crate::errors::{AppError, AppResult};
use chrono::Utc;
use sqlx::{Row, SqlitePool};

/// 股票持仓信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockHolding {
    pub code: String,
    pub holding_amount: f64,
    pub holding_shares: f64,
    pub cost_price: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 加密货币持仓信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CryptoHolding {
    pub symbol: String,
    pub holding_amount: f64,
    pub holding_quantity: f64,
    pub cost_price: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 初始化股票持仓表
pub async fn init_stock_holdings_table(pool: &SqlitePool) -> AppResult<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS stock_holdings (
            code TEXT PRIMARY KEY,
            holding_amount REAL NOT NULL DEFAULT 0,
            holding_shares REAL NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("创建股票持仓表失败: {}", e)))?;
    Ok(())
}

/// 初始化加密货币持仓表
pub async fn init_crypto_holdings_table(pool: &SqlitePool) -> AppResult<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS crypto_holdings (
            symbol TEXT PRIMARY KEY,
            holding_amount REAL NOT NULL DEFAULT 0,
            holding_quantity REAL NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
    )
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("创建加密货币持仓表失败: {}", e)))?;
    Ok(())
}

// ============================================================================
// 股票持仓
// ============================================================================

/// 获取股票持仓
pub async fn get_stock_holding(
    pool: &SqlitePool,
    code: &str,
) -> AppResult<Option<StockHolding>> {
    let row = sqlx::query(
        "SELECT holding_amount, holding_shares, created_at, updated_at \
         FROM stock_holdings WHERE code = ?",
    )
    .bind(code)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    Ok(row.map(|row| {
        let holding_amount: f64 = row.try_get("holding_amount").unwrap_or(0.0);
        let holding_shares: f64 = row.try_get("holding_shares").unwrap_or(0.0);
        let cost_price = if holding_shares > 0.0 {
            holding_amount / holding_shares
        } else {
            0.0
        };
        StockHolding {
            code: code.to_string(),
            holding_amount,
            holding_shares,
            cost_price,
            created_at: row.try_get("created_at").unwrap_or(0),
            updated_at: row.try_get("updated_at").unwrap_or(0),
        }
    }))
}

/// 设置股票持仓
pub async fn set_stock_holding(
    pool: &SqlitePool,
    code: &str,
    holding_amount: f64,
    holding_shares: f64,
) -> AppResult<StockHolding> {
    let holding_amount = round_two_decimals(holding_amount);
    let holding_shares = round_two_decimals(holding_shares);

    if !holding_amount.is_finite() || holding_amount < 0.0 {
        return Err(AppError::ValidationError("持仓金额无效".to_string()));
    }
    if !holding_shares.is_finite() || holding_shares < 0.0 {
        return Err(AppError::ValidationError("持仓数量无效".to_string()));
    }

    let now = Utc::now().timestamp();
    sqlx::query(
        "INSERT INTO stock_holdings (code, holding_amount, holding_shares, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?) \
         ON CONFLICT(code) DO UPDATE SET \
         holding_amount = excluded.holding_amount, holding_shares = excluded.holding_shares, \
         updated_at = excluded.updated_at",
    )
    .bind(code)
    .bind(holding_amount)
    .bind(holding_shares)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    let cost_price = if holding_shares > 0.0 {
        holding_amount / holding_shares
    } else {
        0.0
    };

    Ok(StockHolding {
        code: code.to_string(),
        holding_amount,
        holding_shares,
        cost_price,
        created_at: now,
        updated_at: now,
    })
}

/// 清空股票持仓
pub async fn clear_stock_holding(pool: &SqlitePool, code: &str) -> AppResult<()> {
    sqlx::query("DELETE FROM stock_holdings WHERE code = ?")
        .bind(code)
        .execute(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
    Ok(())
}

// ============================================================================
// 加密货币持仓
// ============================================================================

/// 获取加密货币持仓
pub async fn get_crypto_holding(
    pool: &SqlitePool,
    symbol: &str,
) -> AppResult<Option<CryptoHolding>> {
    let row = sqlx::query(
        "SELECT holding_amount, holding_quantity, created_at, updated_at \
         FROM crypto_holdings WHERE symbol = ?",
    )
    .bind(symbol)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库读取失败: {}", e)))?;

    Ok(row.map(|row| {
        let holding_amount: f64 = row.try_get("holding_amount").unwrap_or(0.0);
        let holding_quantity: f64 = row.try_get("holding_quantity").unwrap_or(0.0);
        let cost_price = if holding_quantity > 0.0 {
            holding_amount / holding_quantity
        } else {
            0.0
        };
        CryptoHolding {
            symbol: symbol.to_string(),
            holding_amount,
            holding_quantity,
            cost_price,
            created_at: row.try_get("created_at").unwrap_or(0),
            updated_at: row.try_get("updated_at").unwrap_or(0),
        }
    }))
}

/// 设置加密货币持仓
pub async fn set_crypto_holding(
    pool: &SqlitePool,
    symbol: &str,
    holding_amount: f64,
    holding_quantity: f64,
) -> AppResult<CryptoHolding> {
    let holding_amount = round_two_decimals(holding_amount);
    let holding_quantity = round_eight_decimals(holding_quantity); // 加密货币精度更高

    if !holding_amount.is_finite() || holding_amount < 0.0 {
        return Err(AppError::ValidationError("持仓金额无效".to_string()));
    }
    if !holding_quantity.is_finite() || holding_quantity < 0.0 {
        return Err(AppError::ValidationError("持仓数量无效".to_string()));
    }

    let now = Utc::now().timestamp();
    sqlx::query(
        "INSERT INTO crypto_holdings (symbol, holding_amount, holding_quantity, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?) \
         ON CONFLICT(symbol) DO UPDATE SET \
         holding_amount = excluded.holding_amount, holding_quantity = excluded.holding_quantity, \
         updated_at = excluded.updated_at",
    )
    .bind(symbol)
    .bind(holding_amount)
    .bind(holding_quantity)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    let cost_price = if holding_quantity > 0.0 {
        holding_amount / holding_quantity
    } else {
        0.0
    };

    Ok(CryptoHolding {
        symbol: symbol.to_string(),
        holding_amount,
        holding_quantity,
        cost_price,
        created_at: now,
        updated_at: now,
    })
}

/// 清空加密货币持仓
pub async fn clear_crypto_holding(pool: &SqlitePool, symbol: &str) -> AppResult<()> {
    sqlx::query("DELETE FROM crypto_holdings WHERE symbol = ?")
        .bind(symbol)
        .execute(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
    Ok(())
}

// ============================================================================
// 工具函数
// ============================================================================

fn round_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn round_eight_decimals(value: f64) -> f64 {
    (value * 100000000.0).round() / 100000000.0
}
