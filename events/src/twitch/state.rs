#[derive(Clone)]
pub struct TwitchService {
    pub twitch_secret: String,
}

impl TwitchService {
    pub fn new() -> Self {
        Self {
            twitch_secret: std::env::var("TWITCH_SECRET").expect("Missing Twitch Secret"),
        }
    }
}
