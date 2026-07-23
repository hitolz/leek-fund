// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod api;
mod commands;
mod db;
mod errors;
mod http_server;
mod migrations;
mod models;
mod modules;
mod services;

use http_server::ChatApiState;
use models::AppState;
use modules::storage;
use services::llm_client;
use std::sync::{Arc, Mutex, RwLock};
use tauri::{CustomMenuItem, Manager, Menu, Submenu};

const REFRESH_MENU_ITEMS: &[(u64, &str, &str)] = &[
    (10_000, "refresh_10s", "10s"),
    (30_000, "refresh_30s", "30s"),
    (60_000, "refresh_60s", "60s"),
    (120_000, "refresh_120s", "120s"),
];

fn build_menu(app_name: &str) -> Menu {
    let mut refresh_menu = Menu::new();
    for (ms, id, label) in REFRESH_MENU_ITEMS {
        let mut item = CustomMenuItem::new(*id, *label);
        if *ms == 10_000 {
            item = item.selected();
        }
        refresh_menu = refresh_menu.add_item(item);
    }

    let refresh_submenu = Submenu::new("刷新", refresh_menu);
    Menu::os_default(app_name).add_submenu(refresh_submenu)
}

fn main() {
    let context = tauri::generate_context!();
    let menu = build_menu(&context.package_info().name);

    tauri::Builder::default()
        .menu(menu)
        .setup(|app| {
            // 初始化存储
            let (pool, db_path, legacy_json_path, warning) =
                tauri::async_runtime::block_on(storage::init_storage(&app.handle()))
                    .expect("Failed to initialize storage");

            // 创建应用状态
            let app_state = AppState::new(pool.clone(), db_path, legacy_json_path, warning);

            // LLM 配置存储在数据库同目录 (~/.leek/)
            let llm_config_dir = app_state.db_path.parent()
                .expect("数据库路径无效")
                .to_path_buf();
            let llm_config = llm_client::load_config(&llm_config_dir);

            // 管理状态
            app.manage(Mutex::new(app_state));

            // 启动 AI 聊天 HTTP 服务器
            let chat_state = Arc::new(ChatApiState {
                pool,
                llm_config: Arc::new(RwLock::new(llm_config)),
                app_data_dir: llm_config_dir,
            });
            tauri::async_runtime::spawn(http_server::start_server(chat_state, 18188));

            Ok(())
        })
        .on_menu_event(|event| {
            let id = event.menu_item_id();
            if let Some((interval_ms, _, _)) = REFRESH_MENU_ITEMS
                .iter()
                .find(|(_, item_id, _)| *item_id == id)
            {
                let app = event.window().app_handle();
                if commands::update_refresh_menu_selection(&app, *interval_ms).is_ok() {
                    let _ = app.emit_all("refresh-interval-selected", *interval_ms);
                }
            }
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
            commands::get_list_fund_detail,
            commands::get_fund_trend,
            commands::get_fund_accum_trend,
            commands::sync_fund_pingzhong,
            commands::get_storage_warning,
            commands::reorder_lists,
            commands::get_holding,
            commands::set_holding,
            commands::clear_holding,
            commands::set_refresh_interval,
            // 股票命令
            commands::search_stock,
            commands::get_stock_quote,
            // 加密货币命令
            commands::get_crypto_quote,
            commands::get_crypto_quotes,
            commands::get_popular_cryptos,
            // 股票持仓命令
            commands::get_stock_holding,
            commands::set_stock_holding,
            commands::clear_stock_holding,
            // 加密货币持仓命令
            commands::get_crypto_holding,
            commands::set_crypto_holding,
            commands::clear_crypto_holding,
            // 黄金命令
            commands::get_gold_quote,
            commands::get_gold_holding,
            commands::set_gold_holding,
            commands::clear_gold_holding,
            // AI 投资驾驶舱命令
            commands::get_portfolio_snapshot,
            commands::refresh_portfolio_snapshot,
        ])
        .run(context)
        .expect("error while running tauri application");
}
