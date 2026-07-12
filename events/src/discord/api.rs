use crate::discord::{self, constants};
use std::fmt;
use tracing::debug;

#[derive(Debug)]
pub enum APIError {
    RequestFailed,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            APIError::RequestFailed => write!(f, "Request Failed"),
        }
    }
}

pub async fn post_message(
    client: &reqwest::Client,
    discord_token: &str,
    channel_id: &str,
    content: &str,
) -> Result<(), APIError> {
    let url = format!("{}/channels/{channel_id}/messages", constants::DISCORD_API);

    let res = client
        .post(&url)
        .header("Authorization", format!("Bot {}", discord_token))
        .json(&discord::models::Message {
            content: content.to_string(),
        })
        .send()
        .await
        .map_err(|_e| APIError::RequestFailed)?;

    debug!("Status: {}", res.status());
    Ok(())
}
