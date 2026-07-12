use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn health_check() -> Result<impl IntoResponse, (StatusCode, String)> {
    Ok((StatusCode::OK, "".to_string()))
}
