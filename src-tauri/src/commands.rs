use crate::errors::AppError;
use crate::models::{AppState, FundDetail, FundInfo, FundList, FundSummary, FundTrend};
use crate::modules::{fund_api, list_manager, storage};
use std::sync::Mutex;
use tauri::State;

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
pub fn get_all_lists(state: State<Mutex<AppState>>) -> Result<Vec<FundList>, String> {
    Ok(list_manager::get_all_lists(&state))
}

/// 创建新列表
#[tauri::command]
pub fn create_list(state: State<Mutex<AppState>>, name: String) -> Result<FundList, String> {
    let list = list_manager::create_list(&state, name).map_err(|e| e.user_message())?;

    // 保存到存储
    save_state(&state)?;

    Ok(list)
}

/// 重命名列表
#[tauri::command]
pub fn rename_list(
    state: State<Mutex<AppState>>,
    id: String,
    new_name: String,
) -> Result<(), String> {
    list_manager::rename_list(&state, id, new_name).map_err(|e| e.user_message())?;

    // 保存到存储
    save_state(&state)?;

    Ok(())
}

/// 删除列表
#[tauri::command]
pub fn delete_list(state: State<Mutex<AppState>>, id: String) -> Result<(), String> {
    list_manager::delete_list(&state, id).map_err(|e| e.user_message())?;

    // 保存到存储
    save_state(&state)?;

    Ok(())
}

/// 添加基金到列表
#[tauri::command]
pub fn add_fund_to_list(
    state: State<Mutex<AppState>>,
    list_id: String,
    fund_code: String,
) -> Result<(), String> {
    list_manager::add_fund_to_list(&state, list_id, fund_code).map_err(|e| e.user_message())?;

    // 保存到存储
    save_state(&state)?;

    Ok(())
}

/// 从列表中移除基金
#[tauri::command]
pub fn remove_fund_from_list(
    state: State<Mutex<AppState>>,
    list_id: String,
    fund_code: String,
) -> Result<(), String> {
    list_manager::remove_fund_from_list(&state, list_id, fund_code)
        .map_err(|e| e.user_message())?;

    // 保存到存储
    save_state(&state)?;

    Ok(())
}

/// 获取列表中的所有基金详情
#[tauri::command]
pub async fn get_list_funds(
    state: State<'_, Mutex<AppState>>,
    list_id: String,
) -> Result<Vec<FundInfo>, String> {
    // 获取列表中的基金代码
    let fund_codes =
        list_manager::get_list_fund_codes(&state, list_id).map_err(|e| e.user_message())?;

    // 并发查询所有基金信息
    let mut funds = Vec::new();
    for code in fund_codes {
        match fund_api::search_fund_info(&code).await {
            Ok(fund) => funds.push(fund),
            Err(e) => {
                eprintln!("Failed to fetch fund {}: {}", code, e.details());
                // 继续查询其他基金，不中断整个流程
            }
        }
    }

    Ok(funds)
}

/// 获取列表中的基金摘要
#[tauri::command]
pub async fn get_list_fund_summaries(
    state: State<'_, Mutex<AppState>>,
    list_id: String,
) -> Result<Vec<FundSummary>, String> {
    let fund_codes =
        list_manager::get_list_fund_codes(&state, list_id).map_err(|e| e.user_message())?;

    let mut funds = Vec::new();
    for code in fund_codes {
        match fund_api::get_fund_summary(&code).await {
            Ok(fund) => funds.push(fund),
            Err(e) => {
                eprintln!("Failed to fetch fund summary {}: {}", code, e.details());
            }
        }
    }

    Ok(funds)
}

/// 获取基金详情
#[tauri::command]
pub async fn get_fund_detail(code: String) -> Result<FundDetail, String> {
    fund_api::get_fund_detail(&code)
        .await
        .map_err(|e| e.user_message())
}

/// 获取基金走势
#[tauri::command]
pub async fn get_fund_trend(code: String) -> Result<FundTrend, String> {
    fund_api::get_fund_trend(&code)
        .await
        .map_err(|e| e.user_message())
}

/// 重新排序列表
#[tauri::command]
pub fn reorder_lists(state: State<Mutex<AppState>>, list_ids: Vec<String>) -> Result<(), String> {
    list_manager::reorder_lists(&state, list_ids).map_err(|e| e.user_message())?;

    // 保存到存储
    save_state(&state)?;

    Ok(())
}

/// 辅助函数：保存状态到存储
fn save_state(state: &State<Mutex<AppState>>) -> Result<(), String> {
    let state = state.lock().unwrap();
    storage::save_data(&state.storage_path, &state.storage).map_err(|e| e.user_message())
}
