use crate::shared::event_router::NotificationRouter;
use crate::twitch::state::TwitchService;

#[derive(Clone)]
pub struct AppState {
    pub twitch: TwitchService,
    pub notifications: NotificationRouter,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            twitch: TwitchService::new(),
            notifications: NotificationRouter::new(),
        }
    }
}
