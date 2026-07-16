use crate::discord;
use crate::shared::notifier::{IntoNotification, Notifier};
use reqwest;

#[derive(Clone)]
pub struct DiscordNotifier {
    pub client: reqwest::Client,
    pub token: String,
    pub channel_id: String,
}

impl DiscordNotifier {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            token: std::env::var("DISCORD_TOKEN").expect("Missing Discord Token"),
            channel_id: std::env::var("CHANNEL_ID").expect("Missing Channel Id"),
        }
    }
}

impl Notifier for DiscordNotifier {
    async fn post_notification(&self, notification: impl IntoNotification) {
        let _ = discord::api::post_message(
            &self.client,
            &self.token,
            &self.channel_id,
            &notification.format_notification(),
        )
        .await;
    }
}
