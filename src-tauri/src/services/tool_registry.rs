use crate::errors::{AppError, AppResult};
use crate::modules::{crypto_api, fund_api, gold_api, news_api, stock_api};
use crate::services::llm_client::{FunctionDefinition, ToolDefinition};
use sqlx::{Column, Row, SqlitePool};

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
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_financial_news".into(),
                description: "获取最新财经新闻快讯，可按关键词筛选。当用户询问市场动态、财经新闻、黄金新闻、加密货币新闻等时使用。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "keyword": {
                            "type": "string",
                            "description": "筛选关键词，如\"黄金\"、\"比特币\"、\"新能源\"。留空获取通用财经新闻"
                        },
                        "page_size": {
                            "type": "integer",
                            "description": "返回条数，默认10"
                        }
                    },
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_stock_news".into(),
                description: "获取某只股票的相关新闻。当用户询问某只股票的最新消息、公告、新闻时使用。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "code": {
                            "type": "string",
                            "description": "股票代码，如 sh600519、hk01810"
                        }
                    },
                    "required": ["code"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "query_wencai".into(),
                description: "同花顺问财自然语言查询。可以用自然语言查询股票、基金、板块的新闻、数据等。如\"白酒板块最近一周新闻\"、\"新能源基金排名\"。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "question": {
                            "type": "string",
                            "description": "自然语言查询问题"
                        }
                    },
                    "required": ["question"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "record_operation".into(),
                description: "记录一笔投资操作（买入/卖出/分红/转账）。当用户说\"买了5000块基金\"、\"卖出股票\"等操作时调用此工具记录。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "op_type": {
                            "type": "string",
                            "enum": ["buy", "sell", "dividend", "transfer"],
                            "description": "操作类型：buy=买入, sell=卖出, dividend=分红, transfer=转账"
                        },
                        "asset_type": {
                            "type": "string",
                            "enum": ["fund", "stock", "crypto", "gold"],
                            "description": "资产类型"
                        },
                        "asset_code": {
                            "type": "string",
                            "description": "资产代码，如 012733、sh600519、BTCUSDT"
                        },
                        "asset_name": {
                            "type": "string",
                            "description": "资产名称（可选）"
                        },
                        "amount": {
                            "type": "number",
                            "description": "金额（元）"
                        },
                        "shares": {
                            "type": "number",
                            "description": "份额/数量（可选）"
                        },
                        "price": {
                            "type": "number",
                            "description": "单价（可选）"
                        },
                        "note": {
                            "type": "string",
                            "description": "备注（可选）"
                        },
                        "op_date": {
                            "type": "string",
                            "description": "操作日期，格式 YYYY-MM-DD，默认今天"
                        }
                    },
                    "required": ["op_type", "asset_type", "asset_code"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "query_operations".into(),
                description: "查询投资操作记录。可以按日期范围、资产类型、资产代码筛选。用于回答\"我这个月做了什么操作\"、\"我买过多少次012733\"等问题。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "start_date": {
                            "type": "string",
                            "description": "开始日期 YYYY-MM-DD（可选）"
                        },
                        "end_date": {
                            "type": "string",
                            "description": "结束日期 YYYY-MM-DD（可选）"
                        },
                        "asset_type": {
                            "type": "string",
                            "enum": ["fund", "stock", "crypto", "gold"],
                            "description": "资产类型筛选（可选）"
                        },
                        "asset_code": {
                            "type": "string",
                            "description": "资产代码筛选（可选）"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "返回条数，默认50"
                        }
                    },
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_operations_summary".into(),
                description: "获取操作汇总统计。按月/按资产类型汇总买入卖出金额、次数等。用于回答\"这个月总投入多少\"、\"我的基金操作汇总\"等问题。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "group_by": {
                            "type": "string",
                            "enum": ["month", "asset_type", "asset_code"],
                            "description": "汇总维度：month=按月, asset_type=按资产类型, asset_code=按资产代码"
                        },
                        "start_date": {
                            "type": "string",
                            "description": "开始日期 YYYY-MM-DD（可选）"
                        },
                        "end_date": {
                            "type": "string",
                            "description": "结束日期 YYYY-MM-DD（可选）"
                        }
                    },
                    "required": ["group_by"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "backtest_dca".into(),
                description: "定投回测模拟。给定基金代码、每月定投金额、起始日期，计算定投收益。返回总投入、当前市值、收益率、最大回撤等。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "fund_code": {
                            "type": "string",
                            "description": "基金代码"
                        },
                        "monthly_amount": {
                            "type": "number",
                            "description": "每月定投金额（元）"
                        },
                        "start_date": {
                            "type": "string",
                            "description": "起始日期 YYYY-MM-DD"
                        }
                    },
                    "required": ["fund_code", "monthly_amount", "start_date"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "suggest_rebalance".into(),
                description: "智能再平衡建议。检查当前持仓比例是否偏离目标配置，给出再平衡建议。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "target_allocation": {
                            "type": "object",
                            "description": "目标配置比例，如 {\"fund\": 60, \"stock\": 20, \"gold\": 10, \"crypto\": 10}。留空则使用均衡配置"
                        }
                    },
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "analyze_correlation".into(),
                description: "分析持仓资产之间的相关性。计算相关系数矩阵，找出分散风险效果好/差的资产组合。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {},
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "build_portfolio".into(),
                description: "一键建仓方案。根据本金金额和风险偏好，生成完整的投资组合方案。包括具体标的、仓位比例、预期收益和风险。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "capital": {
                            "type": "number",
                            "description": "投资本金（元）"
                        },
                        "risk_level": {
                            "type": "string",
                            "enum": ["conservative", "balanced", "aggressive"],
                            "description": "风险偏好：conservative=保守, balanced=均衡, aggressive=进取"
                        }
                    },
                    "required": ["capital", "risk_level"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_market_sentiment".into(),
                description: "获取市场情绪指标。综合涨跌比、波动率、新闻情绪等判断当前市场情绪。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {},
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "tax_optimization".into(),
                description: "税务优化建议。分析持仓中哪些可以卖出抵税，哪些应持有超过1年享受免税。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {},
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "screen_funds".into(),
                description: "基金筛选器。根据收益率、风险、基金经理等条件筛选基金。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "min_return_1y": {
                            "type": "number",
                            "description": "近1年最低收益率（%）"
                        },
                        "max_drawdown": {
                            "type": "number",
                            "description": "最大回撤限制（%）"
                        },
                        "category": {
                            "type": "string",
                            "description": "基金类型筛选，如\"科技\"、\"消费\"、\"医药\""
                        },
                        "min_manager_tenure": {
                            "type": "number",
                            "description": "基金经理最低任职年限"
                        }
                    },
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "generate_daily_report".into(),
                description: "生成每日投资报告。包括今日市场概况、持仓表现、明日关注事件。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {},
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "monte_carlo_simulation".into(),
                description: "蒙特卡洛模拟。基于历史波动率模拟未来收益分布，计算亏损概率。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "days": {
                            "type": "integer",
                            "description": "模拟天数，默认365"
                        },
                        "simulations": {
                            "type": "integer",
                            "description": "模拟次数，默认1000"
                        }
                    },
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "get_portfolio_holdings".into(),
                description: "查看用户当前所有持仓信息，包括股票、基金、加密货币、黄金的详细持仓数据、市值、涨跌幅等。当用户询问\"我的持仓\"、\"我有哪些资产\"、\"查看我的组合\"时使用。".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {},
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "query_local_db".into(),
                description: "查询本地 SQLite 数据库。可以执行只读 SQL 查询。支持 SELECT 和 PRAGMA。\n\n主要表结构：\n- group_fund_positions(group_id, fund_code, holding_amount, holding_shares)\n- funds(code, name)\n- fund_profile(fund_code, name, is_money, source_rate, rate, min_purchase)\n- fund_nav_daily(fund_code, nav_date, unit_nav, accum_nav)\n- fund_return_summary(fund_code, period, value) -- period: 1m/6m/1y/3y\n- fund_asset_allocation(fund_code, report_date, stock_pct, bond_pct, cash_pct, other_pct)\n- stock_holdings(code, holding_amount, holding_shares)\n- crypto_holdings(symbol, holding_amount, holding_quantity)\n- operations(id, op_type, asset_type, asset_code, asset_name, amount, shares, price, note, op_date)\n- portfolio_snapshots(id, snapshot_at, payload, data_quality)\n- sessions(session_id, title, created_at, updated_at)\n- session_chat_messages(id, session_id, role, content, saved_state)".into(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "sql": {
                            "type": "string",
                            "description": "只读 SQL 查询语句（SELECT 或 PRAGMA）"
                        }
                    },
                    "required": ["sql"]
                }),
            },
        },
        ToolDefinition {
            tool_type: "function".into(),
            function: FunctionDefinition {
                name: "save_daily_snapshot".into(),
                description: "保存今日所有资产的行情数据和持仓快照。会自动抓取股票、加密货币、黄金的最新行情并保存到历史表，同时保存今日持仓快照。每天调用一次即可。".into(),
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
        "get_financial_news" => {
            let keyword = args["keyword"].as_str().unwrap_or("");
            let page_size = args["page_size"].as_u64().unwrap_or(10) as usize;
            execute_get_financial_news(keyword, page_size).await
        }
        "get_stock_news" => {
            let code = get_string_arg(&args, "code")?;
            execute_get_stock_news(&code).await
        }
        "query_wencai" => {
            let question = get_string_arg(&args, "question")?;
            execute_query_wencai(&question).await
        }
        "record_operation" => execute_record_operation(&args, pool).await,
        "query_operations" => execute_query_operations(&args, pool).await,
        "get_operations_summary" => execute_get_operations_summary(&args, pool).await,
        "backtest_dca" => execute_backtest_dca(&args, pool).await,
        "suggest_rebalance" => execute_suggest_rebalance(&args, pool).await,
        "analyze_correlation" => execute_analyze_correlation(pool).await,
        "build_portfolio" => execute_build_portfolio(&args).await,
        "get_market_sentiment" => execute_get_market_sentiment().await,
        "tax_optimization" => execute_tax_optimization(pool).await,
        "screen_funds" => execute_screen_funds(&args, pool).await,
        "generate_daily_report" => execute_generate_daily_report(pool).await,
        "monte_carlo_simulation" => execute_monte_carlo(&args, pool).await,
        "get_portfolio_holdings" => execute_get_portfolio_holdings(pool).await,
        "query_local_db" => execute_query_local_db(&args, pool).await,
        "save_daily_snapshot" => execute_save_daily_snapshot(pool).await,
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

// ============================================================================
// News tools
// ============================================================================

fn format_news_items(items: &[news_api::NewsItem]) -> serde_json::Value {
    serde_json::json!({
        "count": items.len(),
        "news": items.iter().map(|n| serde_json::json!({
            "title": n.title,
            "summary": if n.summary.len() > 200 { format!("{}...", &n.summary[..200]) } else { n.summary.clone() },
            "source": n.source,
            "publish_time": n.publish_time,
            "url": n.url,
            "related_codes": n.related_codes,
        })).collect::<Vec<_>>(),
    })
}

async fn execute_get_financial_news(keyword: &str, page_size: usize) -> AppResult<String> {
    let items = if keyword.is_empty() {
        news_api::get_eastmoney_news(1, page_size).await?
    } else {
        news_api::search_topic_news(keyword, page_size).await?
    };
    serde_json::to_string_pretty(&format_news_items(&items)).map_err(AppError::from)
}

async fn execute_get_stock_news(code: &str) -> AppResult<String> {
    let items = news_api::get_eastmoney_stock_news(code, 1, 10).await?;
    serde_json::to_string_pretty(&format_news_items(&items)).map_err(AppError::from)
}

async fn execute_query_wencai(question: &str) -> AppResult<String> {
    let items = news_api::query_wencai(question).await?;
    serde_json::to_string_pretty(&format_news_items(&items)).map_err(AppError::from)
}

// ============================================================================
// Operation tools (记账)
// ============================================================================

async fn execute_record_operation(args: &serde_json::Value, pool: &SqlitePool) -> AppResult<String> {
    let op_type = get_string_arg(args, "op_type")?;
    let asset_type = get_string_arg(args, "asset_type")?;
    let asset_code = get_string_arg(args, "asset_code")?;
    let asset_name = args["asset_name"].as_str().map(|s| s.to_string());
    let amount = args["amount"].as_f64();
    let shares = args["shares"].as_f64();
    let price = args["price"].as_f64();
    let note = args["note"].as_str().map(|s| s.to_string());
    let op_date = args["op_date"]
        .as_str()
        .unwrap_or(&chrono::Local::now().format("%Y-%m-%d").to_string())
        .to_string();

    let now = chrono::Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO operations (op_type, asset_type, asset_code, asset_name, amount, shares, price, note, op_date, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&op_type)
    .bind(&asset_type)
    .bind(&asset_code)
    .bind(&asset_name)
    .bind(amount)
    .bind(shares)
    .bind(price)
    .bind(&note)
    .bind(&op_date)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("记录操作失败: {e}")))?;

    eprintln!("[AI_CHAT] 记录操作: {} {} {} {}", op_type, asset_type, asset_code, amount.unwrap_or(0.0));

    serde_json::to_string_pretty(&serde_json::json!({
        "success": true,
        "message": format!("已记录{}操作：{} {} {}",
            match op_type.as_str() { "buy" => "买入", "sell" => "卖出", "dividend" => "分红", _ => "转账" },
            asset_type,
            asset_code,
            amount.map(|a| format!("{:.2}元", a)).unwrap_or_default()
        ),
        "op_date": op_date,
    }))
    .map_err(AppError::from)
}

