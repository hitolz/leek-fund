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
            let app_state = AppState::new(pool, db_path, legacy_json_path, warning);

            // 管理状态
            app.manage(Mutex::new(app_state));

            Ok(())
        })
        .on_menu_event(|event| {
            let id = event.menu_item_id();
            if let Some((interval_ms, _, _)) =
                REFRESH_MENU_ITEMS.iter().find(|(_, item_id, _)| *item_id == id)
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
        ])
        .run(context)
        .expect("error while running tauri application");
}
