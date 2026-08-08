use crate::discord;
use crate::discord::constants;
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
            token: std::env::var("DISCORD_TOKEN").expect("Missing Discord Token: DISCORD_TOKEN"),
            channel_id: std::env::var("CHANNEL_ID").expect("Missing Channel Id: CHANNEL_ID"),
        }
    }
}

impl Notifier for DiscordNotifier {
    async fn post_notification<T: IntoNotification>(&self, notification: T) {
        let channel_id = &self.channel_id;
        let url = format!("{}/channels/{channel_id}/messages", constants::DISCORD_API);
        let _ = discord::api::post_message(
            &url,
            &self.client,
            &self.token,
            &notification.format_notification(),
        )
        .await;
    }
}
