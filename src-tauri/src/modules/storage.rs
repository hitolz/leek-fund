use crate::errors::{AppError, AppResult};
use crate::models::UserData;
use std::fs;
use std::path::{Path, PathBuf};

/// 初始化存储目录
pub fn init_storage(app_handle: &tauri::AppHandle) -> AppResult<PathBuf> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or_else(|| AppError::StorageError("无法获取应用数据目录".to_string()))?;

    // 创建目录
    fs::create_dir_all(&app_data_dir)?;

    // 返回存储文件路径
    Ok(app_data_dir.join("lists.json"))
}

/// 从文件加载数据
pub fn load_data(path: &Path) -> AppResult<UserData> {
    // 如果文件不存在，返回新的空数据
    if !path.exists() {
        return Ok(UserData::new());
    }

    // 读取文件内容
    let content = fs::read_to_string(path)
        .map_err(|e| AppError::StorageError(format!("读取文件失败: {}", e)))?;

    // 解析 JSON
    let data: UserData = serde_json::from_str(&content).map_err(|e| {
        // 如果解析失败，备份损坏的文件
        let backup_path = get_backup_path(path);
        let _ = fs::rename(path, &backup_path);
        eprintln!("数据文件损坏，已备份到: {}", backup_path.display());

        AppError::StorageError(format!("数据文件损坏，已备份。使用新的空数据。错误: {}", e))
    })?;

    // 验证数据版本
    if data.schema_version != 1 {
        return Err(AppError::StorageError(format!(
            "不支持的数据格式版本: {}",
            data.schema_version
        )));
    }

    Ok(data)
}

/// 保存数据到文件（原子写入）
pub fn save_data(path: &Path, data: &UserData) -> AppResult<()> {
    // 序列化为 JSON
    let json = serde_json::to_string_pretty(data)?;

    // 写入临时文件
    let temp_path = path.with_extension("tmp");
    fs::write(&temp_path, json)
        .map_err(|e| AppError::StorageError(format!("写入临时文件失败: {}", e)))?;

    // 原子性地重命名文件（确保数据完整性）
    fs::rename(&temp_path, path)
        .map_err(|e| AppError::StorageError(format!("保存文件失败: {}", e)))?;

    Ok(())
}

/// 获取备份文件路径
fn get_backup_path(path: &Path) -> PathBuf {
    let timestamp = chrono::Utc::now().timestamp();
    path.with_extension(format!("backup.{}.json", timestamp))
}

/// 验证存储格式
pub fn validate_storage_format(data: &UserData) -> AppResult<()> {
    // 检查版本
    if data.schema_version != 1 {
        return Err(AppError::ValidationError(format!(
            "不支持的版本: {}",
            data.schema_version
        )));
    }

    // 检查列表数量
    if data.lists.len() > 50 {
        return Err(AppError::ValidationError(
            "列表数量超过限制(50个)".to_string(),
        ));
    }

    // 检查列表名称唯一性
    let mut names = std::collections::HashSet::new();
    for list in &data.lists {
        if !names.insert(&list.name) {
            return Err(AppError::ValidationError(format!(
                "列表名称重复: {}",
                list.name
            )));
        }
    }

    // 检查每个列表的基金代码
    for list in &data.lists {
        if list.fund_codes.len() > 200 {
            return Err(AppError::ValidationError(format!(
                "列表 '{}' 的基金数量超过限制(200个)",
                list.name
            )));
        }

        // 检查列表内基金代码唯一性
        let mut codes = std::collections::HashSet::new();
        for code in &list.fund_codes {
            if !codes.insert(code) {
                return Err(AppError::ValidationError(format!(
                    "列表 '{}' 中基金代码重复: {}",
                    list.name, code
                )));
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.json");

        let data = UserData::new();
        save_data(&path, &data).unwrap();

        let loaded = load_data(&path).unwrap();
        assert_eq!(loaded.schema_version, data.schema_version);
        assert_eq!(loaded.lists.len(), 0);
    }

    #[test]
    fn test_atomic_write() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.json");

        let mut data = UserData::new();
        save_data(&path, &data).unwrap();

        // 再次保存
        data.touch();
        save_data(&path, &data).unwrap();

        // 确保临时文件被清理
        let temp_path = path.with_extension("tmp");
        assert!(!temp_path.exists());
    }

    #[test]
    fn test_load_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("nonexistent.json");

        let data = load_data(&path).unwrap();
        assert_eq!(data.lists.len(), 0);
    }

    #[test]
    fn test_corrupted_file_backup() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("corrupted.json");

        // 写入无效的 JSON
        fs::write(&path, "invalid json content").unwrap();

        // 加载应该失败，但文件应该被备份
        let result = load_data(&path);
        assert!(result.is_err());

        // 原文件应该被移走
        assert!(!path.exists());

        // 应该有备份文件
        let backup_files: Vec<_> = fs::read_dir(temp_dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_str()
                    .unwrap()
                    .starts_with("corrupted.backup")
            })
            .collect();
        assert!(!backup_files.is_empty());
    }
}
