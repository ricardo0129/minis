use crate::kick;
use crate::shared;
use crate::shared::appstate::AppState;
use crate::twitch;
use axum::{Router, routing};
use tower_http::trace::TraceLayer;

pub fn build_router(app_state: AppState) -> Router {
    Router::new()
        .route("/twitch/eventsub", routing::post(twitch::routes::event_sub))
        .route("/kick/webhook", routing::post(kick::routes::webhook))
        .route("/health", routing::get(shared::routes::health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}
