use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod discord;
mod shared;
mod twitch;
use crate::shared::app::build_app;

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
    let app = build_app();

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}
