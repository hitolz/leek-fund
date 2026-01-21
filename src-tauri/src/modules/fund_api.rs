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

/// 搜索基金信息
pub async fn search_fund_info(code: &str) -> AppResult<FundInfo> {
    // 验证基金代码格式
    if !FundInfo::validate_code(code) {
        return Err(AppError::ValidationError(
            "基金代码格式错误，请输入6位数字".to_string(),
        ));
    }

    let client = build_http_client();
    let url = format!("http://fundgz.1234567.com.cn/js/{}.js", code);

    // 发送 HTTP 请求
    let response = client.get(&url).send().await?;

    // 检查响应状态
    if !response.status().is_success() {
        return Err(AppError::NotFound(code.to_string()));
    }

    // 获取响应文本
    let text = response.text().await?;

    // 解析 JSONP 响应
    parse_jsonp_response(&text, code)
}

pub async fn get_fund_summary(code: &str) -> AppResult<FundSummary> {
    let info = search_fund_info(code).await?;
    Ok(FundSummary {
        code: info.code,
        name: info.name,
        daily_change_percent: info.change_percent,
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
        update_time: info.update_time,
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
    let response = client.get(&url).send().await?;
    if !response.status().is_success() {
        return Err(AppError::FundTrendUnavailable(code.to_string()));
    }

    let text = response.text().await?;
    let mut points = parse_trend_points(&text)?;
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

/// 解析 JSONP 响应
/// 响应格式: jsonpgz({"fundcode":"001632","name":"...","gsz":"1.234","gztime":"2024-10-20 15:00"})
pub(crate) fn parse_jsonp_response(text: &str, code: &str) -> AppResult<FundInfo> {
    // 提取 JSON 部分（去掉 jsonpgz( 和 )），兼容尾部分号/空白
    let trimmed = text.trim();
    let trimmed = trimmed.strip_suffix(';').unwrap_or(trimmed).trim();
    let json_str = trimmed
        .strip_prefix("jsonpgz(")
        .and_then(|s| s.strip_suffix(")"))
        .ok_or_else(|| AppError::ParseError("无效的JSONP格式".to_string()))?;
    if json_str == "null" || json_str.is_empty() {
        return Err(AppError::NotFound(code.to_string()));
    }

    // 解析 JSON
    let json: serde_json::Value = serde_json::from_str(json_str)?;

    // 提取字段
    let fund_code = json["fundcode"]
        .as_str()
        .ok_or_else(|| AppError::ParseError("缺少基金代码".to_string()))?
        .to_string();

    let name = json["name"]
        .as_str()
        .ok_or_else(|| AppError::ParseError("缺少基金名称".to_string()))?
        .to_string();

    // 净值、涨跌幅和更新时间可能不存在
    let net_value = json["gsz"]
        .as_str()
        .and_then(|s| s.parse::<f64>().ok());

    let change_percent = json["gszzl"].as_str().map(|s| s.to_string());

    let update_time = json["gztime"].as_str().map(|s| s.to_string());

    // 验证基金代码是否匹配
    if fund_code != code {
        return Err(AppError::NotFound(code.to_string()));
    }

    Ok(FundInfo {
        code: fund_code,
        name,
        net_value,
        change_percent,
        update_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_jsonp_response() {
        let response = r#"jsonpgz({"fundcode":"001632","name":"兴全轻资产混合(LOF)","gsz":"3.1234","gztime":"2024-10-20 15:00"})"#;
        
        let fund = parse_jsonp_response(response, "001632").unwrap();
        assert_eq!(fund.code, "001632");
        assert_eq!(fund.name, "兴全轻资产混合(LOF)");
        assert_eq!(fund.net_value, Some(3.1234));
        assert_eq!(fund.change_percent, None);
        assert_eq!(fund.update_time, Some("2024-10-20 15:00".to_string()));
    }

    #[test]
    fn test_parse_jsonp_without_optional_fields() {
        let response = r#"jsonpgz({"fundcode":"001632","name":"测试基金"})"#;
        
        let fund = parse_jsonp_response(response, "001632").unwrap();
        assert_eq!(fund.code, "001632");
        assert_eq!(fund.name, "测试基金");
        assert_eq!(fund.net_value, None);
        assert_eq!(fund.change_percent, None);
        assert_eq!(fund.update_time, None);
    }

    #[test]
    fn test_invalid_jsonp_format() {
        let response = r#"invalid response"#;
        let result = parse_jsonp_response(response, "001632");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_jsonp_with_trailing_semicolon() {
        let response =
            r#"jsonpgz({"fundcode":"001632","name":"测试基金","gsz":"1.234"});"#;
        let fund = parse_jsonp_response(response, "001632").unwrap();
        assert_eq!(fund.code, "001632");
        assert_eq!(fund.name, "测试基金");
        assert_eq!(fund.net_value, Some(1.234));
    }

    #[test]
    fn test_parse_jsonp_with_whitespace() {
        let response =
            "  jsonpgz({\"fundcode\":\"001632\",\"name\":\"测试基金\"})  \n";
        let fund = parse_jsonp_response(response, "001632").unwrap();
        assert_eq!(fund.code, "001632");
        assert_eq!(fund.name, "测试基金");
    }

    #[test]
    fn test_parse_jsonp_null() {
        let response = r#"jsonpgz(null);"#;
        let result = parse_jsonp_response(response, "001632");
        assert!(result.is_err());
    }

    #[test]
    fn test_code_mismatch() {
        let response = r#"jsonpgz({"fundcode":"999999","name":"测试基金"})"#;
        let result = parse_jsonp_response(response, "001632");
        assert!(result.is_err());
    }

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
