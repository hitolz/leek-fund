use crate::errors::{AppError, AppResult};
use crate::models::CryptoQuote;
use reqwest::Client;
use std::time::Duration;

/// 常用加密货币名称映射
const CRYPTO_NAMES: &[(&str, &str)] = &[
    ("BTCUSDT", "比特币"),
    ("ETHUSDT", "以太坊"),
    ("BNBUSDT", "币安币"),
    ("SOLUSDT", "Solana"),
    ("XRPUSDT", "瑞波币"),
    ("DOGEUSDT", "狗狗币"),
    ("ADAUSDT", "艾达币"),
    ("AVAXUSDT", "雪崩"),
    ("DOTUSDT", "波卡"),
    ("MATICUSDT", "Polygon"),
    ("LINKUSDT", "Chainlink"),
    ("UNIUSDT", "Uniswap"),
    ("LTCUSDT", "莱特币"),
    ("ATOMUSDT", "Cosmos"),
    ("NEARUSDT", "NEAR"),
];

/// 构建 HTTP 客户端
fn build_http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(15))
        .user_agent("LeekFund/1.0.0")
        .build()
        .expect("Failed to build HTTP client")
}

/// 获取加密货币名称
fn get_crypto_name(symbol: &str) -> String {
    CRYPTO_NAMES
        .iter()
        .find(|(s, _)| *s == symbol)
        .map(|(_, name)| name.to_string())
        .unwrap_or_else(|| symbol.replace("USDT", ""))
}

/// 获取单个加密货币行情
pub async fn get_crypto_quote(symbol: &str) -> AppResult<CryptoQuote> {
    let symbol = symbol.to_uppercase();
    if !symbol.ends_with("USDT") {
        return Err(AppError::ValidationError(
            "仅支持 USDT 交易对，如 BTCUSDT".to_string(),
        ));
    }

    let client = build_http_client();
    let url = format!(
        "https://data-api.binance.vision/api/v3/ticker/24hr?symbol={}",
        symbol
    );

    eprintln!("[crypto_api] 请求行情: {}", url);

    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        return Err(AppError::NotFound(symbol));
    }

    let json: serde_json::Value = resp.json().await?;

    eprintln!("[crypto_api] 响应: {}", json);

    parse_binance_response(&symbol, &json)
}

/// 批量获取加密货币行情
pub async fn get_crypto_quotes(symbols: &[String]) -> AppResult<Vec<CryptoQuote>> {
    let client = build_http_client();

    // Binance 支持批量查询，但需要 JSON 数组格式
    let symbols_upper: Vec<String> = symbols.iter().map(|s| s.to_uppercase()).collect();

    // 逐个请求（简单实现）
    let mut results = Vec::new();
    for symbol in &symbols_upper {
        match get_crypto_quote_with_client(&client, symbol).await {
            Ok(quote) => results.push(quote),
            Err(e) => {
                eprintln!("[crypto_api] 获取 {} 失败: {}", symbol, e.details());
            }
        }
    }

    Ok(results)
}

/// 使用已有客户端获取行情
async fn get_crypto_quote_with_client(client: &Client, symbol: &str) -> AppResult<CryptoQuote> {
    let url = format!(
        "https://data-api.binance.vision/api/v3/ticker/24hr?symbol={}",
        symbol
    );

    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        return Err(AppError::NotFound(symbol.to_string()));
    }

    let json: serde_json::Value = resp.json().await?;
    parse_binance_response(symbol, &json)
}

/// 解析 Binance API 响应
fn parse_binance_response(symbol: &str, json: &serde_json::Value) -> AppResult<CryptoQuote> {
    let name = get_crypto_name(symbol);

    let price = json["lastPrice"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok());

    let change_percent = json["priceChangePercent"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok());

    let high_24h = json["highPrice"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok());

    let low_24h = json["lowPrice"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok());

    let volume_24h = json["volume"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok());

    Ok(CryptoQuote {
        symbol: symbol.to_string(),
        name,
        price,
        change_percent,
        high_24h,
        low_24h,
        volume_24h,
        update_time: Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()),
    })
}

/// 获取常用加密货币列表
pub fn get_popular_cryptos() -> Vec<(&'static str, &'static str)> {
    CRYPTO_NAMES.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_crypto_name() {
        assert_eq!(get_crypto_name("BTCUSDT"), "比特币");
        assert_eq!(get_crypto_name("ETHUSDT"), "以太坊");
        assert_eq!(get_crypto_name("UNKNOWNUSDT"), "UNKNOWN");
    }

    #[test]
    fn test_parse_binance_response() {
        let json = serde_json::json!({
            "lastPrice": "50000.00",
            "priceChangePercent": "2.50",
            "highPrice": "51000.00",
            "lowPrice": "49000.00",
            "volume": "1234.56"
        });

        let quote = parse_binance_response("BTCUSDT", &json).unwrap();
        assert_eq!(quote.name, "比特币");
        assert_eq!(quote.price, Some(50000.0));
        assert_eq!(quote.change_percent, Some(2.5));
    }
}
