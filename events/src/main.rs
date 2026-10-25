use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod discord;
mod kick;
mod shared;
mod twitch;
use crate::kick::api::retrieve_public_key;
use crate::kick::state::KickService;
use crate::shared::app::build_router;
use crate::shared::event_router::{CHANNEL_QUEUE_SIZE, NotificationRouter, route_loop};
use crate::shared::utils::init_listener;
use crate::twitch::state::TwitchService;
use crate::twitch::verifier::HmacKey;
use rsa::RsaPublicKey;
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
    let (sender, receiver) = channel(CHANNEL_QUEUE_SIZE);

    let key =
        retrieve_public_key::<RsaPublicKey>(format!("{}/public-key", kick::constants::KICK_API))
            .await;

    let kick_service = KickService { public_key: key };
    let twitch_secret =
        std::env::var("TWITCH_SECRET").expect("Missing Twitch Secret: TWITCH_SECRET");
    let twitch_service = TwitchService {
        hmac_key: HmacKey { key: twitch_secret },
    };
    let app_state = shared::appstate::AppState::new(sender, kick_service, twitch_service).await;
    let notification_router = NotificationRouter::new();
    tokio::spawn(route_loop(notification_router, receiver));
    let app = build_router(app_state);
    let listener = init_listener().await.unwrap();

    let _ = axum::serve(listener, app).await;
}
