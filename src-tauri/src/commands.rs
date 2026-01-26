use crate::models::{AppState, FundDetail, FundInfo, FundList, FundSummary, FundTrend};
use crate::modules::{fund_api, fund_storage, list_manager, position_manager};
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

/// 搜索基金信息
#[tauri::command]
pub async fn search_fund(code: String) -> Result<FundInfo, String> {
    eprintln!("search_fund called with code={}", code);
    fund_api::search_fund_info(&code)
        .await
        .map_err(|e| e.user_message())
}

/// 获取所有列表
#[tauri::command]
pub async fn get_all_lists(state: State<'_, Mutex<AppState>>) -> Result<Vec<FundList>, String> {
    list_manager::get_all_lists(&state)
        .await
        .map_err(|e| e.user_message())
}

/// 创建新列表
#[tauri::command]
pub async fn create_list(
    state: State<'_, Mutex<AppState>>,
    name: String,
) -> Result<FundList, String> {
    list_manager::create_list(&state, name)
        .await
        .map_err(|e| e.user_message())
}

/// 重命名列表
#[tauri::command]
pub async fn rename_list(
    state: State<'_, Mutex<AppState>>,
    id: i64,
    new_name: String,
) -> Result<(), String> {
    list_manager::rename_list(&state, id, new_name)
        .await
        .map_err(|e| e.user_message())
}

/// 删除列表
#[tauri::command]
pub async fn delete_list(state: State<'_, Mutex<AppState>>, id: i64) -> Result<(), String> {
    list_manager::delete_list(&state, id)
        .await
        .map_err(|e| e.user_message())
}

/// 添加基金到列表
#[tauri::command]
pub async fn add_fund_to_list(
    state: State<'_, Mutex<AppState>>,
    list_id: i64,
    fund_code: String,
) -> Result<(), String> {
    list_manager::add_fund_to_list(&state, list_id, fund_code)
        .await
        .map_err(|e| e.user_message())
}

/// 从列表中移除基金
#[tauri::command]
pub async fn remove_fund_from_list(
    state: State<'_, Mutex<AppState>>,
    list_id: i64,
    fund_code: String,
) -> Result<(), String> {
    list_manager::remove_fund_from_list(&state, list_id, fund_code)
        .await
        .map_err(|e| e.user_message())
}

/// 获取列表中的所有基金详情
#[tauri::command]
pub async fn get_list_funds(
    state: State<'_, Mutex<AppState>>,
    list_id: i64,
) -> Result<Vec<FundInfo>, String> {
    let fund_codes = list_manager::get_list_fund_codes(&state, list_id)
        .await
        .map_err(|e| e.user_message())?;

    let mut funds = Vec::new();
    for code in fund_codes {
        match fund_api::search_fund_info(&code).await {
            Ok(fund) => funds.push(fund),
            Err(e) => {
                eprintln!("Failed to fetch fund {}: {}", code, e.details());
            }
        }
    }

    Ok(funds)
}

/// 获取列表中的基金摘要
#[tauri::command]
pub async fn get_list_fund_summaries(
    state: State<'_, Mutex<AppState>>,
    list_id: i64,
) -> Result<Vec<FundSummary>, String> {
    let fund_codes = list_manager::get_list_fund_codes(&state, list_id)
        .await
        .map_err(|e| e.user_message())?;

    let mut funds = Vec::new();
    let pool = state.lock().unwrap().pool.clone();
    for code in fund_codes {
        match fund_api::get_fund_summary(&code).await {
            Ok(mut fund) => {
                let position = match position_manager::get_group_fund_position(&pool, list_id, &code).await {
                    Ok(position) => position,
                    Err(err) => {
                        eprintln!("Failed to load holding position: {}", err.details());
                        None
                    }
                };
                let holding_amount = position.as_ref().map(|p| p.holding_amount);
                fund.holding_amount = holding_amount;
                fund.daily_change_amount = list_manager::compute_daily_change_amount(
                    &fund.daily_change_percent,
                    holding_amount,
                );
                funds.push(fund);
            }
            Err(e) => {
                eprintln!("Failed to fetch fund summary {}: {}", code, e.details());
            }
        }
    }

    Ok(funds)
}

