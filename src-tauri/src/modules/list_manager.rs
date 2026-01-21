use crate::errors::{AppError, AppResult};
use crate::models::{AppState, FundList};
use std::sync::Mutex;
use uuid::Uuid;

/// 创建新列表
pub fn create_list(state: &Mutex<AppState>, name: String) -> AppResult<FundList> {
    let mut state = state.lock().unwrap();

    // 验证列表名称
    if !FundList::validate_name(&name) {
        return Err(AppError::ValidationError(
            "列表名称不能为空且不能超过30个字符".to_string(),
        ));
    }

    // 检查名称唯一性
    if state.storage.lists.iter().any(|l| l.name == name) {
        return Err(AppError::DuplicateListName(name));
    }

    // 检查列表数量限制
    if state.storage.lists.len() >= 50 {
        return Err(AppError::ValidationError(
            "已达到最大列表数量限制(50个)".to_string(),
        ));
    }

    // 创建新列表
    let list = FundList {
        id: Uuid::new_v4().to_string(),
        name,
        fund_codes: Vec::new(),
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
        position: state.storage.lists.len(),
    };

    state.storage.lists.push(list.clone());
    state.storage.touch();

    Ok(list)
}

/// 重命名列表
pub fn rename_list(state: &Mutex<AppState>, id: String, new_name: String) -> AppResult<()> {
    let mut state = state.lock().unwrap();

    // 验证新名称
    if !FundList::validate_name(&new_name) {
        return Err(AppError::ValidationError(
            "列表名称不能为空且不能超过30个字符".to_string(),
        ));
    }
    let all_lists = state.storage.lists.clone();
    // 查找列表
    let list = state
        .storage
        .lists
        .iter_mut()
        .find(|l| l.id == id)
        .ok_or_else(|| AppError::ListNotFound(id.clone()))?;

    // 检查新名称是否与其他列表冲突（排除自己）
    if all_lists.iter().any(|l| l.id != id && l.name == new_name) {
        return Err(AppError::DuplicateListName(new_name));
    }

    list.name = new_name;
    list.updated_at = chrono::Utc::now().timestamp();
    state.storage.touch();

    Ok(())
}

/// 删除列表
pub fn delete_list(state: &Mutex<AppState>, id: String) -> AppResult<()> {
    let mut state = state.lock().unwrap();

    // 查找并删除列表
    let index = state
        .storage
        .lists
        .iter()
        .position(|l| l.id == id)
        .ok_or_else(|| AppError::ListNotFound(id))?;

    state.storage.lists.remove(index);

    // 重新调整后续列表的 position
    for (i, list) in state.storage.lists.iter_mut().enumerate() {
        list.position = i;
    }

    state.storage.touch();

    Ok(())
}

/// 添加基金到列表
pub fn add_fund_to_list(
    state: &Mutex<AppState>,
    list_id: String,
    fund_code: String,
) -> AppResult<()> {
    let mut state = state.lock().unwrap();

    // 验证基金代码
    if !crate::models::FundInfo::validate_code(&fund_code) {
        return Err(AppError::ValidationError("无效的基金代码格式".to_string()));
    }

    // 查找列表
    let list = state
        .storage
        .lists
        .iter_mut()
        .find(|l| l.id == list_id)
        .ok_or_else(|| AppError::ListNotFound(list_id))?;

    // 检查是否已存在（去重）
    if list.contains_fund(&fund_code) {
        return Err(AppError::DuplicateFund(fund_code));
    }

    // 检查列表容量
    if list.fund_codes.len() >= 200 {
        return Err(AppError::ValidationError(
            "列表已达到最大基金数量(200个)".to_string(),
        ));
    }

    // 添加基金
    list.fund_codes.push(fund_code);
    list.updated_at = chrono::Utc::now().timestamp();
    state.storage.touch();

    Ok(())
}

/// 从列表中移除基金
pub fn remove_fund_from_list(
    state: &Mutex<AppState>,
    list_id: String,
    fund_code: String,
) -> AppResult<()> {
    let mut state = state.lock().unwrap();

    // 查找列表
    let list = state
        .storage
        .lists
        .iter_mut()
        .find(|l| l.id == list_id)
        .ok_or_else(|| AppError::ListNotFound(list_id))?;

    // 查找并移除基金
    let index = list
        .fund_codes
        .iter()
        .position(|c| c == &fund_code)
        .ok_or_else(|| AppError::ValidationError("基金不在此列表中".to_string()))?;

    list.fund_codes.remove(index);
    list.updated_at = chrono::Utc::now().timestamp();
    state.storage.touch();

    Ok(())
}

