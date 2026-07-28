use crate::kick::state::KickService;
use crate::shared::event::InternalEvent;
use crate::twitch::state::TwitchService;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct AppState {
    pub twitch: TwitchService,
    pub kick: KickService,
    pub events: Arc<Mutex<Vec<InternalEvent>>>,
    pub sender: Sender<InternalEvent>,
}

impl AppState {
    pub async fn new(event_channel: Sender<InternalEvent>) -> Self {
        Self {
            twitch: TwitchService::new(),
            kick: KickService::new().await,
            events: Arc::new(Mutex::new(Vec::new())),
            sender: event_channel,
        }
    }
}
