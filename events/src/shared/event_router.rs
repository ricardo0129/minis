use crate::discord::state::DiscordNotifier;
use crate::shared::event::{EventKind, InternalEvent};
use crate::shared::notifier::Notifier;
use tokio::sync::mpsc::Receiver;
use tracing::info;

#[derive(Clone)]
pub struct NotificationRouter {
    discord: DiscordNotifier,
}

impl NotificationRouter {
    pub fn new() -> Self {
        Self {
            discord: DiscordNotifier::new(),
        }
    }
    pub async fn route_event(&self, event: InternalEvent) {
        match event.payload {
            EventKind::TwitchStreamOnline(event) => self.discord.post_notification(event).await,
            EventKind::TwitchChannelUpdate(event) => self.discord.post_notification(event).await,
            EventKind::KickStreamUpdate(event) => self.discord.post_notification(event).await,
        }
    }
}

pub const CHANNEL_QUEUE_SIZE: usize = 100;

pub async fn route_loop(
    notification_router: NotificationRouter,
    mut receiver: Receiver<InternalEvent>,
) {
    loop {
        match receiver.recv().await {
            Some(event) => {
                info!("Processing: {:?}", event);
                notification_router.route_event(event).await;
            }
            None => {
                info!("Notification Channel Closed");
                break;
            }
        }
    }
}
