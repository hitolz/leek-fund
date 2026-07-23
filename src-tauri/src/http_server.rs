use crate::api::{message, session, stream};
use crate::services::llm_client;
use axum::http::{header, HeaderValue, Method};
use axum::routing::{get, post};
use axum::Router;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

const TRUSTED_ORIGINS: &[&str] = &[
    "http://localhost:1420",
    "http://127.0.0.1:1420",
    "tauri://localhost",
    "http://tauri.localhost",
    "https://tauri.localhost",
];

pub struct ChatApiState {
    pub pool: SqlitePool,
    pub llm_config: Arc<RwLock<llm_client::LlmConfig>>,
    pub app_data_dir: PathBuf,
}

pub type SharedState = Arc<ChatApiState>;

pub fn build_router(state: SharedState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(
            TRUSTED_ORIGINS
                .iter()
                .map(|origin| HeaderValue::from_static(origin))
                .collect::<Vec<_>>(),
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::ACCEPT]);

    Router::new()
        .route("/api/sessions", post(session::create_session))
        .route("/api/sessions/list", get(session::list_sessions))
        .route("/api/sessions/recent", get(session::get_recent_session))
        .route(
            "/api/sessions/:session_id/messages",
            get(message::list_messages),
        )
        .route(
            "/api/sessions/:session_id/messages/stream",
            post(stream::stream_message),
        )
        .route("/api/llm/config", get(get_llm_config).post(set_llm_config))
        .route("/api/llm/test", post(test_llm_connection))
        .route("/api/portfolio/snapshot", get(get_portfolio_snapshot))
        .with_state(state)
        .layer(cors)
}

pub async fn start_server(state: SharedState, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let router = build_router(state);

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("AI chat server bind failed: {}", err);
            return;
        }
    };

    if let Err(err) = axum::serve(listener, router).await {
        eprintln!("AI chat server failed: {}", err);
    }
}

// ============================================================================
// LLM 配置 API
// ============================================================================

use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};

async fn get_llm_config(State(state): State<SharedState>) -> Json<serde_json::Value> {
    let config = state
        .llm_config
        .read()
        .map(|value| value.clone())
        .unwrap_or_default();
    Json(serde_json::json!({
        "provider": config.provider.as_str(),
        "base_url": config.base_url,
        "model": config.model,
        "max_tokens": config.max_tokens,
        "temperature": config.temperature,
        "has_api_key": config.api_key.is_some(),
    }))
}

#[derive(Deserialize)]
struct SetLlmConfigInput {
    provider: String,
    api_key: Option<String>,
    base_url: String,
    model: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Serialize)]
struct SetLlmConfigResponse {
    success: bool,
    message: String,
}

async fn set_llm_config(
    State(state): State<SharedState>,
    Json(payload): Json<SetLlmConfigInput>,
) -> Json<SetLlmConfigResponse> {
    let current = match state.llm_config.read() {
        Ok(config) => config.clone(),
        Err(_) => {
            return config_response(false, "读取当前模型配置失败");
        }
    };
    let config = match merge_config(&current, payload) {
        Ok(config) => config,
        Err(error) => return config_response(false, &error.user_message()),
    };
    if let Err(error) = llm_client::save_config(&state.app_data_dir, &config) {
        return config_response(false, &format!("保存配置失败: {}", error.user_message()));
    }
    match state.llm_config.write() {
        Ok(mut active_config) => *active_config = config,
        Err(_) => return config_response(false, "配置已保存，但更新运行时配置失败"),
    }
    config_response(true, "配置已保存并立即生效")
}

fn merge_config(
    current: &llm_client::LlmConfig,
    payload: SetLlmConfigInput,
) -> crate::errors::AppResult<llm_client::LlmConfig> {
    let provider = llm_client::LlmProvider::parse(&payload.provider)?;
    let submitted_key = payload
        .api_key
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let api_key = submitted_key.or_else(|| {
        (provider == current.provider)
            .then(|| current.api_key.clone())
            .flatten()
    });
    llm_client::LlmConfig {
        provider,
        api_key,
        base_url: payload.base_url,
        model: payload.model,
        max_tokens: payload.max_tokens.unwrap_or(current.max_tokens),
        temperature: payload.temperature.unwrap_or(current.temperature),
    }
    .validate()
}

fn config_response(success: bool, message: &str) -> Json<SetLlmConfigResponse> {
    Json(SetLlmConfigResponse {
        success,
        message: message.to_string(),
    })
}

