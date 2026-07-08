#[derive(Clone)]
pub struct AppState {
    pub twitch_secret: String,
    pub discord_token: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            twitch_secret: std::env::var("TWITCH_SECRET").expect("Missing Twitch Secret"),
            discord_token: std::env::var("DISCORD_TOKEN").expect("Missing Discord Token"),
        }
    }
}
