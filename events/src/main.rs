use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod discord;
mod kick;
mod shared;
mod twitch;
use crate::shared::app::build_router;
use crate::shared::event::InternalEvent;
use crate::shared::event_router::NotificationRouter;
use tokio::sync::mpsc::channel;

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
    let (sender, mut receiver) = channel(100);
    let app_state = shared::appstate::AppState::new(sender).await;
    tokio::spawn(async move {
        let notification_router = NotificationRouter::new();
        loop {
            let event: InternalEvent = receiver.recv().await.expect("Unable to read from channel");
            info!("Processing: {:?}", event);
            notification_router.route_event(event).await;
        }
    });
    let app = build_router(app_state);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}
