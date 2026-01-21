use std::fmt;

/// 应用错误类型
#[derive(Debug)]
pub enum AppError {
    /// 网络错误
    NetworkError(String),
    /// 解析错误
    ParseError(String),
    /// 基金不存在
    NotFound(String),
    /// 基金详情不可用
    FundDetailUnavailable(String),
    /// 基金走势不可用
    FundTrendUnavailable(String),
    /// 重复的基金
    DuplicateFund(String),
    /// 存储错误
    StorageError(String),
    /// 验证错误
    ValidationError(String),
    /// 列表不存在
    ListNotFound(String),
    /// 列表名称冲突
    DuplicateListName(String),
}

impl AppError {
    /// 获取用户友好的中文错误消息
    pub fn user_message(&self) -> String {
        match self {
            AppError::NetworkError(_) => "网络连接失败，请检查网络后重试".to_string(),
            AppError::ParseError(_) => "数据解析失败，请稍后重试".to_string(),
            AppError::NotFound(code) => format!("基金代码 {} 不存在", code),
            AppError::FundDetailUnavailable(code) => format!("基金 {} 详情不可用", code),
            AppError::FundTrendUnavailable(code) => format!("基金 {} 暂无走势数据", code),
            AppError::DuplicateFund(code) => format!("基金 {} 已在列表中", code),
            AppError::StorageError(_) => "数据保存失败，请重试".to_string(),
            AppError::ValidationError(msg) => msg.clone(),
            AppError::ListNotFound(_) => "列表不存在".to_string(),
            AppError::DuplicateListName(_) => "列表名称已存在，请使用其他名称".to_string(),
        }
    }

    /// 获取技术详情（用于日志）
    pub fn details(&self) -> &str {
        match self {
            AppError::NetworkError(e) => e,
            AppError::ParseError(e) => e,
            AppError::NotFound(e) => e,
            AppError::FundDetailUnavailable(e) => e,
            AppError::FundTrendUnavailable(e) => e,
            AppError::DuplicateFund(e) => e,
            AppError::StorageError(e) => e,
            AppError::ValidationError(e) => e,
            AppError::ListNotFound(e) => e,
            AppError::DuplicateListName(e) => e,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.user_message())
    }
}

impl std::error::Error for AppError {}

/// From implementations for common error types
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::NetworkError("请求超时".to_string())
        } else if err.is_connect() {
            AppError::NetworkError("连接失败".to_string())
        } else {
            AppError::NetworkError(format!("网络错误: {}", err))
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::ParseError(format!("JSON解析错误: {}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::StorageError(format!("IO错误: {}", err))
    }
}

/// Result type alias for application errors
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_user_messages() {
        let errors = vec![
            (
                AppError::NetworkError("test".to_string()),
                "网络连接失败，请检查网络后重试",
            ),
            (
                AppError::NotFound("001632".to_string()),
                "基金代码 001632 不存在",
            ),
            (
                AppError::FundDetailUnavailable("001632".to_string()),
                "基金 001632 详情不可用",
            ),
            (
                AppError::FundTrendUnavailable("001632".to_string()),
                "基金 001632 暂无走势数据",
            ),
            (
                AppError::DuplicateFund("001632".to_string()),
                "基金 001632 已在列表中",
            ),
            (
                AppError::StorageError("test".to_string()),
                "数据保存失败，请重试",
            ),
            (AppError::ListNotFound("id".to_string()), "列表不存在"),
            (
                AppError::DuplicateListName("name".to_string()),
                "列表名称已存在，请使用其他名称",
            ),
        ];

        for (error, expected_msg) in errors {
            assert_eq!(error.user_message(), expected_msg);
        }
    }
}
