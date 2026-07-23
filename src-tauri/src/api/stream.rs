use crate::errors::{AppError, AppResult};
use crate::http_server::SharedState;
use crate::models::ChatMessage;
use crate::modules::portfolio_snapshot::{self, AssetSnapshot, PortfolioSnapshot};
use crate::services::{chat_agent, llm_client, message_service, session_service};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, Sse};
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::BTreeSet;
use std::convert::Infallible;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use uuid::Uuid;

type EventSender = mpsc::Sender<Result<Event, Infallible>>;
const MAX_CONVERSATION_MESSAGES: usize = 16;
const MAX_CONVERSATION_CHARS: usize = 16_000;

#[derive(Debug, Deserialize)]
pub struct UserMessageInput {
    pub content: String,
    #[serde(default)]
    pub context: MessageContext,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageContext {
    pub snapshot_id: Option<String>,
    #[serde(default)]
    pub asset_codes: Vec<String>,
    pub context_type: Option<String>,
}

pub async fn stream_message(
    State(state): State<SharedState>,
    Path(session_id): Path<String>,
    Json(payload): Json<UserMessageInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let content = payload.content.trim().to_string();
    if content.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "消息不能为空".to_string()));
    }
    eprintln!(
        "[AI_CHAT] ====== 新消息请求 ======\n[AI_CHAT] session_id: {}\n[AI_CHAT] 用户输入: {}",
        session_id, content
    );
    session_service::ensure_session_exists(&state.pool, &session_id)
        .await
        .map_err(to_http_error)?;

    let context = normalize_context(payload.context)?;
    let snapshot = resolve_snapshot(&state.pool, context.snapshot_id.as_deref())
        .await
        .map_err(to_http_error)?;
    validate_asset_scope(&snapshot, &context).map_err(to_http_error)?;
    let context_json = serde_json::to_string(&context)
        .map_err(AppError::from)
        .map_err(to_http_error)?;
    message_service::save_user_message(
        &state.pool,
        &session_id,
        &content,
        Some(&snapshot.id),
        Some(&context_json),
    )
    .await
    .map_err(to_http_error)?;
    session_service::touch_session(&state.pool, &session_id)
        .await
        .map_err(to_http_error)?;

    let config = state
        .llm_config
        .read()
        .map_err(|_| to_http_error(AppError::StorageError("读取模型配置锁失败".to_string())))?
        .clone();
    eprintln!(
        "[AI_CHAT] provider: {}, model: {}, base_url: {}, max_tokens: {}, temperature: {}",
        config.provider.as_str(),
        config.model,
        config.base_url,
        config.max_tokens,
        config.temperature
    );
    let conversation = message_service::list_messages(&state.pool, &session_id, None)
        .await
        .map(build_conversation_messages)
        .map_err(to_http_error)?;
    eprintln!("[AI_CHAT] 对话历史: {} 条消息", conversation.len());
    let (tx, rx) = mpsc::channel(32);
    let pool = state.pool.clone();
    tokio::spawn(async move {
        if let Err(error) = send_stream(
            pool,
            session_id,
            conversation,
            context,
            snapshot,
            config,
            tx,
        )
        .await
        {
            eprintln!("stream error: {}", error.details());
        }
    });
    Ok(Sse::new(ReceiverStream::new(rx)))
}

