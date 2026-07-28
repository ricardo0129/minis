use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod discord;
mod kick;
mod shared;
mod twitch;
use crate::shared::app::build_router;
use crate::shared::event_router::NotificationRouter;
use crate::shared::event_router::pull_events;
use tokio::time::{Duration, sleep};

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
    let app_state = shared::appstate::AppState::new().await;
    let events = app_state.events.clone();
    tokio::spawn(async move {
        let notification_router = NotificationRouter::new();
        loop {
            sleep(Duration::from_secs(1)).await;
            info!("loop running");
            let process_events = pull_events(&mut events.lock().unwrap());
            for e in process_events {
                info!("{:?}", e);
                notification_router.route_event(e).await;
            }
        }
    });
    let app = build_router(app_state);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}