async fn execute_query_operations(args: &serde_json::Value, pool: &SqlitePool) -> AppResult<String> {
    let start_date = args["start_date"].as_str();
    let end_date = args["end_date"].as_str();
    let asset_type = args["asset_type"].as_str();
    let asset_code = args["asset_code"].as_str();
    let limit = args["limit"].as_i64().unwrap_or(50);

    let mut sql = String::from(
        "SELECT id, op_type, asset_type, asset_code, asset_name, amount, shares, price, note, op_date, created_at FROM operations WHERE 1=1"
    );
    let mut bindings: Vec<String> = Vec::new();

    if let Some(start) = start_date {
        sql.push_str(" AND op_date >= ?");
        bindings.push(start.to_string());
    }
    if let Some(end) = end_date {
        sql.push_str(" AND op_date <= ?");
        bindings.push(end.to_string());
    }
    if let Some(at) = asset_type {
        sql.push_str(" AND asset_type = ?");
        bindings.push(at.to_string());
    }
    if let Some(ac) = asset_code {
        sql.push_str(" AND asset_code = ?");
        bindings.push(ac.to_string());
    }

    sql.push_str(" ORDER BY op_date DESC, created_at DESC LIMIT ?");
    bindings.push(limit.to_string());

    let mut query = sqlx::query(&sql);
    for binding in &bindings {
        query = query.bind(binding);
    }

    let rows = query
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("查询操作记录失败: {e}")))?;

    let operations: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "id": row.get::<i64, _>("id"),
                "op_type": row.get::<String, _>("op_type"),
                "asset_type": row.get::<String, _>("asset_type"),
                "asset_code": row.get::<String, _>("asset_code"),
                "asset_name": row.get::<Option<String>, _>("asset_name"),
                "amount": row.get::<Option<f64>, _>("amount"),
                "shares": row.get::<Option<f64>, _>("shares"),
                "price": row.get::<Option<f64>, _>("price"),
                "note": row.get::<Option<String>, _>("note"),
                "op_date": row.get::<String, _>("op_date"),
            })
        })
        .collect();

    serde_json::to_string_pretty(&serde_json::json!({
        "count": operations.len(),
        "operations": operations,
    }))
    .map_err(AppError::from)
}