async fn send_stream(
    pool: SqlitePool,
    session_id: String,
    conversation: Vec<llm_client::LlmMessage>,
    context: MessageContext,
    snapshot: PortfolioSnapshot,
    config: llm_client::LlmConfig,
    tx: EventSender,
) -> AppResult<()> {
    let context_metadata = serde_json::json!({
        "snapshot_id": snapshot.id,
        "data_as_of": snapshot.snapshot_at,
        "context_type": context.context_type.as_deref().unwrap_or("portfolio"),
        "asset_codes": context.asset_codes,
    });
    if !emit(&tx, "context", context_metadata.to_string()).await {
        return Ok(());
    }

    let uses_portfolio_context = conversation_uses_portfolio_context(&conversation);
    let model_context = uses_portfolio_context
        .then(|| build_model_context(&snapshot, &context, &config))
        .transpose()?;
    if let Err(error) = record_request_audit(&pool, &session_id, &snapshot, &context, &config).await
    {
        eprintln!("save request audit failed: {}", error.details());
    }
    eprintln!(
        "[AI_CHAT] portfolio_context: {}",
        if model_context.is_some() { "有" } else { "无" }
    );
    let mut full_reply = String::new();
    let mut failure = None;
    match chat_agent::stream_reply(&config, conversation, model_context, &pool).await {
        Ok(mut stream) => {
            while let Some(item) = stream.next().await {
                match item {
                    Ok(llm_client::LlmEvent::TextChunk(chunk)) if !chunk.is_empty() => {
                        full_reply.push_str(&chunk);
                        if !emit(&tx, "chunk", chunk).await {
                            return Ok(());
                        }
                    }
                    Ok(llm_client::LlmEvent::ToolCallComplete { name, .. }) => {
                        let msg = format!("正在查询: {}", name);
                        let _ = emit(&tx, "tool_call", msg).await;
                    }
                    Ok(_) => {}
                    Err(error) => {
                        failure = Some(error);
                        break;
                    }
                }
            }
            if full_reply.trim().is_empty() && failure.is_none() {
                failure = Some(AppError::NetworkError("模型返回空响应".to_string()));
            }
        }
        Err(error) => failure = Some(error),
    }

    eprintln!(
        "[AI_CHAT] ====== AI 回复 ======\n[AI_CHAT] 回复长度: {} 字符\n[AI_CHAT] AI 回复内容:\n{}",
        full_reply.len(),
        full_reply
    );

    if let Some(error) = failure {
        eprintln!("[AI_CHAT] LLM 失败: {}", error.details());
        eprintln!("LLM unavailable: {}", error.details());
        let (fallback, fallback_kind) = if uses_portfolio_context {
            (
                build_local_summary(&snapshot, &context),
                "local_snapshot_summary",
            )
        } else {
            (
                "AI 模型暂时不可用，请检查模型设置后重试。".to_string(),
                "unavailable_notice",
            )
        };
        let separator = if full_reply.is_empty() {
            String::new()
        } else if uses_portfolio_context {
            "\n\n---\n\n模型连接中断，以下为固定快照的本地摘要。\n\n".to_string()
        } else {
            "\n\n".to_string()
        };
        let fallback_chunk = format!("{separator}{fallback}");
        full_reply.push_str(&fallback_chunk);
        if !emit(&tx, "chunk", fallback_chunk).await {
            return Ok(());
        }
        let error_payload = serde_json::json!({
            "code": "llm_unavailable",
            "message": error.details(),
            "fallback": fallback_kind
        });
        if !emit(&tx, "error", error_payload.to_string()).await {
            return Ok(());
        }
    }

    let saved_state =
        match message_service::save_assistant_message(&pool, &session_id, &full_reply, "saved")
            .await
        {
            Ok(_) => "saved",
            Err(error) => {
                eprintln!("save assistant message failed: {}", error.details());
                "unsaved"
            }
        };
    let _ = session_service::touch_session(&pool, &session_id).await;
    if !emit(&tx, "saved_state", saved_state.to_string()).await {
        return Ok(());
    }
    emit(&tx, "done", "done".to_string()).await;
    Ok(())
}

fn build_conversation_messages(messages: Vec<ChatMessage>) -> Vec<llm_client::LlmMessage> {
    let mut selected = Vec::new();
    let mut total_chars = 0;

    for message in messages.into_iter().rev() {
        if message.role != "user" && message.role != "assistant" {
            continue;
        }
        let content = message.content.trim();
        if content.is_empty() {
            continue;
        }
        if message.role == "assistant" && content.starts_with("## 本地组合摘要") {
            continue;
        }
        let content_chars = content.chars().count();
        if !selected.is_empty()
            && (selected.len() >= MAX_CONVERSATION_MESSAGES
                || total_chars + content_chars > MAX_CONVERSATION_CHARS)
        {
            break;
        }
        total_chars += content_chars;
        selected.push(if message.role == "user" {
            llm_client::LlmMessage::user(content)
        } else {
            llm_client::LlmMessage::assistant(content)
        });
    }

    selected.reverse();
    selected
}

