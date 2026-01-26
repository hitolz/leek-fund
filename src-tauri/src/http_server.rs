use crate::api::{message, session, stream};
use axum::routing::{get, post};
use axum::Router;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tokio::net::TcpListener;

pub struct ChatApiState {
    pub pool: SqlitePool,
}

pub type SharedState = Arc<ChatApiState>;

pub fn build_router(state: SharedState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/sessions", post(session::create_session))
        .route("/api/sessions/recent", get(session::get_recent_session))
        .route(
            "/api/sessions/:session_id/messages",
            get(message::list_messages),
        )
        .route(
            "/api/sessions/:session_id/messages/stream",
            post(stream::stream_message),
        )
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
