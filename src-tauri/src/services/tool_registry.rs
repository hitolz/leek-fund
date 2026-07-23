use crate::errors::{AppError, AppResult};
use crate::modules::{crypto_api, fund_api, gold_api, stock_api};
use crate::services::llm_client::{FunctionDefinition, ToolDefinition};
use sqlx::{Row, SqlitePool};

// ============================================================================
// Tool definitions
// ============================================================================

pub fn get_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_stock_quote".into(),
                description: "获取股票实时行情，包括当前价格、涨跌幅等。支持A股（如 sh600519）、港股（如 hk00700、hk01810）。当用户提到股票代码时使用此工具。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "code": {
                            "type": "string",
                            "description": "股票代码，格式如 sh600519（A股）、hk00700（港股）。用户可能只说数字如 01810，应推测为港股并使用 hk01810"
                        }
                    },
                    "required": ["code"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "search_stock".into(),
                description: "根据关键词搜索股票，返回匹配的股票代码和名称。当用户提到不确定代码的股票时使用。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "keyword": {
                            "type": "string",
                            "description": "搜索关键词，可以是股票名称或代码"
                        }
                    },
                    "required": ["keyword"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_fund_info".into(),
                description: "根据基金代码获取基金的实时估值信息，包括基金名称、最新净值、涨跌幅等。当用户提到6位基金代码时使用此工具。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "fund_code": {
                            "type": "string",
                            "description": "6位基金代码，例如 012733"
                        }
                    },
                    "required": ["fund_code"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_fund_detail".into(),
                description: "获取基金的详细信息，包括净值、涨跌幅、持仓市值、持有份额等。需要用户已添加该基金到自选列表。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "fund_code": {
                            "type": "string",
                            "description": "6位基金代码"
                        }
                    },
                    "required": ["fund_code"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_local_fund_profile".into(),
                description: "查询本地数据库中存储的基金档案信息，包括基金名称、费率、历史收益、资产配置、基金经理等详细数据。适用于需要深入分析基金的场景。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "fund_code": {
                            "type": "string",
                            "description": "6位基金代码"
                        }
                    },
                    "required": ["fund_code"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_crypto_quote".into(),
                description: "获取加密货币实时行情。支持 BTC、ETH、SOL 等主流币种。当用户提到加密货币、比特币、以太坊等时使用。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "symbol": {
                            "type": "string",
                            "description": "交易对符号，如 BTCUSDT、ETHUSDT。如果用户只说 BTC，应补全为 BTCUSDT"
                        }
                    },
                    "required": ["symbol"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_gold_quote".into(),
                description: "获取黄金 AU9999 实时行情，包括价格、涨跌幅等。当用户提到黄金、金价时使用。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {},
                }),
            },
        },
    ]
}

// ============================================================================
// Tool execution
// ============================================================================

pub async fn execute_tool(
    name: &str,
    arguments: &str,
    pool: &SqlitePool,
) -> AppResult<String> {
    let args: serde_json::Value = serde_json::from_str(arguments)
        .map_err(|e| AppError::ValidationError(format!("工具参数解析失败: {e}")))?;

    eprintln!("[AI_CHAT] 执行工具: {}({})", name, arguments);

    let result = match name {
        "get_stock_quote" => {
            let code = get_string_arg(&args, "code")?;
            execute_get_stock_quote(&code).await
        }
        "search_stock" => {
            let keyword = get_string_arg(&args, "keyword")?;
            execute_search_stock(&keyword).await
        }
        "get_fund_info" => {
            let fund_code = get_string_arg(&args, "fund_code")?;
            execute_get_fund_info(&fund_code).await
        }
        "get_fund_detail" => {
            let fund_code = get_string_arg(&args, "fund_code")?;
            execute_get_fund_detail(&fund_code).await
        }
        "get_local_fund_profile" => {
            let fund_code = get_string_arg(&args, "fund_code")?;
            execute_get_local_fund_profile(&fund_code, pool).await
        }
        "get_crypto_quote" => {
            let symbol = get_string_arg(&args, "symbol")?;
            execute_get_crypto_quote(&symbol).await
        }
        "get_gold_quote" => execute_get_gold_quote().await,
        _ => Err(AppError::ValidationError(format!("未知工具: {name}"))),
    };

    match &result {
        Ok(data) => eprintln!("[AI_CHAT] 工具结果: {} 字符", data.chars().count()),
        Err(e) => eprintln!("[AI_CHAT] 工具执行失败: {}", e.details()),
    }

    result
}

fn get_string_arg(args: &serde_json::Value, key: &str) -> AppResult<String> {
    args[key]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::ValidationError(format!("缺少 {key} 参数")))
}

// ============================================================================
// Stock tools
// ============================================================================

async fn execute_search_stock(keyword: &str) -> AppResult<String> {
    let results = stock_api::search_stock(keyword).await?;
    serde_json::to_string_pretty(&serde_json::json!({
        "results": results.iter().map(|r| serde_json::json!({
            "code": r.code,
            "name": r.name,
            "market": r.market,
        })).collect::<Vec<_>>(),
    }))
    .map_err(AppError::from)
}

async fn execute_get_stock_quote(code: &str) -> AppResult<String> {
    let quote = stock_api::get_stock_quote(code).await?;
    serde_json::to_string_pretty(&serde_json::json!({
        "code": quote.code,
        "name": quote.name,
        "price": quote.price,
        "change_percent": quote.change_percent,
        "change_amount": quote.change_amount,
        "open": quote.open,
        "high": quote.high,
        "low": quote.low,
        "yesterday_close": quote.yesterday_close,
        "volume": quote.volume,
    }))
    .map_err(AppError::from)
}