fn conversation_uses_portfolio_context(conversation: &[llm_client::LlmMessage]) -> bool {
    let Some(message) = conversation
        .iter()
        .rev()
        .find(|message| message.role == "user")
    else {
        return false;
    };
    let content = message.content.as_deref().unwrap_or("").to_lowercase();
    [
        "投资",
        "组合",
        "持仓",
        "资产",
        "基金",
        "股票",
        "黄金",
        "白银",
        "比特币",
        "btc",
        "以太坊",
        "eth",
        "市值",
        "净值",
        "收益",
        "盈亏",
        "涨跌",
        "涨幅",
        "跌幅",
        "风险",
        "集中度",
        "配置",
        "仓位",
        "行情",
        "回撤",
    ]
    .iter()
    .any(|keyword| content.contains(keyword))
}

fn normalize_context(mut context: MessageContext) -> Result<MessageContext, (StatusCode, String)> {
    context.snapshot_id = context
        .snapshot_id
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    context.asset_codes = context
        .asset_codes
        .into_iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let context_type =
        context
            .context_type
            .as_deref()
            .unwrap_or(if context.asset_codes.is_empty() {
                "portfolio"
            } else {
                "asset"
            });
    if !matches!(context_type, "portfolio" | "asset" | "report") {
        return Err((StatusCode::BAD_REQUEST, "不支持的上下文类型".to_string()));
    }
    if context_type == "asset" && context.asset_codes.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "资产上下文必须包含 asset_codes".to_string(),
        ));
    }
    context.context_type = Some(context_type.to_string());
    Ok(context)
}

async fn resolve_snapshot(
    pool: &SqlitePool,
    requested_id: Option<&str>,
) -> AppResult<PortfolioSnapshot> {
    if let Some(id) = requested_id {
        return portfolio_snapshot::get_snapshot_by_id(pool, id)
            .await?
            .ok_or_else(|| AppError::ValidationError(format!("组合快照不存在: {id}")));
    }
    match portfolio_snapshot::get_latest_snapshot(pool).await? {
        Some(snapshot) => Ok(snapshot),
        None => portfolio_snapshot::create_full_snapshot(pool).await,
    }
}

fn validate_asset_scope(snapshot: &PortfolioSnapshot, context: &MessageContext) -> AppResult<()> {
    if context.asset_codes.is_empty() {
        return Ok(());
    }
    let known: BTreeSet<_> = snapshot
        .assets
        .iter()
        .map(|asset| asset.code.as_str())
        .collect();
    let missing: Vec<_> = context
        .asset_codes
        .iter()
        .filter(|code| !known.contains(code.as_str()))
        .cloned()
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(AppError::ValidationError(format!(
            "指定资产不在该快照中: {}",
            missing.join(", ")
        )))
    }
}

fn selected_assets<'a>(
    snapshot: &'a PortfolioSnapshot,
    context: &MessageContext,
) -> Vec<&'a AssetSnapshot> {
    if context.asset_codes.is_empty() {
        return snapshot.assets.iter().collect();
    }
    let selected: BTreeSet<_> = context.asset_codes.iter().map(String::as_str).collect();
    snapshot
        .assets
        .iter()
        .filter(|asset| selected.contains(asset.code.as_str()))
        .collect()
}

