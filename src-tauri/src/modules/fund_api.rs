use crate::errors::{AppError, AppResult};
use crate::models::{FundDetail, FundInfo, FundSummary, FundTrend, TrendPoint};
use reqwest::Client;
use std::time::Duration;
use serde::Deserialize;

/// 构建 HTTP 客户端
fn build_http_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("LeekFund/1.0.0")
        .build()
        .expect("Failed to build HTTP client")
}

/// 天天基金实时估值 API（替代已下线的 fundgz.1234567.com.cn）
const FUND_VALUATION_API: &str = "https://fundcomapi.tiantianfunds.com/mm/newCore/FundValuationLast";
const FUND_VALUATION_FIELDS: &str = "FCODE,SHORTNAME,GSZZL,GZTIME,GSZ,NAV,PDATE";

/// 搜索基金信息
pub async fn search_fund_info(code: &str) -> AppResult<FundInfo> {
    // 验证基金代码格式
    if !FundInfo::validate_code(code) {
        return Err(AppError::ValidationError(
            "基金代码格式错误，请输入6位数字".to_string(),
        ));
    }

    let client = build_http_client();

    // 从天天基金新 API 获取实时估值
    let url = format!(
        "{}?FCODES={}&FIELDS={}",
        FUND_VALUATION_API, code, FUND_VALUATION_FIELDS
    );
    eprintln!("[fund_api] 请求 URL: {}", url);

    let resp = client.get(&url).send().await?;

    if !resp.status().is_success() {
        eprintln!("[fund_api] 响应状态: {}", resp.status());
        return Err(AppError::NotFound(code.to_string()));
    }

    let resp_text = resp.text().await?;
    eprintln!("[fund_api] 响应内容: {}", resp_text);

    let json: serde_json::Value = serde_json::from_str(&resp_text)?;

    if !json["success"].as_bool().unwrap_or(false) {
        return Err(AppError::NotFound(code.to_string()));
    }

    let items = json["data"]
        .as_array()
        .ok_or_else(|| AppError::ParseError("缺少数据".to_string()))?;

    if items.is_empty() {
        return Err(AppError::NotFound(code.to_string()));
    }

    let item = &items[0];

    let fund_code = item["FCODE"]
        .as_str()
        .ok_or_else(|| AppError::ParseError("缺少基金代码".to_string()))?
        .to_string();

    if fund_code != code {
        return Err(AppError::NotFound(code.to_string()));
    }

    let name = item["SHORTNAME"]
        .as_str()
        .unwrap_or(code)
        .to_string();

    // 优先使用实时估值 GSZ，如果没有则使用最新净值 NAV
    let net_value = item["GSZ"]
        .as_f64()
        .or_else(|| item["NAV"].as_str().and_then(|s| s.parse::<f64>().ok()))
        .or_else(|| item["NAV"].as_f64());

    // 优先使用实时涨跌幅 GSZZL
    let change_percent = item["GSZZL"]
        .as_f64()
        .map(|v| format!("{}", v))
        .or_else(|| {
            item["GSZZL"].as_str().map(|s| s.to_string())
        });

    // 优先使用估值时间 GZTIME，如果没有则使用净值日期 PDATE
    let update_time = item["GZTIME"]
        .as_str()
        .filter(|s| !s.is_empty() && *s != "null")
        .or_else(|| item["PDATE"].as_str())
        .map(|s| s.to_string())
        .or_else(|| Some(chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()));

    eprintln!("[fund_api] 解析结果: code={}, name={}, net_value={:?}, change_percent={:?}, update_time={:?}",
        fund_code, name, net_value, change_percent, update_time);

    Ok(FundInfo {
        code: fund_code,
        name,
        net_value,
        change_percent,
        update_time,
    })
}

pub async fn get_fund_summary(code: &str) -> AppResult<FundSummary> {
    let info = search_fund_info(code).await?;
    Ok(FundSummary {
        code: info.code,
        name: info.name,
        daily_change_percent: info.change_percent,
        daily_change_amount: None,
        holding_amount: None,
        update_time: info.update_time,
    })
}

pub async fn get_fund_detail(code: &str) -> AppResult<FundDetail> {
    let info = search_fund_info(code).await?;
    Ok(FundDetail {
        code: info.code,
        name: info.name,
        net_value: info.net_value,
        change_percent: info.change_percent,
        daily_change_amount: None,
        update_time: info.update_time,
        holding_amount: None,
        holding_shares: None,
        cost_price: None,
    })
}

