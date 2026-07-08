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

pub async fn post_message(discord_token: &str, content: &str) -> Result<(), APIError> {
    let channel_id = std::env::var("CHANNEL_ID").unwrap();

    let url = format!("{}/channels/{channel_id}/messages", constants::DISCORD_API);

    let client = reqwest::Client::new();
    let request = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bot {}", discord_token))
        .json(&discord::models::Message {
            content: content.to_string(),
        })
        .build()
        .map_err(|_e| APIError::RequestFailed)?;
    let res = reqwest::Client::execute(&client, request)
        .await
        .map_err(|_e| APIError::RequestFailed)?;
    debug!("Status: {}", res.status());
    debug!("Body: {}", res.text().await.expect("bad"));
    Ok(())
}
