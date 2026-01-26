use crate::errors::AppError;
use crate::http_server::SharedState;
use crate::services::{message_service, session_service};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MessageQuery {
    pub limit: Option<i64>,
}

pub async fn list_messages(
    State(state): State<SharedState>,
    Path(session_id): Path<String>,
    Query(query): Query<MessageQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    session_service::ensure_session_exists(&state.pool, &session_id)
        .await
        .map_err(to_http_error)?;

    let messages = message_service::list_messages(&state.pool, &session_id, query.limit)
        .await
        .map_err(to_http_error)?;

    Ok((StatusCode::OK, Json(messages)))
}

fn to_http_error(err: AppError) -> (StatusCode, String) {
    let status = match err {
        AppError::ChatSessionNotFound(_) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, err.user_message())
}
