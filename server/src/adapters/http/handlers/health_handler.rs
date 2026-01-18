use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "status": "ok", "message": "Service is healthy" })),
    )
}

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}
