use crate::kick::state::KickService;
use crate::shared::event::InternalEvent;
use crate::twitch::state::TwitchService;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub twitch: TwitchService,
    pub kick: KickService,
    pub events: Arc<Mutex<Vec<InternalEvent>>>,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            twitch: TwitchService::new(),
            kick: KickService::new().await,
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
