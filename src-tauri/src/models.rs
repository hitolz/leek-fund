use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::PathBuf;

/// 基金信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundInfo {
    /// 基金代码（6位数字）
    pub code: String,
    /// 基金名称
    pub name: String,
    /// 当前净值
    pub net_value: Option<f64>,
    /// 当日涨跌幅（百分比字符串）
    pub change_percent: Option<String>,
    /// 更新时间
    pub update_time: Option<String>,
}

impl FundInfo {
    /// 验证基金代码格式（必须是6位数字）
    pub fn validate_code(code: &str) -> bool {
        code.len() == 6 && code.chars().all(|c| c.is_ascii_digit())
    }
}

/// 基金摘要信息（用于列表视图）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundSummary {
    /// 基金代码（6位数字）
    pub code: String,
    /// 基金名称
    pub name: String,
    /// 当日涨跌幅（百分比字符串）
    pub daily_change_percent: Option<String>,
    /// 当日涨跌额（基于持仓计算）
    pub daily_change_amount: Option<f64>,
    /// 持仓金额
    pub holding_amount: Option<f64>,
    /// 更新时间
    pub update_time: Option<String>,
}

/// 基金详情信息（用于详情视图）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundDetail {
    /// 基金代码（6位数字）
    pub code: String,
    /// 基金名称
    pub name: String,
    /// 当前净值
    pub net_value: Option<f64>,
    /// 当日涨跌幅（百分比字符串）
    pub change_percent: Option<String>,
    /// 当日涨跌额（基于持仓计算）
    pub daily_change_amount: Option<f64>,
    /// 更新时间
    pub update_time: Option<String>,
    /// 持仓金额
    pub holding_amount: Option<f64>,
    /// 持仓份额
    pub holding_shares: Option<f64>,
    /// 成本价（持仓金额/持仓份额）
    pub cost_price: Option<f64>,
}

/// 基金走势点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendPoint {
    pub date: String,
    pub value: f64,
}

/// 基金走势
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundTrend {
    pub code: String,
    pub window: String,
    pub points: Vec<TrendPoint>,
}

/// 基金列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundList {
    /// 列表ID（自增整型）
    pub id: i64,
    /// 列表名称（用户自定义，1-30字符，唯一）
    pub name: String,
    /// 基金代码列表（有序，列表内唯一）
    pub fund_codes: Vec<String>,
    /// 创建时间（Unix timestamp）
    pub created_at: i64,
    /// 更新时间（Unix timestamp）
    #[serde(default)]
    pub updated_at: i64,
    /// 显示位置（用于排序）
    pub position: i64,
}

/// 分组-基金持仓信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupFundPosition {
    pub list_id: i64,
    pub fund_code: String,
    pub holding_amount: f64,
    pub holding_shares: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl FundList {
    /// 检查列表中是否包含指定基金
    pub fn contains_fund(&self, fund_code: &str) -> bool {
        self.fund_codes.iter().any(|code| code == fund_code)
    }

    /// 验证列表名称（1-64字符，不能全是空格）
    pub fn validate_name(name: &str) -> bool {
        let trimmed = name.trim();
        !trimmed.is_empty() && trimmed.len() <= 64
    }
}

/// 用户数据（根数据结构）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    /// 数据格式版本（当前为1）
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    /// 所有用户列表
    pub lists: Vec<FundList>,
    /// 首次创建时间
    pub created_at: i64,
    /// 最后修改时间
    pub last_modified: i64,
    /// 最后迁移时间
    #[serde(default)]
    pub last_migrated_at: Option<i64>,
    /// 预留偏好设置
    #[serde(default)]
    pub preferences: Option<serde_json::Value>,
}

fn default_schema_version() -> u32 {
    1
}

impl UserData {
    /// 创建新的空用户数据
    pub fn new() -> Self {
        let now = chrono::Utc::now().timestamp();
        UserData {
            schema_version: 1,
            lists: Vec::new(),
            created_at: now,
            last_modified: now,
            last_migrated_at: None,
            preferences: None,
        }
    }

    /// 更新最后修改时间
    pub fn touch(&mut self) {
        self.last_modified = chrono::Utc::now().timestamp();
    }
}

impl Default for UserData {
    fn default() -> Self {
        Self::new()
    }
}

/// 应用状态
#[derive(Debug)]
pub struct AppState {
    /// SQLite 连接池
    pub pool: SqlitePool,
    /// SQLite 文件路径
    pub db_path: PathBuf,
    /// 旧版 JSON 存储文件路径
    pub legacy_json_path: PathBuf,
    /// 存储异常提示
    pub storage_warning: Option<String>,
}

impl AppState {
    pub fn new(
        pool: SqlitePool,
        db_path: PathBuf,
        legacy_json_path: PathBuf,
        storage_warning: Option<String>,
    ) -> Self {
        AppState {
            pool,
            db_path,
            legacy_json_path,
            storage_warning,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fund_code_validation() {
        assert!(FundInfo::validate_code("001632"));
        assert!(FundInfo::validate_code("123456"));
        assert!(!FundInfo::validate_code("12345")); // 太短
        assert!(!FundInfo::validate_code("1234567")); // 太长
        assert!(!FundInfo::validate_code("12345a")); // 包含字母
        assert!(!FundInfo::validate_code("")); // 空字符串
    }

    #[test]
    fn test_list_name_validation() {
        assert!(FundList::validate_name("成长型基金"));
        assert!(FundList::validate_name("a"));
        assert!(FundList::validate_name("0123456789012345678901234567890")); // 31字符
        assert!(!FundList::validate_name("")); // 空
        assert!(!FundList::validate_name("   ")); // 全空格
        assert!(!FundList::validate_name(
            "0123456789012345678901234567890123456789012345678901234567890123"
        )); // 65字符
    }

    #[test]
    fn test_fund_list_contains() {
        let list = FundList {
            id: 1,
            name: "测试列表".to_string(),
            fund_codes: vec!["001632".to_string(), "014938".to_string()],
            created_at: 0,
            updated_at: 0,
            position: 0,
        };

        assert!(list.contains_fund("001632"));
        assert!(list.contains_fund("014938"));
        assert!(!list.contains_fund("999999"));
    }

    #[test]
    fn test_user_data_creation() {
        let storage = UserData::new();
        assert_eq!(storage.schema_version, 1);
        assert!(storage.lists.is_empty());
        assert!(storage.created_at > 0);
        assert_eq!(storage.created_at, storage.last_modified);
    }
}
