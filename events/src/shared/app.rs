use crate::kick;
use crate::shared;
use crate::twitch;
use axum::{Router, routing};
use tower_http::trace::TraceLayer;

pub async fn build_app() -> Router {
    let app_state = shared::appstate::AppState::new().await;
    Router::new()
        .route("/eventsub", routing::post(twitch::routes::event_sub))
        .route("/health", routing::get(shared::routes::health_check))
        .route("/kick/webhook", routing::post(kick::routes::webhook))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
