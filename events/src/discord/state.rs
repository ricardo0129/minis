#[derive(Clone)]
pub struct DiscordService {
    pub discord_token: String,
    pub channel_id: String,
}

impl DiscordService {
    pub fn new() -> Self {
        Self {
            discord_token: std::env::var("DISCORD_TOKEN").expect("Missing Discord Token"),
            channel_id: std::env::var("CHANNEL_ID").expect("Missing Channel Id"),
        }
    }
}