fn build_model_context(
    snapshot: &PortfolioSnapshot,
    context: &MessageContext,
    config: &llm_client::LlmConfig,
) -> AppResult<String> {
    let assets = selected_assets(snapshot, context);
    let local_mode = is_local_model(config);
    let asset_values: Vec<_> = assets
        .iter()
        .map(|asset| {
            if local_mode {
                serde_json::json!({
                    "code": asset.code,
                    "name": asset.name,
                    "category": asset.category.as_str(),
                    "current_value": asset.holding_amount,
                    "cost_amount": asset.cost_amount,
                    "change_percent": asset.change_percent,
                    "daily_change_amount": asset.daily_change_amount,
                    "valuation_basis": asset.valuation_basis,
                    "quote_time": asset.update_time,
                    "data_complete": asset.data_complete,
                })
            } else {
                serde_json::json!({
                    "code": asset.code,
                    "name": asset.name,
                    "category": asset.category.as_str(),
                    "portfolio_percent": percent_of(asset.holding_amount, snapshot.total_value),
                    "change_percent": asset.change_percent,
                    "daily_contribution_percent": percent_of(asset.daily_change_amount.unwrap_or(0.0), snapshot.total_value),
                    "valuation_basis": asset.valuation_basis,
                    "quote_time": asset.update_time,
                    "data_complete": asset.data_complete,
                })
            }
        })
        .collect();
    let deterministic_metrics = if local_mode {
        serde_json::json!({
            "portfolio_total_value": snapshot.total_value,
            "portfolio_daily_change_amount": snapshot.daily_change_amount,
            "portfolio_daily_change_percent": snapshot.daily_change_percent,
            "daily_change_coverage_percent": snapshot.daily_change_coverage_percent,
            "max_single_percent": snapshot.concentration.max_single_percent,
            "top5_percent": snapshot.concentration.top5_percent,
        })
    } else {
        serde_json::json!({
            "portfolio_daily_change_percent": snapshot.daily_change_percent,
            "daily_change_coverage_percent": snapshot.daily_change_coverage_percent,
            "max_single_percent": snapshot.concentration.max_single_percent,
            "top5_percent": snapshot.concentration.top5_percent,
        })
    };
    let value = serde_json::json!({
        "protocol": "leek_fund_snapshot_v1",
        "snapshot_id": snapshot.id,
        "data_as_of": snapshot.snapshot_at,
        "scope": context.context_type.as_deref().unwrap_or("portfolio"),
        "data_mode": if local_mode { "local" } else { "redacted_cloud" },
        "deterministic_metrics": deterministic_metrics,
        "assets": asset_values,
        "unknowns": snapshot.data_quality.gaps,
        "rules": [
            "关键数字是 Rust 基于固定快照计算的结果，不得覆盖或重新计算",
            "本地模式中的 current_value 是当前市值，cost_amount 是用户录入成本；脱敏云模式不提供精确金额",
            "没有资讯来源，不能把具体新闻事件写成涨跌原因"
        ]
    });
    serde_json::to_string_pretty(&value).map_err(AppError::from)
}

fn build_local_summary(snapshot: &PortfolioSnapshot, context: &MessageContext) -> String {
    let assets = selected_assets(snapshot, context);
    let data_as_of = DateTime::<Utc>::from_timestamp(snapshot.snapshot_at, 0)
        .map(|value| value.to_rfc3339())
        .unwrap_or_else(|| snapshot.snapshot_at.to_string());
    let mut output = format!(
        "## 本地组合摘要\n\nAI 解读暂不可用。以下内容完全来自固定快照 `{}`，数据时间 {}。\n\n",
        snapshot.id, data_as_of
    );
    output.push_str("### 计算结果\n\n");
    output.push_str(&format!(
        "- 当前总市值：{:.2} 元\n- 已覆盖资产今日变动：{:.2} 元（{:.4}%）\n- 今日涨跌覆盖率：{:.2}%\n- 最大单项占比：{:.2}%（{}）\n- 前五项占比：{:.2}%\n",
        snapshot.total_value,
        snapshot.daily_change_amount,
        snapshot.daily_change_percent,
        snapshot.daily_change_coverage_percent,
        snapshot.concentration.max_single_percent,
        snapshot.concentration.max_single_name,
        snapshot.concentration.top5_percent,
    ));
    output.push_str("\n### 当前上下文资产\n\n");
    if assets.is_empty() {
        output.push_str("- 无持仓资产\n");
    } else {
        for asset in assets {
            let change = asset
                .daily_change_amount
                .map(|value| format!("{value:.2} 元"))
                .unwrap_or_else(|| "未知".to_string());
            output.push_str(&format!(
                "- {}（{}）：当前市值 {:.2} 元，今日影响 {}\n",
                asset.name, asset.code, asset.holding_amount, change
            ));
        }
    }
    output.push_str("\n### 未知与缺口\n\n");
    if snapshot.data_quality.gaps.is_empty() {
        output.push_str("- 当前快照未发现持仓份额或行情缺口。\n");
    } else {
        for gap in &snapshot.data_quality.gaps {
            output.push_str(&format!("- {gap}\n"));
        }
    }
    output.push_str("- 当前未接入可靠资讯来源，因此不判断具体新闻或事件原因。\n");
    output
}

