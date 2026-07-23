use crate::errors::{AppError, AppResult};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

// ============================================================================
// Types
// ============================================================================

#[derive(Debug, Clone, serde::Serialize)]
pub struct NewsItem {
    pub title: String,
    pub summary: String,
    pub source: String,
    pub url: String,
    pub publish_time: String,
    pub related_codes: Vec<String>,
}

// ============================================================================
// HTTP client
// ============================================================================

fn build_client() -> Client {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .build()
        .expect("Failed to build HTTP client")
}

// ============================================================================
// 1. 东方财富快讯
// ============================================================================

#[derive(Deserialize)]
struct EastMoneyResponse {
    data: Option<EastMoneyData>,
}

#[derive(Deserialize)]
struct EastMoneyData {
    list: Option<Vec<EastMoneyItem>>,
}

#[derive(Deserialize)]
struct EastMoneyItem {
    #[serde(rename = "art_title")]
    title: Option<String>,
    #[serde(rename = "art_content")]
    content: Option<String>,
    #[serde(rename = "showTime")]
    show_time: Option<String>,
    #[serde(rename = "mediaName")]
    media_name: Option<String>,
    #[serde(rename = "art_code")]
    art_code: Option<String>,
    #[serde(rename = "stock_list")]
    stock_list: Option<Vec<EastMoneyStock>>,
}

#[derive(Deserialize)]
struct EastMoneyStock {
    #[serde(rename = "stock_code")]
    code: Option<String>,
}

/// 东方财富快讯 - 财经要闻
pub async fn get_eastmoney_news(page: usize, page_size: usize) -> AppResult<Vec<NewsItem>> {
    let client = build_client();
    let url = format!(
        "https://np-listapi.eastmoney.com/comm/web/getNewsByColumns?client=web&biz=web_news_col&column=350&order=1&needInteractData=0&page_index={}&page_size={}",
        page, page_size
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("请求东方财富快讯失败: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError::NetworkError(format!(
            "东方财富快讯返回错误: {}",
            resp.status()
        )));
    }

    let data: EastMoneyResponse = resp
        .json()
        .await
        .map_err(|e| AppError::ParseError(format!("解析东方财富快讯失败: {e}")))?;

    let items = data
        .data
        .and_then(|d| d.list)
        .unwrap_or_default();

    Ok(items
        .into_iter()
        .map(|item| {
            let related_codes = item
                .stock_list
                .unwrap_or_default()
                .into_iter()
                .filter_map(|s| s.code)
                .collect();

            NewsItem {
                title: item.title.unwrap_or_default(),
                summary: item.content.unwrap_or_default(),
                source: item.media_name.unwrap_or_else(|| "东方财富".into()),
                url: item
                    .art_code
                    .map(|code| format!("https://finance.eastmoney.com/a/{}.html", code))
                    .unwrap_or_default(),
                publish_time: item.show_time.unwrap_or_default(),
                related_codes,
            }
        })
        .collect())
}