/// 获取基金详情
#[tauri::command]
pub async fn get_list_fund_detail(
    state: State<'_, Mutex<AppState>>,
    list_id: i64,
    fund_code: String,
) -> Result<FundDetail, String> {
    let mut detail = fund_api::get_fund_detail(&fund_code)
        .await
        .map_err(|e| e.user_message())?;
    let pool = state.lock().unwrap().pool.clone();
    let position = match position_manager::get_group_fund_position(&pool, list_id, &fund_code).await {
        Ok(position) => position,
        Err(err) => {
            eprintln!("Failed to load holding position: {}", err.details());
            None
        }
    };
    if let Some(position) = position {
        detail.holding_amount = Some(position.holding_amount);
        detail.holding_shares = Some(position.holding_shares);
        if position.holding_shares > 0.0 {
            detail.cost_price = Some(position.holding_amount / position.holding_shares);
        }
        detail.daily_change_amount = list_manager::compute_daily_change_amount(
            &detail.change_percent,
            detail.holding_amount,
        );
    }
    Ok(detail)
}

/// 获取基金走势
#[tauri::command]
pub async fn get_fund_trend(code: String) -> Result<FundTrend, String> {
    fund_api::get_fund_trend(&code)
        .await
        .map_err(|e| e.user_message())
}

/// 获取累计收益率走势
#[tauri::command]
pub async fn get_fund_accum_trend(code: String) -> Result<FundTrend, String> {
    fund_api::get_fund_accum_trend(&code)
        .await
        .map_err(|e| e.user_message())
}

/// 获取持仓信息
#[tauri::command]
pub async fn get_holding(
    state: State<'_, Mutex<AppState>>,
    list_id: i64,
    fund_code: String,
) -> Result<Option<crate::models::GroupFundPosition>, String> {
    let pool = state.lock().unwrap().pool.clone();
    position_manager::get_group_fund_position(&pool, list_id, &fund_code)
        .await
        .map_err(|e| e.user_message())
}

/// 保存持仓信息
#[tauri::command]
pub async fn set_holding(
    state: State<'_, Mutex<AppState>>,
    list_id: i64,
    fund_code: String,
    holding_amount: f64,
    holding_shares: f64,
) -> Result<crate::models::GroupFundPosition, String> {
    let pool = state.lock().unwrap().pool.clone();
    position_manager::set_group_fund_position(
        &pool,
        list_id,
        &fund_code,
        holding_amount,
        holding_shares,
    )
    .await
    .map_err(|e| e.user_message())
}

/// 清空持仓信息
#[tauri::command]
pub async fn clear_holding(
    state: State<'_, Mutex<AppState>>,
    list_id: i64,
    fund_code: String,
) -> Result<(), String> {
    let pool = state.lock().unwrap().pool.clone();
    position_manager::clear_group_fund_position(&pool, list_id, &fund_code)
        .await
        .map_err(|e| e.user_message())
}

/// 获取并保存 pingzhongdata 原始数据与结构化快照
#[tauri::command]
pub async fn sync_fund_pingzhong(
    state: State<'_, Mutex<AppState>>,
    code: String,
) -> Result<(), String> {
    let payload = fund_api::fetch_pingzhong_payload(&code)
        .await
        .map_err(|e| e.user_message())?;

    let pool = state.lock().unwrap().pool.clone();
    fund_storage::save_pingzhong_payload(&pool, &code, &payload)
        .await
        .map_err(|e| e.user_message())
}

/// 重新排序列表
#[tauri::command]
pub async fn reorder_lists(
    state: State<'_, Mutex<AppState>>,
    list_ids: Vec<i64>,
) -> Result<(), String> {
    list_manager::reorder_lists(&state, list_ids)
        .await
        .map_err(|e| e.user_message())
}

/// 获取存储异常提示（如有）
#[tauri::command]
pub fn get_storage_warning(state: State<'_, Mutex<AppState>>) -> Option<String> {
    let state = state.lock().unwrap();
    state.storage_warning.clone()
}

const REFRESH_MENU_ITEMS: &[(u64, &str)] = &[
    (10_000, "refresh_10s"),
    (30_000, "refresh_30s"),
    (60_000, "refresh_60s"),
    (120_000, "refresh_120s"),
];

pub fn update_refresh_menu_selection(
    app: &AppHandle,
    interval_ms: u64,
) -> Result<(), String> {
    let window = app
        .get_window("main")
        .ok_or_else(|| "主窗口未找到".to_string())?;
    let menu = window.menu_handle();
    for (ms, id) in REFRESH_MENU_ITEMS {
        if let Some(item) = menu.try_get_item(*id) {
            item.set_selected(*ms == interval_ms)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// 设置菜单栏刷新选项
#[tauri::command]
pub fn set_refresh_interval(app: AppHandle, interval_ms: u64) -> Result<(), String> {
    update_refresh_menu_selection(&app, interval_ms)
}