#[derive(Deserialize)]
struct TestLlmInput {
    provider: String,
    api_key: Option<String>,
    base_url: String,
    model: String,
}

async fn test_llm_connection(
    State(state): State<SharedState>,
    Json(payload): Json<TestLlmInput>,
) -> Json<serde_json::Value> {
    let current = match state.llm_config.read() {
        Ok(config) => config.clone(),
        Err(_) => {
            return Json(serde_json::json!({
                "success": false,
                "message": "读取当前模型配置失败"
            }))
        }
    };
    let config = match merge_config(
        &current,
        SetLlmConfigInput {
            provider: payload.provider,
            api_key: payload.api_key,
            base_url: payload.base_url,
            model: payload.model,
            max_tokens: Some(512),
            temperature: Some(0.7),
        },
    ) {
        Ok(config) => config,
        Err(error) => {
            return Json(serde_json::json!({
                "success": false,
                "message": error.user_message()
            }))
        }
    };

    match llm_client::test_connection(&config).await {
        Ok(response) => Json(serde_json::json!({
            "success": true,
            "message": format!("连接成功: {}", response)
        })),
        Err(e) => Json(serde_json::json!({
            "success": false,
            "message": format!("连接失败: {}", e.details())
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blank_api_key_keeps_existing_secret() {
        let current = llm_client::LlmConfig {
            provider: llm_client::LlmProvider::OpenAI,
            api_key: Some("secret".to_string()),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "model".to_string(),
            max_tokens: 100,
            temperature: 0.5,
        };
        let merged = merge_config(
            &current,
            SetLlmConfigInput {
                provider: "openai".to_string(),
                api_key: Some("   ".to_string()),
                base_url: current.base_url.clone(),
                model: current.model.clone(),
                max_tokens: None,
                temperature: None,
            },
        )
        .unwrap();
        assert_eq!(merged.api_key.as_deref(), Some("secret"));
    }

    #[test]
    fn compatible_config_accepts_no_existing_secret() {
        let current = llm_client::LlmConfig::default();
        let merged = merge_config(
            &current,
            SetLlmConfigInput {
                provider: "ollama".to_string(),
                api_key: Some(String::new()),
                base_url: "http://127.0.0.1:11434/v1".to_string(),
                model: "qwen".to_string(),
                max_tokens: None,
                temperature: None,
            },
        )
        .unwrap();
        assert_eq!(merged.api_key, None);
    }

    #[test]
    fn provider_change_does_not_reuse_existing_secret() {
        let current = llm_client::LlmConfig {
            provider: llm_client::LlmProvider::OpenAI,
            api_key: Some("openai-secret".to_string()),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "model".to_string(),
            max_tokens: 100,
            temperature: 0.5,
        };
        let result = merge_config(
            &current,
            SetLlmConfigInput {
                provider: "claude".to_string(),
                api_key: Some(String::new()),
                base_url: "https://api.anthropic.com/v1".to_string(),
                model: "claude-model".to_string(),
                max_tokens: None,
                temperature: None,
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn cors_origins_are_explicit_and_cover_tauri_and_dev() {
        assert!(TRUSTED_ORIGINS.contains(&"http://localhost:1420"));
        assert!(TRUSTED_ORIGINS.contains(&"tauri://localhost"));
        assert!(TRUSTED_ORIGINS.contains(&"https://tauri.localhost"));
        assert!(!TRUSTED_ORIGINS.contains(&"*"));
    }
}

// ============================================================================
// 组合快照 API
// ============================================================================

async fn get_portfolio_snapshot(State(state): State<SharedState>) -> Json<serde_json::Value> {
    use crate::modules::portfolio_snapshot;

    // 尝试获取最新快照
    match portfolio_snapshot::get_latest_snapshot(&state.pool).await {
        Ok(Some(snapshot)) => Json(serde_json::json!({
            "success": true,
            "snapshot": snapshot
        })),
        Ok(None) => {
            // 没有快照，创建一个
            match portfolio_snapshot::create_full_snapshot(&state.pool).await {
                Ok(snapshot) => Json(serde_json::json!({
                    "success": true,
                    "snapshot": snapshot
                })),
                Err(e) => Json(serde_json::json!({
                    "success": false,
                    "message": format!("创建快照失败: {}", e)
                })),
            }
        }
        Err(e) => Json(serde_json::json!({
            "success": false,
            "message": format!("读取快照失败: {}", e)
        })),
    }
}