/// 东方财富 - 按股票代码查新闻
pub async fn get_eastmoney_stock_news(
    stock_code: &str,
    page: usize,
    page_size: usize,
) -> AppResult<Vec<NewsItem>> {
    let client = build_client();
    // 去掉 sh/hk 前缀
    let pure_code = stock_code
        .trim_start_matches("sh")
        .trim_start_matches("sz")
        .trim_start_matches("hk");

    let url = format!(
        "https://search-api-web.eastmoney.com/search/jsonp?cb=&param={{\"uid\":\"\",\"keyword\":\"{}\",\"type\":\"cmsArticleWebOld\",\"client\":\"web\",\"clientType\":\"web\",\"clientVersion\":\"curr\",\"param\":{{\"cmsArticleWebOld\":{{\"searchScope\":\"default\",\"sort\":\"default\",\"pageIndex\":{},\"pageSize\":{},\"preTag\":\"\",\"postTag\":\"\"}}}}}}",
        pure_code, page, page_size
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("请求东方财富股票新闻失败: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError::NetworkError(format!(
            "东方财富股票新闻返回错误: {}",
            resp.status()
        )));
    }

    let text = resp
        .text()
        .await
        .map_err(|e| AppError::NetworkError(format!("读取响应失败: {e}")))?;

    // JSONP 响应需要去掉外层包装
    let json_str = if text.starts_with("(") && text.ends_with(")") {
        &text[1..text.len() - 1]
    } else {
        &text
    };

    let data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| AppError::ParseError(format!("解析东方财富股票新闻失败: {e}")))?;

    let mut results = Vec::new();
    if let Some(list) = data["result"]["cmsArticleWebOld"]["list"].as_array() {
        for item in list {
            let title = item["title"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let content = item["content"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let url = item["url"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let date = item["date"]
                .as_str()
                .unwrap_or("")
                .to_string();
            let media = item["mediaName"]
                .as_str()
                .unwrap_or("东方财富")
                .to_string();

            if !title.is_empty() {
                results.push(NewsItem {
                    title,
                    summary: content,
                    source: media,
                    url,
                    publish_time: date,
                    related_codes: vec![pure_code.to_string()],
                });
            }
        }
    }

    Ok(results)
}

// ============================================================================
// 2. 新浪财经
// ============================================================================

#[derive(Deserialize)]
struct SinaRollResponse {
    result: Option<SinaRollResult>,
}

#[derive(Deserialize)]
struct SinaRollResult {
    data: Option<Vec<SinaNewsItem>>,
}

#[derive(Deserialize)]
struct SinaNewsItem {
    title: Option<String>,
    intro: Option<String>,
    url: Option<String>,
    ctime: Option<String>,
    media: Option<String>,
}

/// 新浪财经滚动新闻
pub async fn get_sina_news(page: usize, page_size: usize) -> AppResult<Vec<NewsItem>> {
    let client = build_client();
    let lid = 2516; // 财经新闻频道
    let url = format!(
        "https://feed.mix.sina.com.cn/api/roll/get?pageid=153&lid={}&k=&num={}&page={}&r=0.1",
        lid, page_size, page
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("请求新浪财经失败: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError::NetworkError(format!(
            "新浪财经返回错误: {}",
            resp.status()
        )));
    }

    let data: SinaRollResponse = resp
        .json()
        .await
        .map_err(|e| AppError::ParseError(format!("解析新浪财经失败: {e}")))?;

    let items = data.result.and_then(|r| r.data).unwrap_or_default();

    Ok(items
        .into_iter()
        .map(|item| NewsItem {
            title: item.title.unwrap_or_default(),
            summary: item.intro.unwrap_or_default(),
            source: item.media.unwrap_or_else(|| "新浪财经".into()),
            url: item.url.unwrap_or_default(),
            publish_time: item.ctime.unwrap_or_default(),
            related_codes: Vec::new(),
        })
        .collect())
}

// ============================================================================
// 3. 同花顺问财
// ============================================================================

#[derive(Deserialize)]
struct WencaiResponse {
    data: Option<serde_json::Value>,
}

