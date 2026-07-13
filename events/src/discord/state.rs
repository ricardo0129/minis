use crate::discord;
use reqwest;

#[derive(Clone)]
pub struct DiscordNotifier {
    client: reqwest::Client,
    token: String,
    channel_id: String,
}

pub trait IntoDiscordNotification {
    fn format_notification(&self) -> String;
}

impl DiscordNotifier {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            token: std::env::var("DISCORD_TOKEN").expect("Missing Discord Token"),
            channel_id: std::env::var("CHANNEL_ID").expect("Missing Channel Id"),
        }
    }
    pub async fn twitch_discord_notification(&self, notification: impl IntoDiscordNotification) {
        let _ = discord::api::post_message(
            &self.client,
            &self.token,
            &self.channel_id,
            &notification.format_notification(),
        )
        .await;
    }
}
