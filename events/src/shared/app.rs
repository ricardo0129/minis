use crate::shared;
use crate::twitch;
use axum::{Router, routing};
use tower_http::trace::TraceLayer;

pub fn build_app() -> Router {
    let app_state = shared::appstate::AppState::new();
    Router::new()
        .route("/eventsub", routing::post(twitch::routes::event_sub))
        .route("/health", routing::get(shared::routes::health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
