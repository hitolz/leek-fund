use crate::errors::{AppError, AppResult};
use crate::models::FundInfo;
use chrono::{NaiveDate, Utc};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use sqlx::SqlitePool;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct JsVar {
    name: String,
    value: String,
    value_type: String,
}

#[derive(Debug, Deserialize)]
struct NetWorthPoint {
    x: i64,
    y: f64,
    #[serde(default)]
    equityReturn: Option<f64>,
    #[serde(default)]
    unitMoney: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RateInTypePoint {
    x: i64,
    y: i64,
    sc: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ManagerRaw {
    id: String,
    name: String,
    #[serde(default)]
    pic: Option<String>,
    #[serde(default)]
    star: Option<i64>,
    #[serde(default)]
    workTime: Option<String>,
    #[serde(default)]
    fundSize: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SeriesRaw {
    name: String,
    data: Vec<Option<f64>>,
}

#[derive(Debug, Deserialize)]
struct AllocationRaw {
    categories: Vec<String>,
    series: Vec<SeriesRaw>,
}

/// Save pingzhongdata payload and structured snapshots.
pub async fn save_pingzhong_payload(
    pool: &SqlitePool,
    fund_code: &str,
    payload: &str,
) -> AppResult<()> {
    if !FundInfo::validate_code(fund_code) {
        return Err(AppError::ValidationError(
            "基金代码格式错误，请输入6位数字".to_string(),
        ));
    }

    let fetched_at = Utc::now().timestamp_millis();
    let vars = extract_all_vars(payload);
    let mut var_map = HashMap::new();
    for var in &vars {
        var_map.insert(var.name.clone(), var.value.clone());
    }

    let mut tx = pool
        .begin()
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    sqlx::query(
        "INSERT INTO fund_pingzhong_raw (fund_code, fetched_at, payload) VALUES (?, ?, ?)",
    )
    .bind(fund_code)
    .bind(fetched_at)
    .bind(payload)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    for var in &vars {
        sqlx::query(
            "INSERT INTO fund_pingzhong_kv (fund_code, fetched_at, var_name, value_type, value_text) \
             VALUES (?, ?, ?, ?, ?) \
             ON CONFLICT(fund_code, fetched_at, var_name) DO UPDATE SET \
             value_type = excluded.value_type, value_text = excluded.value_text",
        )
        .bind(fund_code)
        .bind(fetched_at)
        .bind(&var.name)
        .bind(&var.value_type)
        .bind(&var.value)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
    }

    save_profile(&mut tx, fund_code, fetched_at, &var_map).await?;
    save_returns(&mut tx, fund_code, fetched_at, &var_map).await?;
    save_nav_trend(&mut tx, fund_code, fetched_at, &var_map).await?;
    save_rank_trend(&mut tx, fund_code, fetched_at, &var_map).await?;
    save_allocation(&mut tx, fund_code, fetched_at, &var_map).await?;
    save_holder_structure(&mut tx, fund_code, fetched_at, &var_map).await?;
    save_managers(&mut tx, fund_code, fetched_at, &var_map).await?;

    tx.commit()
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

async fn save_profile(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    fund_code: &str,
    updated_at: i64,
    var_map: &HashMap<String, String>,
) -> AppResult<()> {
    let name = match var_map.get("fS_name") {
        Some(value) => strip_quotes(value),
        None => return Ok(()),
    };

    let is_money = var_map
        .get("ishb")
        .map(|value| value.trim() == "true")
        .unwrap_or(false);

    let source_rate = var_map
        .get("fund_sourceRate")
        .and_then(|value| strip_quotes(value).parse::<f64>().ok());

    let rate = var_map
        .get("fund_Rate")
        .and_then(|value| strip_quotes(value).parse::<f64>().ok());

    let min_purchase = var_map
        .get("fund_minsg")
        .and_then(|value| strip_quotes(value).parse::<f64>().ok());

    sqlx::query(
        "INSERT INTO fund_profile (fund_code, name, is_money, source_rate, rate, min_purchase, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?) \
         ON CONFLICT(fund_code) DO UPDATE SET \
         name = excluded.name, is_money = excluded.is_money, source_rate = excluded.source_rate, \
         rate = excluded.rate, min_purchase = excluded.min_purchase, updated_at = excluded.updated_at",
    )
    .bind(fund_code)
    .bind(name)
    .bind(if is_money { 1 } else { 0 })
    .bind(source_rate)
    .bind(rate)
    .bind(min_purchase)
    .bind(updated_at)
    .execute(&mut **tx)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

async fn save_returns(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    fund_code: &str,
    updated_at: i64,
    var_map: &HashMap<String, String>,
) -> AppResult<()> {
    let periods = [
        ("syl_1n", "1m"),
        ("syl_6y", "6m"),
        ("syl_1y", "1y"),
        ("syl_3y", "3y"),
    ];

    for (key, period) in periods {
        if let Some(value) = var_map.get(key) {
            if let Ok(parsed) = strip_quotes(value).parse::<f64>() {
                sqlx::query(
                    "INSERT INTO fund_return_summary (fund_code, period, value, updated_at) \
                     VALUES (?, ?, ?, ?) \
                     ON CONFLICT(fund_code, period) DO UPDATE SET \
                     value = excluded.value, updated_at = excluded.updated_at",
                )
                .bind(fund_code)
                .bind(period)
                .bind(parsed)
                .bind(updated_at)
                .execute(&mut **tx)
                .await
                .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
            }
        }
    }

    Ok(())
}

async fn save_nav_trend(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    fund_code: &str,
    updated_at: i64,
    var_map: &HashMap<String, String>,
) -> AppResult<()> {
    if let Some(value) = var_map.get("Data_netWorthTrend") {
        let points: Vec<NetWorthPoint> = parse_json(value)?;
        for point in points {
            upsert_nav_point(
                tx,
                fund_code,
                point.x,
                Some(point.y),
                None,
                point.equityReturn,
                point.unitMoney,
                updated_at,
            )
            .await?;
        }
    }

    if let Some(value) = var_map.get("Data_ACWorthTrend") {
        let points = parse_pair_array(value)?;
        for (ts, accum) in points {
            upsert_nav_point(
                tx,
                fund_code,
                ts,
                None,
                Some(accum),
                None,
                None,
                updated_at,
            )
            .await?;
        }
    }

    Ok(())
}

async fn upsert_nav_point(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    fund_code: &str,
    nav_date: i64,
    unit_nav: Option<f64>,
    accum_nav: Option<f64>,
    equity_return: Option<f64>,
    unit_money: Option<String>,
    updated_at: i64,
) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO fund_nav_daily (fund_code, nav_date, unit_nav, accum_nav, equity_return, unit_money, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?) \
         ON CONFLICT(fund_code, nav_date) DO UPDATE SET \
         unit_nav = COALESCE(excluded.unit_nav, fund_nav_daily.unit_nav), \
         accum_nav = COALESCE(excluded.accum_nav, fund_nav_daily.accum_nav), \
         equity_return = COALESCE(excluded.equity_return, fund_nav_daily.equity_return), \
         unit_money = COALESCE(excluded.unit_money, fund_nav_daily.unit_money), \
         updated_at = excluded.updated_at",
    )
    .bind(fund_code)
    .bind(nav_date)
    .bind(unit_nav)
    .bind(accum_nav)
    .bind(equity_return)
    .bind(unit_money)
    .bind(updated_at)
    .execute(&mut **tx)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

async fn save_rank_trend(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    fund_code: &str,
    updated_at: i64,
    var_map: &HashMap<String, String>,
) -> AppResult<()> {
    if let Some(value) = var_map.get("Data_rateInSimilarType") {
        let points: Vec<RateInTypePoint> = parse_json(value)?;
        for point in points {
            let total = match point.sc {
                serde_json::Value::Number(num) => num.as_i64(),
                serde_json::Value::String(s) => s.parse::<i64>().ok(),
                _ => None,
            };
            upsert_rank_point(
                tx,
                fund_code,
                point.x,
                Some(point.y),
                total,
                None,
                updated_at,
            )
            .await?;
        }
    }

    if let Some(value) = var_map.get("Data_rateInSimilarPersent") {
        let points = parse_pair_array(value)?;
        for (ts, percentile) in points {
            upsert_rank_point(
                tx,
                fund_code,
                ts,
                None,
                None,
                Some(percentile),
                updated_at,
            )
            .await?;
        }
    }

    Ok(())
}

async fn upsert_rank_point(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    fund_code: &str,
    rank_date: i64,
    rank: Option<i64>,
    total: Option<i64>,
    percentile: Option<f64>,
    updated_at: i64,
) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO fund_rank_daily (fund_code, rank_date, rank, total, percentile, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?) \
         ON CONFLICT(fund_code, rank_date) DO UPDATE SET \
         rank = COALESCE(excluded.rank, fund_rank_daily.rank), \
         total = COALESCE(excluded.total, fund_rank_daily.total), \
         percentile = COALESCE(excluded.percentile, fund_rank_daily.percentile), \
         updated_at = excluded.updated_at",
    )
    .bind(fund_code)
    .bind(rank_date)
    .bind(rank)
    .bind(total)
    .bind(percentile)
    .bind(updated_at)
    .execute(&mut **tx)
    .await
    .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

    Ok(())
}

async fn save_allocation(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    fund_code: &str,
    updated_at: i64,
    var_map: &HashMap<String, String>,
) -> AppResult<()> {
    let value = match var_map.get("Data_assetAllocation") {
        Some(value) => value,
        None => return Ok(()),
    };
    let allocation: AllocationRaw = parse_json(value)?;
    if allocation.categories.is_empty() {
        return Ok(());
    }

    let mut series_map = HashMap::new();
    for series in allocation.series {
        series_map.insert(series.name, series.data);
    }

    for (idx, date_str) in allocation.categories.iter().enumerate() {
        let report_date = match parse_date_to_millis(date_str) {
            Some(value) => value,
            None => continue,
        };

        let stock_pct = get_series_value(&series_map, "股票占净比", idx);
        let bond_pct = get_series_value(&series_map, "债券占净比", idx);
        let cash_pct = get_series_value(&series_map, "现金占净比", idx);
        let other_pct = get_series_value(&series_map, "其他占净比", idx);

        sqlx::query(
            "INSERT INTO fund_asset_allocation (fund_code, report_date, stock_pct, bond_pct, cash_pct, other_pct, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?) \
             ON CONFLICT(fund_code, report_date) DO UPDATE SET \
             stock_pct = excluded.stock_pct, bond_pct = excluded.bond_pct, \
             cash_pct = excluded.cash_pct, other_pct = excluded.other_pct, \
             updated_at = excluded.updated_at",
        )
        .bind(fund_code)
        .bind(report_date)
        .bind(stock_pct)
        .bind(bond_pct)
        .bind(cash_pct)
        .bind(other_pct)
        .bind(updated_at)
        .execute(&mut **tx)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
    }

    Ok(())
}

async fn save_holder_structure(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    fund_code: &str,
    updated_at: i64,
    var_map: &HashMap<String, String>,
) -> AppResult<()> {
    let value = match var_map.get("Data_holderStructure") {
        Some(value) => value,
        None => return Ok(()),
    };
    let structure: AllocationRaw = parse_json(value)?;
    if structure.categories.is_empty() {
        return Ok(());
    }

    let mut series_map = HashMap::new();
    for series in structure.series {
        series_map.insert(series.name, series.data);
    }

    for (idx, date_str) in structure.categories.iter().enumerate() {
        let report_date = match parse_date_to_millis(date_str) {
            Some(value) => value,
            None => continue,
        };

        let institution_pct = get_series_value(&series_map, "机构持有比例", idx);
        let individual_pct = get_series_value(&series_map, "个人持有比例", idx);
        let internal_pct = get_series_value(&series_map, "内部持有比例", idx);

        sqlx::query(
            "INSERT INTO fund_holder_structure (fund_code, report_date, institution_pct, individual_pct, internal_pct, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?) \
             ON CONFLICT(fund_code, report_date) DO UPDATE SET \
             institution_pct = excluded.institution_pct, individual_pct = excluded.individual_pct, \
             internal_pct = excluded.internal_pct, updated_at = excluded.updated_at",
        )
        .bind(fund_code)
        .bind(report_date)
        .bind(institution_pct)
        .bind(individual_pct)
        .bind(internal_pct)
        .bind(updated_at)
        .execute(&mut **tx)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
    }

    Ok(())
}

async fn save_managers(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    fund_code: &str,
    updated_at: i64,
    var_map: &HashMap<String, String>,
) -> AppResult<()> {
    let value = match var_map.get("Data_currentFundManager") {
        Some(value) => value,
        None => return Ok(()),
    };

    let managers: Vec<ManagerRaw> = parse_json(value)?;
    for manager in managers {
        sqlx::query(
            "INSERT INTO fund_manager (manager_id, name, star, pic_url, work_time_text) \
             VALUES (?, ?, ?, ?, ?) \
             ON CONFLICT(manager_id) DO UPDATE SET \
             name = excluded.name, star = excluded.star, pic_url = excluded.pic_url, \
             work_time_text = excluded.work_time_text",
        )
        .bind(&manager.id)
        .bind(&manager.name)
        .bind(manager.star)
        .bind(manager.pic)
        .bind(manager.workTime)
        .execute(&mut **tx)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;

        sqlx::query(
            "INSERT INTO fund_manager_rel (fund_code, manager_id, fund_size_text, updated_at) \
             VALUES (?, ?, ?, ?) \
             ON CONFLICT(fund_code, manager_id) DO UPDATE SET \
             fund_size_text = excluded.fund_size_text, updated_at = excluded.updated_at",
        )
        .bind(fund_code)
        .bind(&manager.id)
        .bind(manager.fundSize)
        .bind(updated_at)
        .execute(&mut **tx)
        .await
        .map_err(|e| AppError::StorageError(format!("数据库写入失败: {}", e)))?;
    }

    Ok(())
}

fn parse_json<T: DeserializeOwned>(value: &str) -> AppResult<T> {
    let parsed = serde_json::from_str(value)?;
    Ok(parsed)
}

fn parse_pair_array(value: &str) -> AppResult<Vec<(i64, f64)>> {
    let raw: Vec<Vec<serde_json::Value>> = parse_json(value)?;
    let mut out = Vec::with_capacity(raw.len());
    for item in raw {
        if item.len() < 2 {
            continue;
        }
        let ts = item[0].as_i64();
        let val = item[1].as_f64();
        if let (Some(ts), Some(val)) = (ts, val) {
            out.push((ts, val));
        }
    }
    Ok(out)
}

fn parse_date_to_millis(date_str: &str) -> Option<i64> {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;
    date.and_hms_opt(0, 0, 0)
        .map(|dt| dt.and_utc().timestamp_millis())
}

fn get_series_value(
    series_map: &HashMap<String, Vec<Option<f64>>>,
    key: &str,
    index: usize,
) -> Option<f64> {
    series_map
        .get(key)
        .and_then(|values| values.get(index).cloned().flatten())
}

fn strip_quotes(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.len() >= 2 {
        let first = trimmed.chars().next().unwrap_or('\0');
        let last = trimmed.chars().last().unwrap_or('\0');
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return trimmed[1..trimmed.len() - 1].to_string();
        }
    }
    trimmed.to_string()
}

fn classify_value(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.starts_with('[') {
        "array".to_string()
    } else if trimmed.starts_with('{') {
        "object".to_string()
    } else if trimmed.starts_with('"') || trimmed.starts_with('\'') {
        "string".to_string()
    } else if trimmed == "true" || trimmed == "false" {
        "bool".to_string()
    } else if trimmed == "null" {
        "null".to_string()
    } else if trimmed.parse::<f64>().is_ok() {
        "number".to_string()
    } else {
        "unknown".to_string()
    }
}

fn extract_all_vars(text: &str) -> Vec<JsVar> {
    let mut vars = Vec::new();
    let mut idx = 0;
    while let Some(pos) = text[idx..].find("var ") {
        let start = idx + pos + 4;
        let mut name_end = start;
        for (offset, ch) in text[start..].char_indices() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                name_end = start + offset + ch.len_utf8();
            } else {
                break;
            }
        }
        if name_end <= start {
            idx = start;
            continue;
        }
        let name = text[start..name_end].trim().to_string();
        let mut cursor = name_end;
        while cursor < text.len() && text.as_bytes()[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor >= text.len() || text.as_bytes()[cursor] != b'=' {
            idx = cursor + 1;
            continue;
        }
        cursor += 1;
        while cursor < text.len() && text.as_bytes()[cursor].is_ascii_whitespace() {
            cursor += 1;
        }
        if cursor >= text.len() {
            break;
        }
        if let Some((value, end_idx)) = parse_js_value(text, cursor) {
            let value_type = classify_value(&value);
            vars.push(JsVar {
                name,
                value,
                value_type,
            });
            idx = end_idx;
        } else {
            idx = cursor + 1;
        }
    }
    vars
}

