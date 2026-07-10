use crate::discord::state::DiscordService;
use crate::twitch::state::TwitchService;

#[derive(Clone)]
pub struct AppState {
    pub twitch: TwitchService,
    pub discord: DiscordService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            twitch: TwitchService::new(),
            discord: DiscordService::new(),
        }
    }
}
