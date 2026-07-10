#[derive(Clone)]
pub struct DiscordService {
    pub discord_token: String,
}

impl DiscordService {
    pub fn new() -> Self {
        Self {
            discord_token: std::env::var("DISCORD_TOKEN").expect("Missing Discord Token"),
        }
    }
}