// ============================================================================
// Fund tools
// ============================================================================

async fn execute_get_fund_info(fund_code: &str) -> AppResult<String> {
    let info = fund_api::search_fund_info(fund_code).await?;
    serde_json::to_string_pretty(&serde_json::json!({
        "code": info.code,
        "name": info.name,
        "net_value": info.net_value,
        "change_percent": info.change_percent,
        "update_time": info.update_time,
    }))
    .map_err(AppError::from)
}

async fn execute_get_fund_detail(fund_code: &str) -> AppResult<String> {
    let detail = fund_api::get_fund_detail(fund_code).await?;
    serde_json::to_string_pretty(&serde_json::json!({
        "code": detail.code,
        "name": detail.name,
        "net_value": detail.net_value,
        "change_percent": detail.change_percent,
        "daily_change_amount": detail.daily_change_amount,
        "update_time": detail.update_time,
        "holding_amount": detail.holding_amount,
        "holding_shares": detail.holding_shares,
        "cost_price": detail.cost_price,
    }))
    .map_err(AppError::from)
}

async fn execute_get_local_fund_profile(
    fund_code: &str,
    pool: &SqlitePool,
) -> AppResult<String> {
    let profile = sqlx::query(
        "SELECT name, is_money, source_rate, rate, min_purchase FROM fund_profile WHERE fund_code = ?",
    )
    .bind(fund_code)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("查询基金档案失败: {e}")))?;

    let profile = match profile {
        Some(row) => serde_json::json!({
            "name": row.get::<Option<String>, _>("name"),
            "is_money": row.get::<Option<i64>, _>("is_money").unwrap_or(0) == 1,
            "source_rate": row.get::<Option<f64>, _>("source_rate"),
            "rate": row.get::<Option<f64>, _>("rate"),
            "min_purchase": row.get::<Option<f64>, _>("min_purchase"),
        }),
        None => {
            return Ok(serde_json::json!({
                "found": false,
                "message": format!("本地数据库中未找到基金 {} 的档案信息，可能需要先同步数据", fund_code)
            })
            .to_string());
        }
    };

    let returns = sqlx::query(
        "SELECT period, value FROM fund_return_summary WHERE fund_code = ? ORDER BY period",
    )
    .bind(fund_code)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("查询收益数据失败: {e}")))?;

    let returns: Vec<serde_json::Value> = returns
        .iter()
        .map(|row| {
            serde_json::json!({
                "period": row.get::<String, _>("period"),
                "return_pct": row.get::<f64, _>("value"),
            })
        })
        .collect();

    let allocation = sqlx::query(
        "SELECT stock_pct, bond_pct, cash_pct, other_pct FROM fund_asset_allocation WHERE fund_code = ? ORDER BY report_date DESC LIMIT 1",
    )
    .bind(fund_code)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("查询资产配置失败: {e}")))?;

    let allocation = allocation.map(|row| {
        serde_json::json!({
            "stock_pct": row.get::<Option<f64>, _>("stock_pct"),
            "bond_pct": row.get::<Option<f64>, _>("bond_pct"),
            "cash_pct": row.get::<Option<f64>, _>("cash_pct"),
            "other_pct": row.get::<Option<f64>, _>("other_pct"),
        })
    });

    let managers = sqlx::query(
        "SELECT fm.name, fm.star, fm.work_time_text FROM fund_manager_rel fmr \
         JOIN fund_manager fm ON fmr.manager_id = fm.manager_id \
         WHERE fmr.fund_code = ?",
    )
    .bind(fund_code)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("查询基金经理失败: {e}")))?;

    let managers: Vec<serde_json::Value> = managers
        .iter()
        .map(|row| {
            serde_json::json!({
                "name": row.get::<String, _>("name"),
                "star": row.get::<Option<i64>, _>("star"),
                "tenure": row.get::<Option<String>, _>("work_time_text"),
            })
        })
        .collect();

    serde_json::to_string_pretty(&serde_json::json!({
        "found": true,
        "profile": profile,
        "return_summary": returns,
        "asset_allocation": allocation,
        "managers": managers,
    }))
    .map_err(AppError::from)
}

// ============================================================================
// Crypto tools
// ============================================================================

async fn execute_get_crypto_quote(symbol: &str) -> AppResult<String> {
    let quote = crypto_api::get_crypto_quote(symbol).await?;
    serde_json::to_string_pretty(&serde_json::json!({
        "symbol": quote.symbol,
        "name": quote.name,
        "price": quote.price,
        "change_percent": quote.change_percent,
        "high_24h": quote.high_24h,
        "low_24h": quote.low_24h,
        "volume_24h": quote.volume_24h,
        "update_time": quote.update_time,
    }))
    .map_err(AppError::from)
}

// ============================================================================
// Gold tools
// ============================================================================

async fn execute_get_gold_quote() -> AppResult<String> {
    let quote = gold_api::get_gold_quote().await?;
    serde_json::to_string_pretty(&serde_json::json!({
        "code": quote.code,
        "name": quote.name,
        "price": quote.price,
        "change_percent": quote.change_percent,
        "change_amount": quote.change_amount,
        "open": quote.open,
        "high": quote.high,
        "low": quote.low,
        "yesterday_close": quote.yesterday_close,
        "volume": quote.volume,
        "update_time": quote.update_time,
    }))
    .map_err(AppError::from)
}