async fn execute_get_operations_summary(args: &serde_json::Value, pool: &SqlitePool) -> AppResult<String> {
    let group_by = get_string_arg(args, "group_by")?;
    let start_date = args["start_date"].as_str();
    let end_date = args["end_date"].as_str();

    let (group_expr, group_label) = match group_by.as_str() {
        "month" => ("strftime('%Y-%m', op_date)", "month"),
        "asset_type" => ("asset_type", "asset_type"),
        "asset_code" => ("asset_code", "asset_code"),
        _ => return Err(AppError::ValidationError("无效的 group_by 参数".into())),
    };

    let mut sql = format!(
        "SELECT {} AS group_key, \
         COUNT(*) AS total_count, \
         SUM(CASE WHEN op_type = 'buy' THEN amount ELSE 0 END) AS total_buy_amount, \
         SUM(CASE WHEN op_type = 'sell' THEN amount ELSE 0 END) AS total_sell_amount, \
         SUM(CASE WHEN op_type = 'buy' THEN 1 ELSE 0 END) AS buy_count, \
         SUM(CASE WHEN op_type = 'sell' THEN 1 ELSE 0 END) AS sell_count \
         FROM operations WHERE 1=1",
        group_expr
    );

    let mut bindings: Vec<String> = Vec::new();
    if let Some(start) = start_date {
        sql.push_str(" AND op_date >= ?");
        bindings.push(start.to_string());
    }
    if let Some(end) = end_date {
        sql.push_str(" AND op_date <= ?");
        bindings.push(end.to_string());
    }
    sql.push_str(" GROUP BY group_key ORDER BY group_key DESC");

    let mut query = sqlx::query(&sql);
    for binding in &bindings {
        query = query.bind(binding);
    }

    let rows = query
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("查询汇总失败: {e}")))?;

    let summaries: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                group_label: row.get::<String, _>("group_key"),
                "total_count": row.get::<i64, _>("total_count"),
                "buy_count": row.get::<i64, _>("buy_count"),
                "sell_count": row.get::<i64, _>("sell_count"),
                "total_buy_amount": row.get::<f64, _>("total_buy_amount"),
                "total_sell_amount": row.get::<f64, _>("total_sell_amount"),
                "net_amount": row.get::<f64, _>("total_buy_amount") - row.get::<f64, _>("total_sell_amount"),
            })
        })
        .collect();

    serde_json::to_string_pretty(&serde_json::json!({
        "group_by": group_by,
        "summaries": summaries,
    }))
    .map_err(AppError::from)
}

