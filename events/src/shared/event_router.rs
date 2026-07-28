use crate::discord::state::DiscordNotifier;
use crate::shared::event::{EventKind, InternalEvent};
use crate::shared::notifier::Notifier;

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
            EventKind::TwitchStreamOnline {
                event: stream_online,
            } => {
                self.discord.post_notification(stream_online).await;
            }
            EventKind::KickStreamUpdate {
                event: stream_update,
            } => {
                self.discord.post_notification(stream_update).await;
            }
        }
    }
}

pub fn push_event(event_queue: &mut Vec<InternalEvent>, event: InternalEvent) {
    event_queue.push(event);
}

pub fn pull_events(event_queue: &mut Vec<InternalEvent>) -> Vec<InternalEvent> {
    let num_events = event_queue.len();
    event_queue.drain(0..num_events).collect()
}