pub async fn get_fund_trend(code: &str) -> AppResult<FundTrend> {
    if !FundInfo::validate_code(code) {
        return Err(AppError::ValidationError(
            "基金代码格式错误，请输入6位数字".to_string(),
        ));
    }

    let client = build_http_client();
    let url = format!("https://fund.eastmoney.com/pingzhongdata/{}.js", code);
    eprintln!("[fund_api] 请求走势数据: {}", url);

    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        eprintln!("[fund_api] 走势请求失败: {}", response.status());
        return Err(AppError::FundTrendUnavailable(code.to_string()));
    }

    let text = response.text().await?;
    eprintln!("[fund_api] 走响响应大小: {} bytes", text.len());

    let mut points = parse_trend_points(&text)?;
    eprintln!("[fund_api] 解析到 {} 个走势点", points.len());

    if points.is_empty() {
        return Err(AppError::FundTrendUnavailable(code.to_string()));
    }

    let window = if points.len() > 30 {
        points = points.split_off(points.len() - 30);
        "最近30个交易日".to_string()
    } else {
        format!("最近{}个交易日", points.len())
    };

    Ok(FundTrend {
        code: code.to_string(),
        window,
        points,
    })
}

/// 获取 pingzhongdata 原始响应
pub async fn fetch_pingzhong_payload(code: &str) -> AppResult<String> {
    if !FundInfo::validate_code(code) {
        return Err(AppError::ValidationError(
            "基金代码格式错误，请输入6位数字".to_string(),
        ));
    }

    let client = build_http_client();
    let url = format!("https://fund.eastmoney.com/pingzhongdata/{}.js", code);
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(AppError::FundDetailUnavailable(code.to_string()));
    }

    response
        .text()
        .await
        .map_err(|e| AppError::NetworkError(format!("读取响应失败: {}", e)))
}

pub async fn get_fund_accum_trend(code: &str) -> AppResult<FundTrend> {
    if !FundInfo::validate_code(code) {
        return Err(AppError::ValidationError(
            "基金代码格式错误，请输入6位数字".to_string(),
        ));
    }

    let client = build_http_client();
    let url = format!("https://fund.eastmoney.com/pingzhongdata/{}.js", code);
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(AppError::FundTrendUnavailable(code.to_string()));
    }

    let text = response.text().await?;
    let mut points = parse_accum_trend_points(&text)?;
    if points.is_empty() {
        return Err(AppError::FundTrendUnavailable(code.to_string()));
    }

    let window = if points.len() > 30 {
        points = points.split_off(points.len() - 30);
        "最近30个交易日".to_string()
    } else {
        format!("最近{}个交易日", points.len())
    };

    Ok(FundTrend {
        code: code.to_string(),
        window,
        points,
    })
}

#[derive(Debug, Deserialize)]
struct NetWorthPoint {
    x: i64,
    y: f64,
}

fn parse_trend_points(text: &str) -> AppResult<Vec<TrendPoint>> {
    let array_text = extract_js_array(text, "Data_netWorthTrend")
        .ok_or_else(|| AppError::ParseError("缺少走势数据".to_string()))?;
    let raw_points: Vec<NetWorthPoint> = serde_json::from_str(&array_text)?;
    let mut points = Vec::with_capacity(raw_points.len());
    for point in raw_points {
        let date = chrono::NaiveDateTime::from_timestamp_millis(point.x)
            .map(|dt| dt.date().to_string())
            .unwrap_or_else(|| "1970-01-01".to_string());
        points.push(TrendPoint {
            date,
            value: point.y,
        });
    }
    Ok(points)
}

fn parse_accum_trend_points(text: &str) -> AppResult<Vec<TrendPoint>> {
    let array_text = extract_js_array(text, "Data_ACWorthTrend")
        .ok_or_else(|| AppError::ParseError("缺少累计走势数据".to_string()))?;
    let raw_points: Vec<Vec<serde_json::Value>> = serde_json::from_str(&array_text)?;
    let mut points = Vec::with_capacity(raw_points.len());
    let mut base: Option<f64> = None;
    for item in raw_points {
        if item.len() < 2 {
            continue;
        }
        let ts = item[0].as_i64();
        let value = item[1].as_f64();
        if let (Some(ts), Some(value)) = (ts, value) {
            let base_value = base.get_or_insert(value);
            let return_value = if *base_value == 0.0 {
                0.0
            } else {
                (value / *base_value - 1.0) * 100.0
            };
            let date = chrono::NaiveDateTime::from_timestamp_millis(ts)
                .map(|dt| dt.date().to_string())
                .unwrap_or_else(|| "1970-01-01".to_string());
            points.push(TrendPoint {
                date,
                value: return_value,
            });
        }
    }
    Ok(points)
}

fn extract_js_array(text: &str, var_name: &str) -> Option<String> {
    let index = text.find(var_name)?;
    let after = &text[index..];
    let start = after.find('[')? + index;
    let mut depth = 0;
    let mut end = None;
    for (offset, ch) in text[start..].char_indices() {
        match ch {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + offset + 1);
                    break;
                }
            }
            _ => {}
        }
    }
    let end = end?;
    Some(text[start..end].to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_trend_points() {
        let response = r#"
            var Data_netWorthTrend = [
              {"x":1697846400000,"y":1.234},
              {"x":1697932800000,"y":1.245}
            ];
        "#;
        let points = parse_trend_points(response).unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].value, 1.234);
        assert_eq!(points[1].value, 1.245);
    }
}
