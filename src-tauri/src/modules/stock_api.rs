use crate::errors::{AppError, AppResult};
use crate::models::StockQuote;
use reqwest::Client;
use std::time::Duration;

/// 构建 HTTP 客户端
fn build_http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("LeekFund/1.0.0")
        .build()
        .expect("Failed to build HTTP client")
}

/// 股票搜索结果
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StockSearchResult {
    pub code: String,
    pub name: String,
    pub market: String,
}

/// 搜索股票（腾讯接口）
pub async fn search_stock(keyword: &str) -> AppResult<Vec<StockSearchResult>> {
    if keyword.is_empty() {
        return Ok(vec![]);
    }

    let client = build_http_client();
    let url = format!(
        "https://proxy.finance.qq.com/ifzqgtimg/appstock/smartbox/search/get?q={}",
        keyword
    );

    eprintln!("[stock_api] 搜索股票: keyword={}", keyword);

    let resp = client.get(&url).send().await?;
    if !resp.status().is_success() {
        return Err(AppError::NetworkError(format!("搜索失败: {}", resp.status())));
    }

    let json: serde_json::Value = resp.json().await?;
    let mut results = Vec::new();

    if let Some(stocks) = json["data"]["stock"].as_array() {
        for item in stocks {
            if let Some(arr) = item.as_array() {
                if arr.len() >= 3 {
                    let market = arr[0].as_str().unwrap_or("").to_string();
                    let code = arr[1].as_str().unwrap_or("").to_string();
                    let name = arr[2].as_str().unwrap_or("").to_string();

                    // 只返回 A股和港股
                    if market == "sh" || market == "sz" || market == "hk" {
                        results.push(StockSearchResult {
                            code: format!("{}{}", market, code),
                            name,
                            market,
                        });
                    }
                }
            }
        }
    }

    eprintln!("[stock_api] 搜索结果: {} 条", results.len());
    Ok(results)
}

/// 获取股票行情
pub async fn get_stock_quote(code: &str) -> AppResult<StockQuote> {
    let client = build_http_client();

    // 判断市场类型
    if code.starts_with("hk") {
        // 港股 - 使用腾讯接口
        get_hk_stock_quote(&client, code).await
    } else if code.starts_with("sh") || code.starts_with("sz") {
        // A股 - 使用新浪接口
        get_a_stock_quote(&client, code).await
    } else {
        Err(AppError::ValidationError(format!("不支持的股票代码格式: {}", code)))
    }
}

/// 获取A股行情（新浪接口）
async fn get_a_stock_quote(client: &Client, code: &str) -> AppResult<StockQuote> {
    let url = format!("https://hq.sinajs.cn/list={}", code);

    eprintln!("[stock_api] 请求A股行情: {}", url);

    let resp = client
        .get(&url)
        .header("Referer", "http://finance.sina.com.cn/")
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(AppError::NotFound(code.to_string()));
    }

    // 新浪接口返回 GBK 编码
    let bytes = resp.bytes().await?;
    let (text, _, _) = encoding_rs::GBK.decode(&bytes);
    let text = text.as_ref();

    eprintln!("[stock_api] A股响应: {}", text);

    parse_sina_stock_response(code, text)
}

/// 解析新浪股票响应
fn parse_sina_stock_response(code: &str, text: &str) -> AppResult<StockQuote> {
    // 格式: var hq_str_sh600519="贵州茅台,1800.00,1795.00,1810.00,1815.00,1790.00,1810.00,1811.00,12345678,22345678900,...";
    let line = text.trim();
    let data_start = line.find('"').ok_or_else(|| AppError::ParseError("无效的股票数据格式".to_string()))? + 1;
    let data_end = line.rfind('"').ok_or_else(|| AppError::ParseError("无效的股票数据格式".to_string()))?;
    let data = &line[data_start..data_end];

    let fields: Vec<&str> = data.split(',').collect();
    if fields.len() < 32 {
        return Err(AppError::ParseError("股票数据字段不足".to_string()));
    }

    let name = fields[0].to_string();
    let open = fields[1].parse::<f64>().ok();
    let yesterday_close = fields[2].parse::<f64>().ok();
    let price = fields[3].parse::<f64>().ok();
    let high = fields[4].parse::<f64>().ok();
    let low = fields[5].parse::<f64>().ok();
    let volume = fields[8].parse::<f64>().ok();

    // 计算涨跌幅和涨跌额
    let (change_amount, change_percent) = match (price, yesterday_close) {
        (Some(p), Some(yc)) if yc > 0.0 => {
            let amount = p - yc;
            let percent = (amount / yc) * 100.0;
            (Some(amount), Some(percent))
        }
        _ => (None, None),
    };

    let update_time = if fields.len() > 30 {
        Some(format!("{} {}", fields[30], fields[31]))
    } else {
        None
    };

    Ok(StockQuote {
        code: code.to_string(),
        name,
        price,
        change_percent,
        change_amount,
        open,
        high,
        low,
        yesterday_close,
        volume,
        update_time,
    })
}

