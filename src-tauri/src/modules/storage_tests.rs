use super::storage::{save_data, load_data, validate_storage_format};
use crate::models::{FundList, UserData};
use tempfile::TempDir;

#[test]
fn test_validate_storage_format_allows_empty() {
    let data = UserData::new();
    validate_storage_format(&data).unwrap();
}

#[test]
fn test_save_and_load_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("roundtrip.json");

    let data = UserData::new();
    save_data(&path, &data).unwrap();
    let loaded = load_data(&path).unwrap();
    assert_eq!(loaded.schema_version, data.schema_version);
    assert_eq!(loaded.lists.len(), 0);
}

#[test]
fn test_validate_list_size_limit() {
    let mut data = UserData::new();
    let list = FundList {
        id: "id".to_string(),
        name: "list".to_string(),
        fund_codes: (0..201).map(|i| format!("{:06}", i)).collect(),
        created_at: 0,
        updated_at: 0,
        position: 0,
    };
    data.lists.push(list);
    assert!(validate_storage_format(&data).is_err());
}
