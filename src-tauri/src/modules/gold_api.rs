use crate::errors::{AppError, AppResult};
use reqwest::Client;
use std::time::Duration;

/// 黄金行情信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GoldQuote {
    /// 代码
    pub code: String,
    /// 名称
    pub name: String,
    /// 最新价（元/克）
    pub price: Option<f64>,
    /// 涨跌幅(%)
    pub change_percent: Option<f64>,
    /// 涨跌额（元/克）
    pub change_amount: Option<f64>,
    /// 开盘价
    pub open: Option<f64>,
    /// 最高价
    pub high: Option<f64>,
    /// 最低价
    pub low: Option<f64>,
    /// 昨收价
    pub yesterday_close: Option<f64>,
    /// 成交量（手）
    pub volume: Option<f64>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 构建 HTTP 客户端
fn build_http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("LeekFund/1.0.0")
        .build()
        .expect("Failed to build HTTP client")
}

/// 获取黄金 AU9999 行情（东方财富 API）
pub async fn get_gold_quote() -> AppResult<GoldQuote> {
    let client = build_http_client();
    let url = "http://push2.eastmoney.com/api/qt/stock/get?secid=118.AU9999&fields=f43,f44,f45,f46,f47,f48,f57,f58,f60,f170";

    eprintln!("[gold_api] 请求黄金行情: {}", url);

    let resp = client.get(url).send().await?;

    if !resp.status().is_success() {
        return Err(AppError::NetworkError(format!("请求失败: {}", resp.status())));
    }

    let json: serde_json::Value = resp.json().await?;

    eprintln!("[gold_api] 响应: {}", json);

    parse_gold_response(&json)
}

/// 解析黄金行情响应
/// 东方财富 API 返回的价格单位是分，需要除以 100 转换为元
fn parse_gold_response(json: &serde_json::Value) -> AppResult<GoldQuote> {
    let data = &json["data"];

    // 价格字段都是以分为单位，需要除以 100
    let price = data["f43"].as_i64().map(|v| v as f64 / 100.0);
    let high = data["f44"].as_i64().map(|v| v as f64 / 100.0);
    let low = data["f45"].as_i64().map(|v| v as f64 / 100.0);
    let open = data["f46"].as_i64().map(|v| v as f64 / 100.0);
    let yesterday_close = data["f60"].as_i64().map(|v| v as f64 / 100.0);

    // 涨跌幅，需要除以 100
    let change_percent = data["f170"].as_i64().map(|v| v as f64 / 100.0);

    // 涨跌额 = 最新价 - 昨收价
    let change_amount = match (price, yesterday_close) {
        (Some(p), Some(yc)) => Some(p - yc),
        _ => None,
    };

    let volume = data["f47"].as_f64();

    let code = data["f57"].as_str().unwrap_or("AU9999").to_string();
    let name = data["f58"].as_str().unwrap_or("黄金9999").to_string();

    Ok(GoldQuote {
        code,
        name,
        price,
        change_percent,
        change_amount,
        open,
        high,
        low,
        yesterday_close,
        volume,
        update_time: Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
    })
}

/// 黄金持仓标识
const GOLD_HOLDING_KEY: &str = "AU9999";

/// 获取黄金持仓信息
pub async fn get_gold_holding(
    pool: &sqlx::SqlitePool,
) -> AppResult<Option<crate::modules::asset_position::CryptoHolding>> {
    crate::modules::asset_position::get_crypto_holding(pool, GOLD_HOLDING_KEY).await
}

/// 设置黄金持仓
pub async fn set_gold_holding(
    pool: &sqlx::SqlitePool,
    holding_amount: f64,
    holding_quantity: f64,
) -> AppResult<crate::modules::asset_position::CryptoHolding> {
    crate::modules::asset_position::set_crypto_holding(pool, GOLD_HOLDING_KEY, holding_amount, holding_quantity).await
}

/// 清空黄金持仓
pub async fn clear_gold_holding(pool: &sqlx::SqlitePool) -> AppResult<()> {
    crate::modules::asset_position::clear_crypto_holding(pool, GOLD_HOLDING_KEY).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gold_response() {
        let json = serde_json::json!({
            "rc": 0,
            "data": {
                "f43": 89830,
                "f44": 90800,
                "f45": 89700,
                "f46": 90000,
                "f47": 67708,
                "f48": 610554528.0,
                "f57": "AU9999",
                "f58": "黄金9999",
                "f60": 89900,
                "f170": -8
            }
        });

        let quote = parse_gold_response(&json).unwrap();
        assert_eq!(quote.code, "AU9999");
        assert_eq!(quote.name, "黄金9999");
        assert!((quote.price.unwrap() - 898.30).abs() < 0.01);
        assert!((quote.high.unwrap() - 908.00).abs() < 0.01);
        assert!((quote.low.unwrap() - 897.00).abs() < 0.01);
        assert!((quote.open.unwrap() - 900.00).abs() < 0.01);
        assert!((quote.yesterday_close.unwrap() - 899.00).abs() < 0.01);
        assert!((quote.change_percent.unwrap() - (-0.08)).abs() < 0.01);
        assert!((quote.change_amount.unwrap() - (-0.70)).abs() < 0.01);
    }
}