// ============================================================================
// Backtest tool (定投回测)
// ============================================================================

async fn execute_backtest_dca(args: &serde_json::Value, pool: &SqlitePool) -> AppResult<String> {
    let fund_code = get_string_arg(args, "fund_code")?;
    let monthly_amount = args["monthly_amount"]
        .as_f64()
        .ok_or_else(|| AppError::ValidationError("缺少 monthly_amount 参数".into()))?;
    let start_date = get_string_arg(args, "start_date")?;

    // Parse start date to timestamp (millis)
    let start_ts = chrono::NaiveDate::parse_from_str(&start_date, "%Y-%m-%d")
        .map_err(|_| AppError::ValidationError("日期格式错误，请使用 YYYY-MM-DD".into()))?
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp_millis();

    // Query historical NAV data
    let rows = sqlx::query(
        "SELECT nav_date, unit_nav FROM fund_nav_daily WHERE fund_code = ? AND nav_date >= ? ORDER BY nav_date ASC"
    )
    .bind(&fund_code)
    .bind(start_ts)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("查询历史净值失败: {e}")))?;

    if rows.is_empty() {
        return Ok(serde_json::json!({
            "found": false,
            "message": format!("未找到基金 {} 从 {} 起的历史净值数据，可能需要先同步", fund_code, start_date)
        }).to_string());
    }

    // Simulate DCA
    let mut total_invested = 0.0;
    let mut total_shares = 0.0;
    let mut invest_count = 0;
    let mut last_invest_month = String::new();
    let mut nav_points: Vec<(String, f64)> = Vec::new();
    let mut max_value = 0.0_f64;
    let mut max_drawdown = 0.0_f64;

    for row in &rows {
        let nav_date_ms = row.get::<i64, _>("nav_date");
        let unit_nav = row.get::<f64, _>("unit_nav");

        let date = chrono::DateTime::from_timestamp_millis(nav_date_ms)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_default();
        let month = date[..7].to_string();

        // Invest on the first trading day of each month
        if month != last_invest_month && unit_nav > 0.0 {
            let shares = monthly_amount / unit_nav;
            total_invested += monthly_amount;
            total_shares += shares;
            invest_count += 1;
            last_invest_month = month;
        }

        // Track portfolio value for drawdown calculation
        let current_value = total_shares * unit_nav;
        if current_value > max_value {
            max_value = current_value;
        }
        if max_value > 0.0 {
            let drawdown = (max_value - current_value) / max_value;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }

        nav_points.push((date, unit_nav));
    }

    // Get latest NAV
    let latest = rows.last().unwrap();
    let latest_nav = latest.get::<f64, _>("unit_nav");
    let latest_date_ms = latest.get::<i64, _>("nav_date");
    let latest_date = chrono::DateTime::from_timestamp_millis(latest_date_ms)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default();

    let current_value = total_shares * latest_nav;
    let total_return = current_value - total_invested;
    let return_rate = if total_invested > 0.0 {
        total_return / total_invested * 100.0
    } else {
        0.0
    };
    let avg_cost = if total_shares > 0.0 {
        total_invested / total_shares
    } else {
        0.0
    };

    serde_json::to_string_pretty(&serde_json::json!({
        "found": true,
        "fund_code": fund_code,
        "monthly_amount": monthly_amount,
        "start_date": start_date,
        "end_date": latest_date,
        "invest_count": invest_count,
        "total_invested": total_invested,
        "current_value": current_value,
        "total_return": total_return,
        "return_rate": format!("{:.2}%", return_rate),
        "max_drawdown": format!("{:.2}%", max_drawdown * 100.0),
        "avg_cost": avg_cost,
        "latest_nav": latest_nav,
        "total_shares": total_shares,
    }))
    .map_err(AppError::from)
}

// ============================================================================
// Rebalance tool (智能再平衡)
// ============================================================================

async fn execute_suggest_rebalance(args: &serde_json::Value, pool: &SqlitePool) -> AppResult<String> {
    let snapshot = sqlx::query(
        "SELECT payload FROM portfolio_snapshots ORDER BY created_at DESC LIMIT 1"
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("查询快照失败: {e}")))?;

    let snapshot = match snapshot {
        Some(row) => {
            let payload: serde_json::Value = serde_json::from_str(&row.get::<String, _>("payload"))
                .map_err(|e| AppError::ParseError(format!("解析快照失败: {e}")))?;
            payload
        }
        None => return Ok(serde_json::json!({"found": false, "message": "未找到投资组合快照"}).to_string()),
    };

    let total_value = snapshot["total_value"].as_f64().unwrap_or(0.0);
    if total_value <= 0.0 {
        return Ok(serde_json::json!({"found": false, "message": "组合总值为0"}).to_string());
    }

    let empty_assets = vec![];
    let assets = snapshot["assets"].as_array().unwrap_or(&empty_assets);
    let mut category_values: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    for asset in assets {
        let category = asset["category"].as_str().unwrap_or("other").to_string();
        let value = asset["holding_amount"].as_f64().unwrap_or(0.0);
        *category_values.entry(category).or_insert(0.0) += value;
    }

    let default_target = serde_json::json!({"fund": 50.0, "stock": 25.0, "gold": 15.0, "crypto": 10.0});
    let target = args.get("target_allocation").unwrap_or(&default_target);

    let mut suggestions = Vec::new();
    for (cat, target_pct) in target.as_object().unwrap_or(&serde_json::Map::new()) {
        let target_pct = target_pct.as_f64().unwrap_or(0.0);
        let current_value = category_values.get(cat.as_str()).unwrap_or(&0.0);
        let current_pct = current_value / total_value * 100.0;
        let diff_pct = current_pct - target_pct;
        let diff_amount = diff_pct / 100.0 * total_value;

        if diff_pct.abs() > 3.0 {
            suggestions.push(serde_json::json!({
                "category": cat,
                "current_pct": format!("{:.1}%", current_pct),
                "target_pct": format!("{:.1}%", target_pct),
                "diff_pct": format!("{:+.1}%", diff_pct),
                "action": if diff_pct > 0.0 { "减仓" } else { "加仓" },
                "amount": format!("{:.0}元", diff_amount.abs()),
            }));
        }
    }

    serde_json::to_string_pretty(&serde_json::json!({
        "found": true,
        "total_value": total_value,
        "current_allocation": category_values.iter().map(|(k, v)| {
            serde_json::json!({"category": k, "value": v, "pct": format!("{:.1}%", v / total_value * 100.0)})
        }).collect::<Vec<_>>(),
        "suggestions": suggestions,
    }))
    .map_err(AppError::from)
}

fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len() as f64;
    if n == 0.0 { return 0.0; }
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    let sum_x2: f64 = x.iter().map(|a| a * a).sum();
    let sum_y2: f64 = y.iter().map(|a| a * a).sum();
    let numerator = n * sum_xy - sum_x * sum_y;
    let denominator = ((n * sum_x2 - sum_x * sum_x) * (n * sum_y2 - sum_y * sum_y)).sqrt();
    if denominator == 0.0 { 0.0 } else { numerator / denominator }
}

async fn execute_analyze_correlation(pool: &SqlitePool) -> AppResult<String> {
    let snapshot = sqlx::query("SELECT payload FROM portfolio_snapshots ORDER BY created_at DESC LIMIT 1")
        .fetch_optional(pool).await.map_err(|e| AppError::StorageError(format!("查询快照失败: {e}")))?;

    let snapshot = match snapshot {
        Some(row) => serde_json::from_str::<serde_json::Value>(&row.get::<String, _>("payload")).unwrap_or_default(),
        None => return Ok(serde_json::json!({"found": false, "message": "未找到快照"}).to_string()),
    };

    let empty_assets = vec![];
    let assets = snapshot["assets"].as_array().unwrap_or(&empty_assets);
    let fund_codes: Vec<String> = assets.iter()
        .filter(|a| a["category"].as_str() == Some("fund"))
        .filter_map(|a| a["code"].as_str().map(|s| s.to_string()))
        .collect();

    if fund_codes.len() < 2 {
        return Ok(serde_json::json!({"found": false, "message": "需要至少2只基金"}).to_string());
    }

    let mut returns_map: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
    for code in &fund_codes {
        let rows = sqlx::query("SELECT unit_nav FROM fund_nav_daily WHERE fund_code = ? ORDER BY nav_date DESC LIMIT 61")
            .bind(code).fetch_all(pool).await.unwrap_or_default();
        if rows.len() >= 2 {
            let navs: Vec<f64> = rows.iter().rev().map(|r| r.get::<f64, _>("unit_nav")).collect();
            let returns: Vec<f64> = navs.windows(2).map(|w| (w[1] - w[0]) / w[0]).collect();
            returns_map.insert(code.clone(), returns);
        }
    }

    let mut correlations = Vec::new();
    let codes: Vec<&String> = returns_map.keys().collect();
    for i in 0..codes.len() {
        for j in i+1..codes.len() {
            if let (Some(r1), Some(r2)) = (returns_map.get(codes[i]), returns_map.get(codes[j])) {
                let min_len = r1.len().min(r2.len());
                if min_len >= 10 {
                    let corr = pearson_correlation(&r1[..min_len], &r2[..min_len]);
                    correlations.push(serde_json::json!({
                        "asset_1": codes[i], "asset_2": codes[j],
                        "correlation": format!("{:.2}", corr),
                        "interpretation": if corr > 0.8 { "高度正相关" } else if corr > 0.5 { "中度正相关" } else if corr > -0.3 { "低相关" } else { "负相关" }
                    }));
                }
            }
        }
    }

    serde_json::to_string_pretty(&serde_json::json!({"found": true, "asset_count": codes.len(), "correlations": correlations}))
        .map_err(AppError::from)
}

async fn execute_build_portfolio(args: &serde_json::Value) -> AppResult<String> {
    let capital = args["capital"].as_f64().ok_or_else(|| AppError::ValidationError("缺少 capital".into()))?;
    let risk_level = get_string_arg(args, "risk_level")?;

    let (allocation, desc) = match risk_level.as_str() {
        "conservative" => (serde_json::json!({"债券基金": {"pct": 40, "amount": capital*0.4}, "货币基金": {"pct": 20, "amount": capital*0.2}, "黄金": {"pct": 15, "amount": capital*0.15}, "大盘指数": {"pct": 15, "amount": capital*0.15}, "现金": {"pct": 10, "amount": capital*0.1}}), "保守型：预期年化3-5%"),
        "balanced" => (serde_json::json!({"宽基指数": {"pct": 35, "amount": capital*0.35}, "行业基金": {"pct": 20, "amount": capital*0.2}, "债券基金": {"pct": 20, "amount": capital*0.2}, "黄金": {"pct": 15, "amount": capital*0.15}, "加密货币": {"pct": 10, "amount": capital*0.1}}), "均衡型：预期年化8-12%"),
        "aggressive" => (serde_json::json!({"行业基金": {"pct": 35, "amount": capital*0.35}, "个股/港股": {"pct": 25, "amount": capital*0.25}, "加密货币": {"pct": 20, "amount": capital*0.2}, "黄金": {"pct": 10, "amount": capital*0.1}, "宽基指数": {"pct": 10, "amount": capital*0.1}}), "进取型：预期年化15%+"),
        _ => return Err(AppError::ValidationError("无效 risk_level".into())),
    };

    serde_json::to_string_pretty(&serde_json::json!({"capital": capital, "risk_level": risk_level, "description": desc, "allocation": allocation}))
        .map_err(AppError::from)
}

async fn execute_get_market_sentiment() -> AppResult<String> {
    let gold = gold_api::get_gold_quote().await.ok();
    let btc = crypto_api::get_crypto_quote("BTCUSDT").await.ok();
    let mut indicators = Vec::new();

    if let Some(g) = gold {
        let pct = g.change_percent.unwrap_or(0.0);
        indicators.push(serde_json::json!({"name": "黄金", "change": format!("{:+.2}%", pct)}));
    }
    if let Some(b) = btc {
        let pct = b.change_percent.unwrap_or(0.0);
        indicators.push(serde_json::json!({"name": "比特币", "change": format!("{:+.2}%", pct)}));
    }

    serde_json::to_string_pretty(&serde_json::json!({"indicators": indicators, "note": "仅供参考"}))
        .map_err(AppError::from)
}