/// 获取港股行情（腾讯接口）
async fn get_hk_stock_quote(client: &Client, code: &str) -> AppResult<StockQuote> {
    let url = format!("https://qt.gtimg.cn/q={}", code);

    eprintln!("[stock_api] 请求港股行情: {}", url);

    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        return Err(AppError::NotFound(code.to_string()));
    }

    // 腾讯接口返回 GBK 编码
    let bytes = resp.bytes().await?;
    let (text, _, _) = encoding_rs::GBK.decode(&bytes);
    let text = text.as_ref();

    eprintln!("[stock_api] 港股响应: {}", text);

    parse_tencent_hk_response(code, text)
}

/// 解析腾讯港股响应
fn parse_tencent_hk_response(code: &str, text: &str) -> AppResult<StockQuote> {
    // 格式: v_r_hk00700="100~腾讯控股~00700~400.000~405.000~400.000~...";
    let line = text.trim();
    let data_start = line.find('"').ok_or_else(|| AppError::ParseError("无效的港股数据格式".to_string()))? + 1;
    let data_end = line.rfind('"').ok_or_else(|| AppError::ParseError("无效的港股数据格式".to_string()))?;
    let data = &line[data_start..data_end];

    let fields: Vec<&str> = data.split('~').collect();
    if fields.len() < 45 {
        return Err(AppError::ParseError("港股数据字段不足".to_string()));
    }

    let name = fields[1].to_string();
    let price = fields[3].parse::<f64>().ok();
    let yesterday_close = fields[4].parse::<f64>().ok();
    let open = fields[5].parse::<f64>().ok();
    let high = fields[33].parse::<f64>().ok();
    let low = fields[34].parse::<f64>().ok();
    let volume = fields[36].parse::<f64>().ok();
    let change_amount = fields[31].parse::<f64>().ok();
    let change_percent = fields[32].parse::<f64>().ok();

    let update_time = if fields.len() > 30 {
        Some(fields[30].to_string())
    } else {
        None
    };

    Ok(StockQuote {
        code: code.to_string(),
        name,
        price,
        change_percent,
        change_amount,
        open,
        high,
        low,
        yesterday_close,
        volume,
        update_time,
    })
}

/// 保存股票每日行情到数据库
pub async fn save_stock_daily_quote(
    pool: &sqlx::SqlitePool,
    quote: &StockQuote,
    date: &str,
) -> crate::errors::AppResult<()> {
    let now = chrono::Utc::now().timestamp();
    sqlx::query(
        "INSERT OR REPLACE INTO stock_daily_quotes \
         (code, quote_date, price, change_percent, change_amount, open, high, low, yesterday_close, volume, created_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&quote.code)
    .bind(date)
    .bind(quote.price)
    .bind(quote.change_percent)
    .bind(quote.change_amount)
    .bind(quote.open)
    .bind(quote.high)
    .bind(quote.low)
    .bind(quote.yesterday_close)
    .bind(quote.volume)
    .bind(now)
    .execute(pool)
    .await
    .map_err(|e| crate::errors::AppError::StorageError(format!("保存股票行情失败: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sina_response() {
        let text = r#"var hq_str_sh600519="贵州茅台,1800.00,1795.00,1810.00,1815.00,1790.00,1810.00,1811.00,12345678,22345678900,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2026-07-22,15:00:00,00";"#;
        let quote = parse_sina_stock_response("sh600519", text).unwrap();
        assert_eq!(quote.name, "贵州茅台");
        assert_eq!(quote.price, Some(1810.0));
        assert_eq!(quote.open, Some(1800.0));
    }
}