/// 重新排序列表
pub fn reorder_lists(state: &Mutex<AppState>, list_ids: Vec<String>) -> AppResult<()> {
    let mut state = state.lock().unwrap();

    // 验证所有 ID 都存在
    if list_ids.len() != state.storage.lists.len() {
        return Err(AppError::ValidationError("列表ID不完整".to_string()));
    }

    for id in &list_ids {
        if !state.storage.lists.iter().any(|l| &l.id == id) {
            return Err(AppError::ValidationError(format!("无效的列表ID: {}", id)));
        }
    }

    // 创建新的排序
    let mut new_lists = Vec::new();
    let now = chrono::Utc::now().timestamp();
    for (i, id) in list_ids.iter().enumerate() {
        let mut list = state
            .storage
            .lists
            .iter()
            .find(|l| &l.id == id)
            .unwrap()
            .clone();
        list.position = i;
        list.updated_at = now;
        new_lists.push(list);
    }

    state.storage.lists = new_lists;
    state.storage.touch();

    Ok(())
}

/// 获取所有列表（已按 position 排序）
pub fn get_all_lists(state: &Mutex<AppState>) -> Vec<FundList> {
    let state = state.lock().unwrap();
    let mut lists = state.storage.lists.clone();
    lists.sort_by_key(|l| l.position);
    lists
}

/// 获取列表中的基金代码
pub fn get_list_fund_codes(state: &Mutex<AppState>, list_id: String) -> AppResult<Vec<String>> {
    let state = state.lock().unwrap();
    let list = state
        .storage
        .lists
        .iter()
        .find(|l| l.id == list_id)
        .ok_or_else(|| AppError::ListNotFound(list_id))?;

    Ok(list.fund_codes.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::UserData;
    use std::path::PathBuf;

    fn create_test_state() -> Mutex<AppState> {
        Mutex::new(AppState::new(
            UserData::new(),
            PathBuf::from("/tmp/test.json"),
        ))
    }

    #[test]
    fn test_create_list() {
        let state = create_test_state();
        let list = create_list(&state, "测试列表".to_string()).unwrap();

        assert_eq!(list.name, "测试列表");
        assert_eq!(list.fund_codes.len(), 0);
        assert_eq!(list.position, 0);

        let lists = get_all_lists(&state);
        assert_eq!(lists.len(), 1);
    }

    #[test]
    fn test_duplicate_list_name() {
        let state = create_test_state();
        create_list(&state, "测试列表".to_string()).unwrap();

        let result = create_list(&state, "测试列表".to_string());
        assert!(matches!(result, Err(AppError::DuplicateListName(_))));
    }

    #[test]
    fn test_add_fund_to_list() {
        let state = create_test_state();
        let list = create_list(&state, "测试列表".to_string()).unwrap();

        add_fund_to_list(&state, list.id.clone(), "001632".to_string()).unwrap();

        let lists = get_all_lists(&state);
        assert_eq!(lists[0].fund_codes.len(), 1);
        assert_eq!(lists[0].fund_codes[0], "001632");
    }

    #[test]
    fn test_duplicate_fund_in_same_list() {
        let state = create_test_state();
        let list = create_list(&state, "测试列表".to_string()).unwrap();

        add_fund_to_list(&state, list.id.clone(), "001632".to_string()).unwrap();
        let result = add_fund_to_list(&state, list.id.clone(), "001632".to_string());

        assert!(matches!(result, Err(AppError::DuplicateFund(_))));
    }

    #[test]
    fn test_same_fund_in_different_lists() {
        let state = create_test_state();
        let list1 = create_list(&state, "列表1".to_string()).unwrap();
        let list2 = create_list(&state, "列表2".to_string()).unwrap();

        add_fund_to_list(&state, list1.id.clone(), "001632".to_string()).unwrap();
        add_fund_to_list(&state, list2.id.clone(), "001632".to_string()).unwrap();

        let lists = get_all_lists(&state);
        assert_eq!(lists[0].fund_codes[0], "001632");
        assert_eq!(lists[1].fund_codes[0], "001632");
    }

    #[test]
    fn test_remove_fund_from_list() {
        let state = create_test_state();
        let list = create_list(&state, "测试列表".to_string()).unwrap();

        add_fund_to_list(&state, list.id.clone(), "001632".to_string()).unwrap();
        remove_fund_from_list(&state, list.id.clone(), "001632".to_string()).unwrap();

        let lists = get_all_lists(&state);
        assert_eq!(lists[0].fund_codes.len(), 0);
    }

    #[test]
    fn test_delete_list() {
        let state = create_test_state();
        let list = create_list(&state, "测试列表".to_string()).unwrap();

        delete_list(&state, list.id).unwrap();

        let lists = get_all_lists(&state);
        assert_eq!(lists.len(), 0);
    }

    #[test]
    fn test_rename_list() {
        let state = create_test_state();
        let list = create_list(&state, "旧名称".to_string()).unwrap();

        rename_list(&state, list.id.clone(), "新名称".to_string()).unwrap();

        let lists = get_all_lists(&state);
        assert_eq!(lists[0].name, "新名称");
    }
}