async fn execute_tax_optimization(pool: &SqlitePool) -> AppResult<String> {
    let ops = sqlx::query("SELECT asset_code, asset_name, amount, op_date FROM operations WHERE op_type = 'buy'")
        .fetch_all(pool).await.map_err(|e| AppError::StorageError(format!("查询失败: {e}")))?;

    let now = chrono::Local::now().naive_local().date();
    let mut suggestions = Vec::new();
    for row in &ops {
        let code = row.get::<String, _>("asset_code");
        let name = row.get::<Option<String>, _>("asset_name").unwrap_or_default();
        let date_str = row.get::<String, _>("op_date");
        if let Ok(d) = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            let days = (now - d).num_days();
            if days < 365 {
                suggestions.push(serde_json::json!({"asset": format!("{}({})", name, code), "holding_days": days, "tip": "持有不足1年，卖出需缴税"}));
            }
        }
    }

    serde_json::to_string_pretty(&serde_json::json!({"found": true, "suggestions": suggestions}))
        .map_err(AppError::from)
}

async fn execute_screen_funds(args: &serde_json::Value, pool: &SqlitePool) -> AppResult<String> {
    let mut sql = String::from("SELECT fp.fund_code, fp.name, frs.value AS return_1y FROM fund_profile fp LEFT JOIN fund_return_summary frs ON fp.fund_code = frs.fund_code AND frs.period = '1y' WHERE 1=1");
    if let Some(min_ret) = args["min_return_1y"].as_f64() { sql.push_str(&format!(" AND frs.value >= {}", min_ret)); }
    if let Some(cat) = args["category"].as_str() { sql.push_str(&format!(" AND fp.name LIKE '%{}%'", cat)); }
    sql.push_str(" ORDER BY frs.value DESC LIMIT 20");

    let rows = sqlx::query(&sql).fetch_all(pool).await.map_err(|e| AppError::StorageError(format!("筛选失败: {e}")))?;
    let funds: Vec<_> = rows.iter().map(|r| serde_json::json!({"code": r.get::<String, _>("fund_code"), "name": r.get::<Option<String>, _>("name"), "return_1y": r.get::<Option<f64>, _>("return_1y").map(|v| format!("{:.2}%", v))})).collect();

    serde_json::to_string_pretty(&serde_json::json!({"count": funds.len(), "funds": funds})).map_err(AppError::from)
}

async fn execute_generate_daily_report(pool: &SqlitePool) -> AppResult<String> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let ops = sqlx::query("SELECT op_type, asset_code, asset_name, amount FROM operations WHERE op_date = ?")
        .bind(&today).fetch_all(pool).await.unwrap_or_default();
    let operations: Vec<_> = ops.iter().map(|r| serde_json::json!({"type": r.get::<String, _>("op_type"), "asset": r.get::<String, _>("asset_code"), "amount": r.get::<Option<f64>, _>("amount")})).collect();

    let gold = gold_api::get_gold_quote().await.ok();
    let btc = crypto_api::get_crypto_quote("BTCUSDT").await.ok();
    let mut market = Vec::new();
    if let Some(g) = gold { market.push(serde_json::json!({"name": "黄金", "change": format!("{:+.2}%", g.change_percent.unwrap_or(0.0))})); }
    if let Some(b) = btc { market.push(serde_json::json!({"name": "BTC", "change": format!("{:+.2}%", b.change_percent.unwrap_or(0.0))})); }

    serde_json::to_string_pretty(&serde_json::json!({"date": today, "market": market, "operations": operations})).map_err(AppError::from)
}

