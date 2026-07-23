use axum::{
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use reqwest::Client;
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

const FUND_REALTIME_BASE: &str = "https://fundcomapi.tiantianfunds.com";
const FUND_PINGZHONG_BASE: &str = "https://fund.eastmoney.com";

#[derive(Clone)]
struct AppState {
    http: Client,
}

#[derive(Deserialize)]
struct CodeParam {
    code: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let http = Client::builder()
        .no_proxy()
        .build()
        .expect("failed to build HTTP client");

    let state = Arc::new(AppState { http });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/fund/realtime/{code}", get(proxy_fund_realtime))
        .route("/api/fund/pingzhong/{code}", get(proxy_fund_pingzhong))
        .route("/api/health", get(health))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3100);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("leek-fund-proxy listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}

async fn proxy_fund_realtime(
    State(state): State<Arc<AppState>>,
    Path(params): Path<CodeParam>,
) -> Response {
    if !is_valid_fund_code(&params.code) {
        return (StatusCode::BAD_REQUEST, Json("invalid fund code")).into_response();
    }

    let url = format!(
        "{}/mm/newCore/FundValuationLast?FCODES={}&FIELDS=FCODE,SHORTNAME,GSZZL,GZTIME,GSZ,NAV,PDATE",
        FUND_REALTIME_BASE, params.code
    );
    proxy_get(&state.http, &url, "application/json").await
}

async fn proxy_fund_pingzhong(
    State(state): State<Arc<AppState>>,
    Path(params): Path<CodeParam>,
) -> Response {
    if !is_valid_fund_code(&params.code) {
        return (StatusCode::BAD_REQUEST, Json("invalid fund code")).into_response();
    }

    let url = format!("{}/pingzhongdata/{}.js", FUND_PINGZHONG_BASE, params.code);
    proxy_get(&state.http, &url, "application/javascript").await
}

fn is_valid_fund_code(code: &str) -> bool {
    code.len() == 6 && code.chars().all(|c| c.is_ascii_digit())
}

async fn proxy_get(client: &Client, url: &str, content_type: &str) -> Response {
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status();
            let body = match resp.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    tracing::error!("failed to read upstream body: {}", e);
                    return (StatusCode::BAD_GATEWAY, "upstream read error").into_response();
                }
            };

            let mut headers = HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_str(content_type).unwrap(),
            );
            headers.insert(
                axum::http::header::CACHE_CONTROL,
                HeaderValue::from_static("no-cache"),
            );

            (
                StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::OK),
                headers,
                body.to_vec(),
            )
                .into_response()
        }
        Err(e) => {
            tracing::error!("upstream request failed: {}", e);
            (StatusCode::BAD_GATEWAY, "upstream unavailable").into_response()
        }
    }
}
