use super::list_manager::{add_fund_to_list, create_list, rename_list};
use crate::models::AppState;
use crate::models::UserData;
use std::path::PathBuf;
use std::sync::Mutex;

fn create_test_state() -> Mutex<AppState> {
    Mutex::new(AppState::new(
        UserData::new(),
        PathBuf::from("/tmp/test.json"),
    ))
}

#[test]
fn test_updated_at_changes_on_rename() {
    let state = create_test_state();
    let list = create_list(&state, "测试列表".to_string()).unwrap();
    let before = list.updated_at;
    rename_list(&state, list.id.clone(), "新名称".to_string()).unwrap();

    let state_guard = state.lock().unwrap();
    let updated = state_guard
        .storage
        .lists
        .iter()
        .find(|l| l.id == list.id)
        .unwrap();
    assert!(updated.updated_at >= before);
}

#[test]
fn test_updated_at_changes_on_add_fund() {
    let state = create_test_state();
    let list = create_list(&state, "测试列表".to_string()).unwrap();
    let before = list.updated_at;
    add_fund_to_list(&state, list.id.clone(), "001632".to_string()).unwrap();

    let state_guard = state.lock().unwrap();
    let updated = state_guard
        .storage
        .lists
        .iter()
        .find(|l| l.id == list.id)
        .unwrap();
    assert!(updated.updated_at >= before);
}
