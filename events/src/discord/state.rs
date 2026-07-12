use crate::discord;
use crate::twitch::protocol::StreamOnline;
use reqwest;

#[derive(Clone)]
pub struct DiscordNotifier {
    client: reqwest::Client,
    token: String,
    channel_id: String,
}

impl DiscordNotifier {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            token: std::env::var("DISCORD_TOKEN").expect("Missing Discord Token"),
            channel_id: std::env::var("CHANNEL_ID").expect("Missing Channel Id"),
        }
    }
    pub async fn twitch_discord_notification(&self, event: StreamOnline) {
        let _ = discord::api::post_message(
            &self.client,
            &self.token,
            &self.channel_id,
            &serde_json::to_string(&event).expect("failed to generate string"),
        )
        .await;
    }
}
