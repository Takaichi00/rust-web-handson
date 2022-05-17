use axum::{http::StatusCode, response::IntoResponse};

pub async fn hc() -> impl IntoResponse {
    tracing::debug!("Access health check endpoint from user!");
    (StatusCode::OK, "ok")
}
