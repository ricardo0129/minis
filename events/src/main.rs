use axum::{Router, routing};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod discord;
mod twitch;

#[tokio::main]
async fn main() {
    // build our application with a route
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let twitch_state = twitch::appstate::AppState::new();
    let app = Router::new()
        .route("/eventsub", routing::post(twitch::routes::event_sub))
        .with_state(twitch_state);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}