async fn record_request_audit(
    pool: &SqlitePool,
    session_id: &str,
    snapshot: &PortfolioSnapshot,
    context: &MessageContext,
    config: &llm_client::LlmConfig,
) -> AppResult<()> {
    let local_mode = is_local_model(config);
    let asset_fields = if local_mode {
        serde_json::json!([
            "code",
            "name",
            "category",
            "current_value",
            "cost_amount",
            "change_percent",
            "daily_change_amount",
            "valuation_basis",
            "quote_time",
            "data_complete"
        ])
    } else {
        serde_json::json!([
            "code",
            "name",
            "category",
            "portfolio_percent",
            "change_percent",
            "daily_contribution_percent",
            "valuation_basis",
            "quote_time",
            "data_complete"
        ])
    };
    let fields = serde_json::json!({
        "snapshot_fields": ["id", "snapshot_at", "deterministic_metrics", "data_gaps"],
        "asset_fields": asset_fields,
        "asset_codes": context.asset_codes,
    });
    let data_mode = if local_mode {
        "local"
    } else {
        "redacted_cloud"
    };
    sqlx::query(
        "INSERT INTO ai_request_audits \
         (id, session_id, snapshot_id, provider, data_mode, fields_sent, created_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(Uuid::new_v4().to_string())
    .bind(session_id)
    .bind(&snapshot.id)
    .bind(config.provider.as_str())
    .bind(data_mode)
    .bind(fields.to_string())
    .bind(Utc::now().timestamp())
    .execute(pool)
    .await
    .map_err(|error| AppError::StorageError(format!("保存模型出站审计失败: {error}")))?;
    Ok(())
}

fn is_local_model(config: &llm_client::LlmConfig) -> bool {
    if config.provider != llm_client::LlmProvider::OpenAICompatible {
        return false;
    }
    let base_url = config.base_url.to_ascii_lowercase();
    base_url.starts_with("http://127.0.0.1")
        || base_url.starts_with("http://localhost")
        || base_url.starts_with("http://[::1]")
}

fn percent_of(value: f64, total: f64) -> f64 {
    if total > 0.0 {
        (value / total * 1_000_000.0).round() / 10_000.0
    } else {
        0.0
    }
}

async fn emit(tx: &EventSender, event: &str, data: String) -> bool {
    tx.send(Ok(Event::default().event(event).data(data)))
        .await
        .is_ok()
}

fn to_http_error(error: AppError) -> (StatusCode, String) {
    let status = match error {
        AppError::ChatSessionNotFound(_) => StatusCode::NOT_FOUND,
        AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, error.user_message())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::portfolio_snapshot::{
        AssetCategory, ConcentrationMetrics, DataQuality, ValuationBasis,
    };

    fn snapshot(id: &str, value: f64) -> PortfolioSnapshot {
        PortfolioSnapshot {
            id: id.to_string(),
            snapshot_at: 1,
            total_value: value,
            daily_change_amount: 0.0,
            daily_change_percent: 0.0,
            daily_change_coverage_percent: 100.0,
            assets: vec![AssetSnapshot {
                code: "A".to_string(),
                name: "资产A".to_string(),
                category: AssetCategory::Fund,
                cost_amount: value,
                holding_amount: value,
                holding_quantity: 1.0,
                current_price: Some(value),
                change_percent: Some(0.0),
                daily_change_amount: Some(0.0),
                valuation_basis: ValuationBasis::Quote,
                group_name: None,
                update_time: None,
                data_complete: true,
            }],
            allocation: Vec::new(),
            top_movers: Vec::new(),
            concentration: ConcentrationMetrics {
                max_single_percent: 100.0,
                max_single_name: "资产A".to_string(),
                top5_percent: 100.0,
                top5_names: vec!["资产A".to_string()],
            },
            data_quality: DataQuality {
                total_assets: 1,
                complete_assets: 1,
                missing_holding: 0,
                missing_quote: 0,
                quote_coverage_percent: 100.0,
                freshness: "无可用行情时间".to_string(),
                gaps: Vec::new(),
            },
        }
    }

    fn chat_message(id: i64, role: &str, content: &str) -> ChatMessage {
        ChatMessage {
            id,
            session_id: "session".to_string(),
            role: role.to_string(),
            content: content.to_string(),
            saved_state: Some("saved".to_string()),
            created_at: id,
            updated_at: id,
        }
    }

    #[test]
    fn conversation_history_keeps_recent_messages_in_order() {
        let messages = (0..20)
            .map(|id| {
                chat_message(
                    id,
                    if id % 2 == 0 { "user" } else { "assistant" },
                    &format!("message-{id}"),
                )
            })
            .collect();
        let history = build_conversation_messages(messages);
        assert_eq!(history.len(), MAX_CONVERSATION_MESSAGES);
        assert_eq!(history.first().unwrap().content.as_deref(), Some("message-4"));
        assert_eq!(history.last().unwrap().content.as_deref(), Some("message-19"));
    }

    #[test]
    fn portfolio_context_is_only_used_for_investment_questions() {
        let general = vec![llm_client::LlmMessage::user("刚才的测试暗号是什么？")];
        let investment = vec![llm_client::LlmMessage::user("分析一下我的持仓集中度")];

        assert!(!conversation_uses_portfolio_context(&general));
        assert!(conversation_uses_portfolio_context(&investment));
    }

    #[tokio::test]
    async fn requested_snapshot_wins_over_latest_snapshot() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query(
            "CREATE TABLE portfolio_snapshots (id TEXT PRIMARY KEY, snapshot_at INTEGER NOT NULL, payload TEXT NOT NULL, data_quality TEXT NOT NULL, created_at INTEGER NOT NULL)",
        )
        .execute(&pool)
        .await
        .unwrap();
        for (created_at, value) in [
            (1, snapshot("fixed", 100.0)),
            (2, snapshot("latest", 200.0)),
        ] {
            sqlx::query("INSERT INTO portfolio_snapshots VALUES (?, ?, ?, '{}', ?)")
                .bind(&value.id)
                .bind(value.snapshot_at)
                .bind(serde_json::to_string(&value).unwrap())
                .bind(created_at)
                .execute(&pool)
                .await
                .unwrap();
        }
        let resolved = resolve_snapshot(&pool, Some("fixed")).await.unwrap();
        assert_eq!(resolved.id, "fixed");
        assert_eq!(resolved.total_value, 100.0);
    }

    #[test]
    fn fallback_contains_snapshot_and_local_metrics() {
        let value = snapshot("fixed", 100.0);
        let summary = build_local_summary(&value, &MessageContext::default());
        assert!(summary.contains("fixed"));
        assert!(summary.contains("100.00 元"));
        assert!(summary.contains("不判断具体新闻或事件原因"));
    }

    #[test]
    fn remote_context_redacts_exact_amounts() {
        let value = snapshot("fixed", 100.0);
        let config = llm_client::LlmConfig {
            provider: llm_client::LlmProvider::OpenAI,
            api_key: Some("secret".to_string()),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "model".to_string(),
            max_tokens: 100,
            temperature: 0.5,
        };
        let context = build_model_context(&value, &MessageContext::default(), &config).unwrap();
        let context: serde_json::Value = serde_json::from_str(&context).unwrap();
        assert_eq!(context["data_mode"], "redacted_cloud");
        assert!(context["assets"][0].get("current_value").is_none());
        assert_eq!(context["assets"][0]["portfolio_percent"], 100.0);
        assert!(context["deterministic_metrics"]
            .get("portfolio_total_value")
            .is_none());
    }
}
