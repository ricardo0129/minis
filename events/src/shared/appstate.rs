use crate::kick::state::KickService;
use crate::shared::event::InternalEvent;
use crate::twitch::state::TwitchService;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct AppState {
    pub twitch: TwitchService,
    pub kick: KickService,
    pub sender: Sender<InternalEvent>,
}

impl AppState {
    pub async fn new(event_channel: Sender<InternalEvent>) -> Self {
        Self {
            twitch: TwitchService::new(),
            kick: KickService::new().await,
            sender: event_channel,
        }
    }
}
