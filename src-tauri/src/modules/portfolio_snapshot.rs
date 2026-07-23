use crate::errors::{AppError, AppResult};
use crate::modules::{crypto_api, fund_api, gold_api, stock_api};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum AssetCategory {
    Fund,
    Stock,
    Crypto,
    Gold,
}

impl AssetCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fund => "fund",
            Self::Stock => "stock",
            Self::Crypto => "crypto",
            Self::Gold => "gold",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Fund => "基金",
            Self::Stock => "股票",
            Self::Crypto => "加密货币",
            Self::Gold => "黄金",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ValuationBasis {
    Quote,
    CostFallback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetSnapshot {
    pub code: String,
    pub name: String,
    pub category: AssetCategory,
    /// 用户录入的成本金额，不参与当前市值占比计算。
    pub cost_amount: f64,
    /// 当前市值；有行情和份额时为 quantity * price，否则回退到成本金额。
    pub holding_amount: f64,
    pub holding_quantity: f64,
    pub current_price: Option<f64>,
    pub change_percent: Option<f64>,
    pub daily_change_amount: Option<f64>,
    pub valuation_basis: ValuationBasis,
    pub group_name: Option<String>,
    pub update_time: Option<String>,
    pub data_complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetAllocation {
    pub category: AssetCategory,
    pub label: String,
    pub total_value: f64,
    pub percent: f64,
    pub daily_change: f64,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMover {
    pub code: String,
    pub name: String,
    pub category: AssetCategory,
    pub holding_amount: f64,
    pub daily_change_amount: f64,
    pub change_percent: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationMetrics {
    pub max_single_percent: f64,
    pub max_single_name: String,
    pub top5_percent: f64,
    pub top5_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQuality {
    pub total_assets: usize,
    pub complete_assets: usize,
    pub missing_holding: usize,
    pub missing_quote: usize,
    pub quote_coverage_percent: f64,
    pub freshness: String,
    pub gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioSnapshot {
    pub id: String,
    pub snapshot_at: i64,
    pub total_value: f64,
    pub daily_change_amount: f64,
    /// 基于有行情覆盖资产的昨收市值计算，不把缺失行情资产假定为零涨跌。
    pub daily_change_percent: f64,
    pub daily_change_coverage_percent: f64,
    pub assets: Vec<AssetSnapshot>,
    pub allocation: Vec<AssetAllocation>,
    pub top_movers: Vec<AssetMover>,
    pub concentration: ConcentrationMetrics,
    pub data_quality: DataQuality,
}

pub async fn create_full_snapshot(pool: &SqlitePool) -> AppResult<PortfolioSnapshot> {
    let mut assets = collect_fund_assets(pool).await?;
    assets.extend(collect_stock_assets(pool).await?);
    assets.extend(collect_crypto_assets(pool).await?);
    assets.extend(collect_gold_assets(pool).await?);
    assets.sort_by(asset_identity_cmp);

    let snapshot = build_snapshot(Uuid::new_v4().to_string(), Utc::now().timestamp(), assets);
    save_snapshot(pool, &snapshot).await?;
    Ok(snapshot)
}

pub async fn get_latest_snapshot(pool: &SqlitePool) -> AppResult<Option<PortfolioSnapshot>> {
    load_snapshot(
        pool,
        "SELECT payload FROM portfolio_snapshots ORDER BY snapshot_at DESC, created_at DESC LIMIT 1",
        None,
    )
    .await
}

pub async fn get_snapshot_by_id(
    pool: &SqlitePool,
    id: &str,
) -> AppResult<Option<PortfolioSnapshot>> {
    load_snapshot(
        pool,
        "SELECT payload FROM portfolio_snapshots WHERE id = ?",
        Some(id),
    )
    .await
}

async fn load_snapshot(
    pool: &SqlitePool,
    sql: &str,
    id: Option<&str>,
) -> AppResult<Option<PortfolioSnapshot>> {
    let mut query = sqlx::query(sql);
    if let Some(id) = id {
        query = query.bind(id);
    }
    let row = query
        .fetch_optional(pool)
        .await
        .map_err(|error| AppError::StorageError(format!("读取快照失败: {error}")))?;
    row.map(|row| {
        let payload: String = row
            .try_get("payload")
            .map_err(|error| AppError::StorageError(format!("读取快照失败: {error}")))?;
        serde_json::from_str(&payload).map_err(AppError::from)
    })
    .transpose()
}

async fn save_snapshot(pool: &SqlitePool, snapshot: &PortfolioSnapshot) -> AppResult<()> {
    let payload = serde_json::to_string(snapshot)?;
    let data_quality = serde_json::to_string(&snapshot.data_quality)?;
    sqlx::query(
        "INSERT INTO portfolio_snapshots (id, snapshot_at, payload, data_quality, created_at) \
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&snapshot.id)
    .bind(snapshot.snapshot_at)
    .bind(payload)
    .bind(data_quality)
    .bind(Utc::now().timestamp())
    .execute(pool)
    .await
    .map_err(|error| AppError::StorageError(format!("保存快照失败: {error}")))?;
    Ok(())
}

fn build_snapshot(
    id: String,
    snapshot_at: i64,
    mut assets: Vec<AssetSnapshot>,
) -> PortfolioSnapshot {
    assets.sort_by(asset_identity_cmp);
    let total_value = round_money(assets.iter().map(|asset| asset.holding_amount).sum());
    let covered_value: f64 = assets
        .iter()
        .filter(|asset| asset.daily_change_amount.is_some())
        .map(|asset| asset.holding_amount)
        .sum();
    let daily_change_amount = round_money(
        assets
            .iter()
            .filter_map(|asset| asset.daily_change_amount)
            .sum(),
    );
    let previous_covered_value = covered_value - daily_change_amount;
    let daily_change_percent = if previous_covered_value > 0.0 {
        round_percent(daily_change_amount / previous_covered_value * 100.0)
    } else {
        0.0
    };
    let daily_change_coverage_percent = if total_value > 0.0 {
        round_percent(covered_value / total_value * 100.0)
    } else {
        0.0
    };

    PortfolioSnapshot {
        id,
        snapshot_at,
        total_value,
        daily_change_amount,
        daily_change_percent,
        daily_change_coverage_percent,
        allocation: compute_allocation(&assets, total_value),
        top_movers: compute_top_movers(&assets),
        concentration: compute_concentration(&assets, total_value),
        data_quality: compute_data_quality(&assets),
        assets,
    }
}

async fn collect_fund_assets(pool: &SqlitePool) -> AppResult<Vec<AssetSnapshot>> {
    let rows = sqlx::query(
        "SELECT p.fund_code, p.holding_amount, p.holding_shares, g.name AS group_name \
         FROM group_fund_positions p JOIN groups g ON g.id = p.group_id \
         WHERE p.holding_amount > 0 OR p.holding_shares > 0 \
         ORDER BY p.fund_code, g.id",
    )
    .fetch_all(pool)
    .await
    .map_err(|error| AppError::StorageError(format!("读取基金持仓失败: {error}")))?;

    let mut assets = Vec::with_capacity(rows.len());
    for row in rows {
        let code: String = row.try_get("fund_code").unwrap_or_default();
        let cost_amount = finite_non_negative(row.try_get("holding_amount").unwrap_or(0.0));
        let quantity = finite_non_negative(row.try_get("holding_shares").unwrap_or(0.0));
        let group_name = row.try_get("group_name").ok();
        let quote = fund_api::search_fund_info(&code).await.ok();
        let price = quote
            .as_ref()
            .and_then(|value| valid_positive(value.net_value));
        let change_percent = quote
            .as_ref()
            .and_then(|value| value.change_percent.as_deref())
            .and_then(parse_percent);
        assets.push(make_asset(
            code.clone(),
            quote
                .as_ref()
                .map(|value| value.name.clone())
                .unwrap_or(code),
            AssetCategory::Fund,
            cost_amount,
            quantity,
            price,
            change_percent,
            group_name,
            quote.and_then(|value| value.update_time),
        ));
    }
    Ok(assets)
}

async fn collect_stock_assets(pool: &SqlitePool) -> AppResult<Vec<AssetSnapshot>> {
    let rows = sqlx::query(
        "SELECT code, holding_amount, holding_shares FROM stock_holdings \
         WHERE holding_amount > 0 OR holding_shares > 0 ORDER BY code",
    )
    .fetch_all(pool)
    .await
    .map_err(|error| AppError::StorageError(format!("读取股票持仓失败: {error}")))?;
    let mut assets = Vec::with_capacity(rows.len());
    for row in rows {
        let code: String = row.try_get("code").unwrap_or_default();
        let cost_amount = finite_non_negative(row.try_get("holding_amount").unwrap_or(0.0));
        let quantity = finite_non_negative(row.try_get("holding_shares").unwrap_or(0.0));
        let quote = stock_api::get_stock_quote(&code).await.ok();
        assets.push(make_asset(
            code.clone(),
            quote
                .as_ref()
                .map(|value| value.name.clone())
                .unwrap_or(code),
            AssetCategory::Stock,
            cost_amount,
            quantity,
            quote.as_ref().and_then(|value| valid_positive(value.price)),
            quote
                .as_ref()
                .and_then(|value| valid_percent(value.change_percent)),
            None,
            quote.and_then(|value| value.update_time),
        ));
    }
    Ok(assets)
}

async fn collect_crypto_assets(pool: &SqlitePool) -> AppResult<Vec<AssetSnapshot>> {
    let rows = sqlx::query(
        "SELECT symbol, holding_amount, holding_quantity FROM crypto_holdings \
         WHERE symbol != 'AU9999' AND (holding_amount > 0 OR holding_quantity > 0) ORDER BY symbol",
    )
    .fetch_all(pool)
    .await
    .map_err(|error| AppError::StorageError(format!("读取加密持仓失败: {error}")))?;
    let mut assets = Vec::with_capacity(rows.len());
    for row in rows {
        let code: String = row.try_get("symbol").unwrap_or_default();
        let cost_amount = finite_non_negative(row.try_get("holding_amount").unwrap_or(0.0));
        let quantity = finite_non_negative(row.try_get("holding_quantity").unwrap_or(0.0));
        let quote = crypto_api::get_crypto_quote(&code).await.ok();
        assets.push(make_asset(
            code.clone(),
            quote
                .as_ref()
                .map(|value| value.name.clone())
                .unwrap_or(code),
            AssetCategory::Crypto,
            cost_amount,
            quantity,
            quote.as_ref().and_then(|value| valid_positive(value.price)),
            quote
                .as_ref()
                .and_then(|value| valid_percent(value.change_percent)),
            None,
            quote.and_then(|value| value.update_time),
        ));
    }
    Ok(assets)
}

async fn collect_gold_assets(pool: &SqlitePool) -> AppResult<Vec<AssetSnapshot>> {
    let row = sqlx::query(
        "SELECT holding_amount, holding_quantity FROM crypto_holdings \
         WHERE symbol = 'AU9999' AND (holding_amount > 0 OR holding_quantity > 0)",
    )
    .fetch_optional(pool)
    .await
    .map_err(|error| AppError::StorageError(format!("读取黄金持仓失败: {error}")))?;
    let Some(row) = row else {
        return Ok(Vec::new());
    };
    let quote = gold_api::get_gold_quote().await.ok();
    Ok(vec![make_asset(
        "AU9999".to_string(),
        quote
            .as_ref()
            .map(|value| value.name.clone())
            .unwrap_or_else(|| "黄金9999".to_string()),
        AssetCategory::Gold,
        finite_non_negative(row.try_get("holding_amount").unwrap_or(0.0)),
        finite_non_negative(row.try_get("holding_quantity").unwrap_or(0.0)),
        quote.as_ref().and_then(|value| valid_positive(value.price)),
        quote
            .as_ref()
            .and_then(|value| valid_percent(value.change_percent)),
        None,
        quote.and_then(|value| value.update_time),
    )])
}

#[allow(clippy::too_many_arguments)]
fn make_asset(
    code: String,
    name: String,
    category: AssetCategory,
    cost_amount: f64,
    quantity: f64,
    price: Option<f64>,
    change_percent: Option<f64>,
    group_name: Option<String>,
    update_time: Option<String>,
) -> AssetSnapshot {
    let quote_value = price
        .filter(|_| quantity > 0.0)
        .map(|price| quantity * price);
    let (holding_amount, valuation_basis) = match quote_value {
        Some(value) => (round_money(value), ValuationBasis::Quote),
        None => (round_money(cost_amount), ValuationBasis::CostFallback),
    };
    let daily_change_amount = if valuation_basis == ValuationBasis::Quote {
        compute_change_amount(change_percent, holding_amount)
    } else {
        None
    };
    let data_complete = valuation_basis == ValuationBasis::Quote && change_percent.is_some();
    AssetSnapshot {
        code,
        name,
        category,
        cost_amount: round_money(cost_amount),
        holding_amount,
        holding_quantity: quantity,
        current_price: price,
        change_percent,
        daily_change_amount,
        valuation_basis,
        group_name,
        update_time,
        data_complete,
    }
}

fn compute_change_amount(change_percent: Option<f64>, current_value: f64) -> Option<f64> {
    let rate = change_percent? / 100.0;
    if !rate.is_finite() || rate <= -1.0 || current_value < 0.0 {
        return None;
    }
    Some(round_money(current_value - current_value / (1.0 + rate)))
}

fn compute_allocation(assets: &[AssetSnapshot], total_value: f64) -> Vec<AssetAllocation> {
    let mut values: BTreeMap<AssetCategory, (f64, f64, usize)> = BTreeMap::new();
    for asset in assets {
        let entry = values.entry(asset.category.clone()).or_default();
        entry.0 += asset.holding_amount;
        entry.1 += asset.daily_change_amount.unwrap_or(0.0);
        entry.2 += 1;
    }
    let mut allocation: Vec<_> = values
        .into_iter()
        .map(|(category, (value, change, count))| AssetAllocation {
            label: category.label().to_string(),
            percent: if total_value > 0.0 {
                round_percent(value / total_value * 100.0)
            } else {
                0.0
            },
            total_value: round_money(value),
            daily_change: round_money(change),
            category,
            count,
        })
        .collect();
    allocation.sort_by(|left, right| {
        float_desc(left.total_value, right.total_value)
            .then_with(|| left.category.cmp(&right.category))
    });
    allocation
}

fn compute_top_movers(assets: &[AssetSnapshot]) -> Vec<AssetMover> {
    let mut movers: Vec<_> = assets
        .iter()
        .filter_map(|asset| {
            asset.daily_change_amount.map(|change| AssetMover {
                code: asset.code.clone(),
                name: asset.name.clone(),
                category: asset.category.clone(),
                holding_amount: asset.holding_amount,
                daily_change_amount: change,
                change_percent: asset.change_percent,
            })
        })
        .collect();
    movers.sort_by(|left, right| {
        float_desc(
            left.daily_change_amount.abs(),
            right.daily_change_amount.abs(),
        )
        .then_with(|| left.category.cmp(&right.category))
        .then_with(|| left.code.cmp(&right.code))
    });
    movers.truncate(5);
    movers
}

fn compute_concentration(assets: &[AssetSnapshot], total_value: f64) -> ConcentrationMetrics {
    if total_value <= 0.0 || assets.is_empty() {
        return ConcentrationMetrics {
            max_single_percent: 0.0,
            max_single_name: String::new(),
            top5_percent: 0.0,
            top5_names: Vec::new(),
        };
    }
    let mut sorted: Vec<_> = assets.iter().collect();
    sorted.sort_by(|left, right| {
        float_desc(left.holding_amount, right.holding_amount)
            .then_with(|| asset_identity_cmp(left, right))
    });
    let top5: Vec<_> = sorted.iter().take(5).copied().collect();
    ConcentrationMetrics {
        max_single_percent: round_percent(sorted[0].holding_amount / total_value * 100.0),
        max_single_name: sorted[0].name.clone(),
        top5_percent: round_percent(
            top5.iter().map(|asset| asset.holding_amount).sum::<f64>() / total_value * 100.0,
        ),
        top5_names: top5.iter().map(|asset| asset.name.clone()).collect(),
    }
}

fn compute_data_quality(assets: &[AssetSnapshot]) -> DataQuality {
    let total_assets = assets.len();
    let complete_assets = assets.iter().filter(|asset| asset.data_complete).count();
    let missing_holding = assets
        .iter()
        .filter(|asset| asset.holding_quantity <= 0.0)
        .count();
    let missing_quote = assets
        .iter()
        .filter(|asset| asset.current_price.is_none())
        .count();
    let covered_value: f64 = assets
        .iter()
        .filter(|asset| asset.current_price.is_some())
        .map(|asset| asset.holding_amount)
        .sum();
    let total_value: f64 = assets.iter().map(|asset| asset.holding_amount).sum();
    let mut gaps = Vec::new();
    for asset in assets {
        if asset.holding_quantity <= 0.0 {
            gaps.push(format!(
                "{}({}): 缺少持仓份额，当前市值回退为成本金额",
                asset.name, asset.code
            ));
        }
        if asset.current_price.is_none() {
            gaps.push(format!("{}({}): 行情数据缺失", asset.name, asset.code));
        } else if asset.change_percent.is_none() {
            gaps.push(format!("{}({}): 涨跌幅数据缺失", asset.name, asset.code));
        }
    }
    DataQuality {
        total_assets,
        complete_assets,
        missing_holding,
        missing_quote,
        quote_coverage_percent: if total_value > 0.0 {
            round_percent(covered_value / total_value * 100.0)
        } else {
            0.0
        },
        freshness: latest_update_time(assets).unwrap_or_else(|| "无可用行情时间".to_string()),
        gaps,
    }
}

fn latest_update_time(assets: &[AssetSnapshot]) -> Option<String> {
    assets
        .iter()
        .filter_map(|asset| asset.update_time.clone())
        .max()
}

fn parse_percent(value: &str) -> Option<f64> {
    value
        .trim()
        .trim_end_matches('%')
        .parse()
        .ok()
        .and_then(valid_number)
}

fn valid_percent(value: Option<f64>) -> Option<f64> {
    value.and_then(valid_number)
}

fn valid_number(value: f64) -> Option<f64> {
    value.is_finite().then_some(value)
}

fn valid_positive(value: Option<f64>) -> Option<f64> {
    value.filter(|value| value.is_finite() && *value > 0.0)
}

fn finite_non_negative(value: f64) -> f64 {
    if value.is_finite() && value >= 0.0 {
        value
    } else {
        0.0
    }
}

fn round_money(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn round_percent(value: f64) -> f64 {
    (value * 10_000.0).round() / 10_000.0
}

fn float_desc(left: f64, right: f64) -> Ordering {
    right.partial_cmp(&left).unwrap_or(Ordering::Equal)
}

fn asset_identity_cmp(left: &AssetSnapshot, right: &AssetSnapshot) -> Ordering {
    left.category
        .cmp(&right.category)
        .then_with(|| left.code.cmp(&right.code))
        .then_with(|| left.group_name.cmp(&right.group_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asset(
        code: &str,
        category: AssetCategory,
        current: f64,
        change: Option<f64>,
    ) -> AssetSnapshot {
        AssetSnapshot {
            code: code.to_string(),
            name: code.to_string(),
            category,
            cost_amount: current,
            holding_amount: current,
            holding_quantity: 1.0,
            current_price: Some(current),
            change_percent: change,
            daily_change_amount: compute_change_amount(change, current),
            valuation_basis: ValuationBasis::Quote,
            group_name: None,
            update_time: Some("2026-07-23 14:32:00".to_string()),
            data_complete: change.is_some(),
        }
    }

    #[test]
    fn change_amount_uses_previous_value_as_base() {
        assert_eq!(compute_change_amount(Some(10.0), 110.0), Some(10.0));
        assert_eq!(compute_change_amount(Some(-10.0), 90.0), Some(-10.0));
        assert_eq!(compute_change_amount(Some(-100.0), 0.0), None);
    }

    #[test]
    fn snapshot_metrics_are_deterministic_and_use_covered_denominator() {
        let assets = vec![
            asset("B", AssetCategory::Stock, 90.0, Some(-10.0)),
            asset("A", AssetCategory::Fund, 110.0, Some(10.0)),
            asset("C", AssetCategory::Crypto, 200.0, None),
        ];
        let first = build_snapshot("fixed".to_string(), 1, assets.clone());
        let second = build_snapshot("fixed".to_string(), 1, assets);
        assert_eq!(
            serde_json::to_string(&first).unwrap(),
            serde_json::to_string(&second).unwrap()
        );
        assert_eq!(first.total_value, 400.0);
        assert_eq!(first.daily_change_amount, 0.0);
        assert_eq!(first.daily_change_percent, 0.0);
        assert_eq!(first.daily_change_coverage_percent, 50.0);
        assert_eq!(first.assets[0].code, "A");
    }

    #[test]
    fn quote_value_is_separate_from_cost_amount() {
        let value = make_asset(
            "A".to_string(),
            "A".to_string(),
            AssetCategory::Stock,
            80.0,
            2.0,
            Some(50.0),
            Some(0.0),
            None,
            None,
        );
        assert_eq!(value.cost_amount, 80.0);
        assert_eq!(value.holding_amount, 100.0);
        assert_eq!(value.valuation_basis, ValuationBasis::Quote);
    }
}
