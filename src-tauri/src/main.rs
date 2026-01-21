// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod errors;
mod models;
mod modules;
mod migrations;

use models::AppState;
use modules::storage;
use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化存储
            let (pool, db_path, legacy_json_path, warning) =
                tauri::async_runtime::block_on(storage::init_storage(&app.handle()))
                    .expect("Failed to initialize storage");

            // 创建应用状态
            let app_state = AppState::new(pool, db_path, legacy_json_path, warning);

            // 管理状态
            app.manage(Mutex::new(app_state));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::search_fund,
            commands::get_all_lists,
            commands::create_list,
            commands::rename_list,
            commands::delete_list,
            commands::add_fund_to_list,
            commands::remove_fund_from_list,
            commands::get_list_funds,
            commands::get_list_fund_summaries,
            commands::get_fund_detail,
            commands::get_fund_trend,
            commands::get_fund_accum_trend,
            commands::sync_fund_pingzhong,
            commands::get_storage_warning,
            commands::reorder_lists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
