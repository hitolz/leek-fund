use crate::errors::AppError;
use crate::http_server::SharedState;
use crate::services::session_service;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn create_session(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let session = session_service::get_or_create_recent_session(&state.pool)
        .await
        .map_err(to_http_error)?;

    Ok((StatusCode::CREATED, Json(session)))
}

pub async fn get_recent_session(
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let session = session_service::get_recent_session(&state.pool)
        .await
        .map_err(to_http_error)?;

    match session {
        Some(session) => Ok((StatusCode::OK, Json(session)).into_response()),
        None => Ok((StatusCode::NOT_FOUND, "no session".to_string()).into_response()),
    }
}

fn to_http_error(err: AppError) -> (StatusCode, String) {
    let status = match err {
        AppError::ChatSessionNotFound(_) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (status, err.user_message())
}