/// 同花顺问财 - 自然语言查询
pub async fn query_wencai(question: &str) -> AppResult<Vec<NewsItem>> {
    let client = build_client();

    let resp = client
        .post("https://www.iwencai.com/customized/chart/get-robot-data")
        .header("Content-Type", "application/json")
        .header("Referer", "https://www.iwencai.com/")
        .json(&serde_json::json!({
            "question": question,
            "perpage": 10,
            "page": 1,
            "secondary_intent": "news",
            "log_info": "{\"input_type\":\"typewrite\"}",
            "source": "Ths_iwencai_Xuangu",
            "version": "2.0",
            "query_area": "",
            "block_list": "",
            "add_info": "{\"urp\":{\"scene\":1,\"company\":1,\"business\":1},\"contentType\":\"json\",\"searchInfo\":true}"
        }))
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("请求同花顺问财失败: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError::NetworkError(format!(
            "同花顺问财返回错误: {}",
            resp.status()
        )));
    }

    let data: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| AppError::ParseError(format!("解析同花顺问财失败: {e}")))?;

    let mut results = Vec::new();

    // 尝试解析问财返回的数据
    if let Some(items) = data["data"]["answer"][0]["txt"][0]["content"]["components"]
        .as_array()
    {
        for item in items {
            if let Some(rows) = item["data"]["datas"].as_array() {
                for row in rows {
                    let title = row
                        .get("标题")
                        .or_else(|| row.get("title"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let summary = row
                        .get("摘要")
                        .or_else(|| row.get("content"))
                        .or_else(|| row.get("summary"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let url = row
                        .get("链接")
                        .or_else(|| row.get("url"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let time = row
                        .get("发布时间")
                        .or_else(|| row.get("date"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    if !title.is_empty() {
                        results.push(NewsItem {
                            title,
                            summary,
                            source: "同花顺问财".into(),
                            url,
                            publish_time: time,
                            related_codes: Vec::new(),
                        });
                    }
                }
            }
        }
    }

    // 如果上面解析方式没结果，尝试直接返回文本
    if results.is_empty() {
        if let Some(text) = data["data"]["answer"][0]["txt"][0]["content"]["components"][0]["data"]
            .as_str()
        {
            if !text.is_empty() {
                results.push(NewsItem {
                    title: format!("问财查询: {}", question),
                    summary: text.to_string(),
                    source: "同花顺问财".into(),
                    url: "https://www.iwencai.com/".into(),
                    publish_time: String::new(),
                    related_codes: Vec::new(),
                });
            }
        }
    }

    Ok(results)
}

// ============================================================================
// 4. 按主题搜索新闻（东方财富搜索）
// ============================================================================

/// 按关键词搜索财经新闻
pub async fn search_topic_news(keyword: &str, page_size: usize) -> AppResult<Vec<NewsItem>> {
    let client = build_client();
    let url = format!(
        "https://search-api-web.eastmoney.com/search/jsonp?cb=&param={{\"uid\":\"\",\"keyword\":\"{}\",\"type\":\"cmsArticleWebOld\",\"client\":\"web\",\"clientType\":\"web\",\"clientVersion\":\"curr\",\"param\":{{\"cmsArticleWebOld\":{{\"searchScope\":\"default\",\"sort\":\"default\",\"pageIndex\":1,\"pageSize\":{},\"preTag\":\"\",\"postTag\":\"\"}}}}}}",
        keyword, page_size
    );

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("搜索新闻失败: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError::NetworkError(format!(
            "搜索新闻返回错误: {}",
            resp.status()
        )));
    }

    let text = resp
        .text()
        .await
        .map_err(|e| AppError::NetworkError(format!("读取响应失败: {e}")))?;

    let json_str = if text.starts_with("(") && text.ends_with(")") {
        &text[1..text.len() - 1]
    } else {
        &text
    };

    let data: serde_json::Value = serde_json::from_str(json_str)
        .map_err(|e| AppError::ParseError(format!("解析搜索结果失败: {e}")))?;

    let mut results = Vec::new();
    if let Some(list) = data["result"]["cmsArticleWebOld"]["list"].as_array() {
        for item in list {
            let title = item["title"].as_str().unwrap_or("").to_string();
            let content = item["content"].as_str().unwrap_or("").to_string();
            let url = item["url"].as_str().unwrap_or("").to_string();
            let date = item["date"].as_str().unwrap_or("").to_string();
            let media = item["mediaName"].as_str().unwrap_or("东方财富").to_string();

            if !title.is_empty() {
                results.push(NewsItem {
                    title,
                    summary: content,
                    source: media,
                    url,
                    publish_time: date,
                    related_codes: Vec::new(),
                });
            }
        }
    }

    Ok(results)
}