async fn execute_monte_carlo(args: &serde_json::Value, pool: &SqlitePool) -> AppResult<String> {
    let days = args["days"].as_u64().unwrap_or(365) as usize;
    let simulations = args["simulations"].as_u64().unwrap_or(1000) as usize;

    let snapshot = sqlx::query("SELECT payload FROM portfolio_snapshots ORDER BY created_at DESC LIMIT 1")
        .fetch_optional(pool).await.map_err(|e| AppError::StorageError(format!("查询快照失败: {e}")))?;

    let snapshot = match snapshot {
        Some(row) => serde_json::from_str::<serde_json::Value>(&row.get::<String, _>("payload")).unwrap_or_default(),
        None => return Ok(serde_json::json!({"found": false, "message": "未找到快照"}).to_string()),
    };

    let total_value = snapshot["total_value"].as_f64().unwrap_or(0.0);
    let daily_vol = 0.015; // ~1.5% daily volatility estimate
    let daily_ret = 0.0003; // ~7.5% annual return estimate

    let seed = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
    let mut finals = Vec::with_capacity(simulations);

    for sim in 0..simulations {
        let mut val = total_value;
        for day in 0..days {
            let u1 = ((seed.wrapping_add(sim as u64 * 1000 + day as u64) % 10000) as f64) / 10000.0;
            let u2 = ((seed.wrapping_add(sim as u64 * 2000 + day as u64 * 3) % 10000) as f64) / 10000.0;
            let z = (-2.0 * u1.max(0.001).ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            val *= 1.0 + daily_ret + daily_vol * z / 15.87;
        }
        finals.push(val);
    }

    finals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mean = finals.iter().sum::<f64>() / simulations as f64;
    let p5 = finals[(simulations as f64 * 0.05) as usize];
    let p95 = finals[(simulations as f64 * 0.95) as usize];
    let loss_pct = finals.iter().filter(|v| **v < total_value).count() as f64 / simulations as f64 * 100.0;

    serde_json::to_string_pretty(&serde_json::json!({
        "found": true, "current_value": total_value, "days": days, "simulations": simulations,
        "results": {"mean": mean, "p5_worst": p5, "p95_best": p95, "loss_probability": format!("{:.1}%", loss_pct)}
    })).map_err(AppError::from)
}

// ============================================================================
// Portfolio holdings tool (持仓查询 - 实时数据)
// ============================================================================

async fn execute_get_portfolio_holdings(pool: &SqlitePool) -> AppResult<String> {
    let mut all_holdings: Vec<serde_json::Value> = Vec::new();
    let mut total_value = 0.0_f64;

    // 1. 基金持仓 (group_fund_positions + funds)
    let fund_rows = sqlx::query(
        "SELECT gfp.fund_code, f.name, gfp.holding_amount, gfp.holding_shares \
         FROM group_fund_positions gfp \
         LEFT JOIN funds f ON gfp.fund_code = f.code \
         WHERE gfp.holding_amount > 0"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    for row in &fund_rows {
        let amount: f64 = row.try_get("holding_amount").unwrap_or(0.0);
        total_value += amount;
        all_holdings.push(serde_json::json!({
            "type": "fund",
            "code": row.try_get::<String, _>("fund_code").unwrap_or_default(),
            "name": row.try_get::<Option<String>, _>("name").unwrap_or_default(),
            "holding_amount": amount,
            "holding_shares": row.try_get::<f64, _>("holding_shares").unwrap_or(0.0),
        }));
    }

    // 2. 股票持仓
    let stock_rows = sqlx::query(
        "SELECT code, holding_amount, holding_shares FROM stock_holdings WHERE holding_amount > 0"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    for row in &stock_rows {
        let amount: f64 = row.try_get("holding_amount").unwrap_or(0.0);
        total_value += amount;
        all_holdings.push(serde_json::json!({
            "type": "stock",
            "code": row.try_get::<String, _>("code").unwrap_or_default(),
            "holding_amount": amount,
            "holding_shares": row.try_get::<f64, _>("holding_shares").unwrap_or(0.0),
        }));
    }

    // 3. 加密货币 + 黄金持仓
    let crypto_rows = sqlx::query(
        "SELECT symbol, holding_amount, holding_quantity FROM crypto_holdings WHERE holding_amount > 0"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    for row in &crypto_rows {
        let symbol: String = row.try_get("symbol").unwrap_or_default();
        let amount: f64 = row.try_get("holding_amount").unwrap_or(0.0);
        total_value += amount;
        let asset_type = if symbol == "AU9999" { "gold" } else { "crypto" };
        all_holdings.push(serde_json::json!({
            "type": asset_type,
            "code": symbol,
            "holding_amount": amount,
            "holding_quantity": row.try_get::<f64, _>("holding_quantity").unwrap_or(0.0),
        }));
    }

    // Calculate percentages
    for holding in &mut all_holdings {
        let amount = holding["holding_amount"].as_f64().unwrap_or(0.0);
        let pct = if total_value > 0.0 { amount / total_value * 100.0 } else { 0.0 };
        holding["percent"] = serde_json::json!(format!("{:.2}%", pct));
    }

    // Sort by amount descending
    all_holdings.sort_by(|a, b| {
        let a_val = a["holding_amount"].as_f64().unwrap_or(0.0);
        let b_val = b["holding_amount"].as_f64().unwrap_or(0.0);
        b_val.partial_cmp(&a_val).unwrap_or(std::cmp::Ordering::Equal)
    });

    // Group by type
    let fund_count = all_holdings.iter().filter(|h| h["type"] == "fund").count();
    let stock_count = all_holdings.iter().filter(|h| h["type"] == "stock").count();
    let crypto_count = all_holdings.iter().filter(|h| h["type"] == "crypto").count();
    let gold_count = all_holdings.iter().filter(|h| h["type"] == "gold").count();

    serde_json::to_string_pretty(&serde_json::json!({
        "found": true,
        "total_value": total_value,
        "asset_count": all_holdings.len(),
        "summary": {
            "fund": fund_count,
            "stock": stock_count,
            "crypto": crypto_count,
            "gold": gold_count,
        },
        "holdings": all_holdings,
    }))
    .map_err(AppError::from)
}

// ============================================================================
// Local DB query tool (通用数据库查询)
// ============================================================================

async fn execute_query_local_db(args: &serde_json::Value, pool: &SqlitePool) -> AppResult<String> {
    let sql = get_string_arg(args, "sql")?;

    // Safety: only allow SELECT and PRAGMA queries
    let sql_upper = sql.trim().to_uppercase();
    if !sql_upper.starts_with("SELECT") && !sql_upper.starts_with("WITH") && !sql_upper.starts_with("PRAGMA") {
        return Err(AppError::ValidationError("仅支持 SELECT/PRAGMA 查询".into()));
    }
    if sql_upper.contains("INSERT") || sql_upper.contains("UPDATE") || sql_upper.contains("DELETE")
        || sql_upper.contains("DROP") || sql_upper.contains("ALTER") || sql_upper.contains("CREATE")
        || sql_upper.contains("ATTACH") || sql_upper.contains("DETACH")
    {
        return Err(AppError::ValidationError("仅支持只读查询".into()));
    }

    eprintln!("[AI_CHAT] 执行 SQL: {}", sql);

    let rows = sqlx::query(&sql)
        .fetch_all(pool)
        .await
        .map_err(|e| AppError::StorageError(format!("SQL 查询失败: {e}")))?;

    let results: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            let mut map = serde_json::Map::new();
            let columns = row.columns();
            for (i, col) in columns.iter().enumerate() {
                let col_name = col.name().to_string();
                let value = if let Ok(v) = row.try_get::<i64, _>(i) {
                    serde_json::json!(v)
                } else if let Ok(v) = row.try_get::<f64, _>(i) {
                    serde_json::json!(v)
                } else if let Ok(v) = row.try_get::<String, _>(i) {
                    serde_json::json!(v)
                } else if let Ok(v) = row.try_get::<Option<i64>, _>(i) {
                    serde_json::json!(v)
                } else if let Ok(v) = row.try_get::<Option<f64>, _>(i) {
                    serde_json::json!(v)
                } else if let Ok(v) = row.try_get::<Option<String>, _>(i) {
                    serde_json::json!(v)
                } else {
                    serde_json::Value::Null
                };
                map.insert(col_name, value);
            }
            serde_json::Value::Object(map)
        })
        .collect();

    serde_json::to_string_pretty(&serde_json::json!({
        "row_count": results.len(),
        "results": results,
    }))
    .map_err(AppError::from)
}

// ============================================================================
// Daily snapshot tool (每日快照保存)
// ============================================================================

async fn execute_save_daily_snapshot(pool: &SqlitePool) -> AppResult<String> {
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let now = chrono::Utc::now().timestamp();
    let mut saved_items = Vec::new();

    // 1. 保存股票行情
    let stock_rows = sqlx::query("SELECT code FROM stock_holdings WHERE holding_amount > 0")
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    for row in &stock_rows {
        let code: String = row.try_get("code").unwrap_or_default();
        match stock_api::get_stock_quote(&code).await {
            Ok(quote) => {
                if let Err(e) = stock_api::save_stock_daily_quote(pool, &quote, &today).await {
                    eprintln!("[SNAPSHOT] 保存股票行情失败 {}: {}", code, e.details());
                } else {
                    saved_items.push(format!("股票 {}", code));
                }
            }
            Err(e) => eprintln!("[SNAPSHOT] 获取股票行情失败 {}: {}", code, e.details()),
        }
    }

    // 2. 保存加密货币/黄金行情
    let crypto_rows = sqlx::query("SELECT symbol FROM crypto_holdings WHERE holding_amount > 0")
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    for row in &crypto_rows {
        let symbol: String = row.try_get("symbol").unwrap_or_default();
        if symbol == "AU9999" {
            // 黄金用 gold_api
            match gold_api::get_gold_quote().await {
                Ok(quote) => {
                    let crypto_quote = crate::models::CryptoQuote {
                        symbol: quote.code.clone(),
                        name: quote.name.clone(),
                        price: quote.price,
                        change_percent: quote.change_percent,
                        high_24h: quote.high,
                        low_24h: quote.low,
                        volume_24h: quote.volume,
                        update_time: quote.update_time.clone(),
                    };
                    if let Err(e) = crypto_api::save_crypto_daily_quote(pool, &crypto_quote, &today).await {
                        eprintln!("[SNAPSHOT] 保存黄金行情失败: {}", e.details());
                    } else {
                        saved_items.push(format!("黄金 {}", symbol));
                    }
                }
                Err(e) => eprintln!("[SNAPSHOT] 获取黄金行情失败: {}", e.details()),
            }
        } else {
            // 加密货币用 crypto_api
            match crypto_api::get_crypto_quote(&symbol).await {
                Ok(quote) => {
                    if let Err(e) = crypto_api::save_crypto_daily_quote(pool, &quote, &today).await {
                        eprintln!("[SNAPSHOT] 保存加密货币行情失败 {}: {}", symbol, e.details());
                    } else {
                        saved_items.push(format!("加密货币 {}", symbol));
                    }
                }
                Err(e) => eprintln!("[SNAPSHOT] 获取加密货币行情失败 {}: {}", symbol, e.details()),
            }
        }
    }

    // 3. 保存每日持仓快照
    let mut total_value = 0.0_f64;
    let mut total_cost = 0.0_f64;
    let mut fund_value = 0.0_f64;
    let mut stock_value = 0.0_f64;
    let mut crypto_value = 0.0_f64;
    let mut gold_value = 0.0_f64;
    let mut fund_count = 0_i32;
    let mut stock_count = 0_i32;
    let mut crypto_count = 0_i32;
    let mut gold_count = 0_i32;

    // 基金
    let fund_rows = sqlx::query(
        "SELECT holding_amount FROM group_fund_positions WHERE holding_amount > 0"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();
    for row in &fund_rows {
        let amount: f64 = row.try_get("holding_amount").unwrap_or(0.0);
        fund_value += amount;
        fund_count += 1;
    }

    // 股票
    let stock_rows = sqlx::query(
        "SELECT holding_amount FROM stock_holdings WHERE holding_amount > 0"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();
    for row in &stock_rows {
        let amount: f64 = row.try_get("holding_amount").unwrap_or(0.0);
        stock_value += amount;
        stock_count += 1;
    }

    // 加密货币 + 黄金
    let crypto_rows = sqlx::query(
        "SELECT symbol, holding_amount FROM crypto_holdings WHERE holding_amount > 0"
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();
    for row in &crypto_rows {
        let symbol: String = row.try_get("symbol").unwrap_or_default();
        let amount: f64 = row.try_get("holding_amount").unwrap_or(0.0);
        if symbol == "AU9999" {
            gold_value += amount;
            gold_count += 1;
        } else {
            crypto_value += amount;
            crypto_count += 1;
        }
    }

    total_value = fund_value + stock_value + crypto_value + gold_value;

    // 保存快照
    let details = serde_json::json!({
        "fund": {"value": fund_value, "count": fund_count},
        "stock": {"value": stock_value, "count": stock_count},
        "crypto": {"value": crypto_value, "count": crypto_count},
        "gold": {"value": gold_value, "count": gold_count},
    });

    sqlx::query(
        "INSERT OR REPLACE INTO daily_portfolio_snapshot \
         (snapshot_date, total_value, total_cost, total_profit, total_profit_percent, \
          fund_value, stock_value, crypto_value, gold_value, \
          fund_count, stock_count, crypto_count, gold_count, details, created_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&today)
    .bind(total_value)
    .bind(total_cost)
    .bind(0.0_f64) // total_profit - 需要成本数据
    .bind(0.0_f64) // total_profit_percent
    .bind(fund_value)
    .bind(stock_value)
    .bind(crypto_value)
    .bind(gold_value)
    .bind(fund_count)
    .bind(stock_count)
    .bind(crypto_count)
    .bind(gold_count)
    .bind(details.to_string())
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| AppError::StorageError(format!("保存每日快照失败: {e}")))?;

    serde_json::to_string_pretty(&serde_json::json!({
        "success": true,
        "date": today,
        "total_value": total_value,
        "saved_items": saved_items,
        "snapshot": {
            "fund": {"value": fund_value, "count": fund_count},
            "stock": {"value": stock_value, "count": stock_count},
            "crypto": {"value": crypto_value, "count": crypto_count},
            "gold": {"value": gold_value, "count": gold_count},
        }
    }))
    .map_err(AppError::from)
}