fn parse_js_value(text: &str, start: usize) -> Option<(String, usize)> {
    let bytes = text.as_bytes();
    let first = *bytes.get(start)?;
    if first == b'[' {
        let (value, end) = capture_enclosed(text, start, '[', ']')?;
        return Some((value, end));
    }
    if first == b'{' {
        let (value, end) = capture_enclosed(text, start, '{', '}')?;
        return Some((value, end));
    }
    if first == b'"' || first == b'\'' {
        let (value, end) = capture_string(text, start)?;
        return Some((value, end));
    }

    let mut end = start;
    while end < text.len() {
        let ch = text.as_bytes()[end];
        if ch == b';' {
            break;
        }
        end += 1;
    }
    if end <= start {
        return None;
    }
    Some((text[start..end].trim().to_string(), end))
}

fn capture_enclosed(text: &str, start: usize, open: char, close: char) -> Option<(String, usize)> {
    let mut depth = 0;
    let mut in_string: Option<char> = None;
    let mut escape = false;
    let mut end = None;
    for (offset, ch) in text[start..].char_indices() {
        if let Some(quote) = in_string {
            if escape {
                escape = false;
                continue;
            }
            if ch == '\\' {
                escape = true;
                continue;
            }
            if ch == quote {
                in_string = None;
            }
            continue;
        }

        if ch == '"' || ch == '\'' {
            in_string = Some(ch);
            continue;
        }

        if ch == open {
            depth += 1;
        } else if ch == close {
            depth -= 1;
            if depth == 0 {
                end = Some(start + offset + ch.len_utf8());
                break;
            }
        }
    }
    let end = end?;
    Some((text[start..end].to_string(), end))
}

fn capture_string(text: &str, start: usize) -> Option<(String, usize)> {
    let quote = text[start..].chars().next()?;
    let mut escape = false;
    let mut end = None;
    for (offset, ch) in text[start + 1..].char_indices() {
        if escape {
            escape = false;
            continue;
        }
        if ch == '\\' {
            escape = true;
            continue;
        }
        if ch == quote {
            end = Some(start + 1 + offset + ch.len_utf8());
            break;
        }
    }
    let end = end?;
    Some((text[start..end].to_string(), end))
}
