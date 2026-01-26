use crate::errors::{AppError, AppResult};
use crate::http_server::SharedState;
use crate::services::{chat_agent, message_service, session_service};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, Sse};
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use std::convert::Infallible;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

#[derive(Deserialize)]
pub struct UserMessageInput {
    pub content: String,
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

    session_service::ensure_session_exists(&state.pool, &session_id)
        .await
        .map_err(to_http_error)?;

    message_service::save_user_message(&state.pool, &session_id, &content)
        .await
        .map_err(to_http_error)?;

    session_service::touch_session(&state.pool, &session_id)
        .await
        .map_err(to_http_error)?;

    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(32);
    let pool = state.pool.clone();
    let session_id_clone = session_id.clone();

    tokio::spawn(async move {
        if let Err(err) = send_stream(pool, session_id_clone, content, tx).await {
            eprintln!("stream error: {}", err.details());
        }
    });

    Ok(Sse::new(ReceiverStream::new(rx)))
}

async fn send_stream(
    pool: sqlx::SqlitePool,
    session_id: String,
    content: String,
    tx: mpsc::Sender<Result<Event, Infallible>>,
) -> AppResult<()> {
    let mut full_reply = String::new();
    let mut stream = chat_agent::stream_reply(content).await?;

    while let Some(chunk) = stream.next().await {
        full_reply.push_str(&chunk);
        let _ = tx
            .send(Ok(Event::default().event("chunk").data(chunk)))
            .await;
    }

    let mut saved_state = "saved";
    if let Err(err) = message_service::save_assistant_message(&pool, &session_id, &full_reply, "saved").await {
        eprintln!("save assistant message failed: {}", err.details());
        saved_state = "unsaved";
    }

    let _ = tx
        .send(Ok(Event::default().event("saved_state").data(saved_state)))
        .await;
    let _ = tx.send(Ok(Event::default().event("done").data("done"))).await;

    session_service::touch_session(&pool, &session_id).await?;

    Ok(())
}

fn to_http_error(err: AppError) -> (StatusCode, String) {
    let status = match err {
        AppError::ChatSessionNotFound(_) => StatusCode::NOT_FOUND,
        AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, err.user_message())
}
