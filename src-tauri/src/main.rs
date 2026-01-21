// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod errors;
mod models;
mod modules;

use models::AppState;
use modules::storage;
use std::sync::Mutex;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化存储
            let storage_path =
                storage::init_storage(&app.handle()).expect("Failed to initialize storage");

            // 加载数据
            let storage_data = storage::load_data(&storage_path).unwrap_or_else(|e| {
                eprintln!("Failed to load data: {}. Starting with empty state.", e);
                models::UserData::new()
            });

            // 创建应用状态
            let app_state = AppState::new(storage_data, storage_path);

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
            commands::reorder_lists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
