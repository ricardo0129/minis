use crate::kick::state::KickService;
use crate::shared::event_router::NotificationRouter;
use crate::twitch::state::TwitchService;

#[derive(Clone)]
pub struct AppState {
    pub twitch: TwitchService,
    pub kick: KickService,
    pub notifications: NotificationRouter,
}

impl AppState {
    pub async fn new() -> Self {
        Self {
            twitch: TwitchService::new(),
            kick: KickService::new().await,
            notifications: NotificationRouter::new(),
        }
    }
}
