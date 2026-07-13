use crate::discord::state::IntoDiscordNotification;
use crate::shared::utils::header_to_string;
use crate::twitch::constants;
use axum::http::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub enum MessageType {
    Verification,
    Notification,
    Revocation,
}

impl FromStr for MessageType {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "webhook_callback_verification" => Ok(MessageType::Verification),
            "notification" => Ok(MessageType::Notification),
            "revocation" => Ok(MessageType::Revocation),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeBody {
    pub challenge: String,
}

pub struct MessageHeaders {
    pub message_id: String,
    pub message_timestamp: String,
    pub message_signature: String,
    pub message_type: String,
}

impl MessageHeaders {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        Self {
            message_id: header_to_string(&headers[constants::TWITCH_MESSAGE_ID]),
            message_timestamp: header_to_string(&headers[constants::TWITCH_MESSAGE_TIMESTAMP]),
            message_signature: header_to_string(&headers[constants::TWITCH_MESSAGE_SIGNATURE]),
            message_type: header_to_string(&headers[constants::TWITCH_MESSAGE_TYPE]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamOnline {
    pub id: String,
    pub broadcaster_user_id: String,
    pub broadcaster_user_login: String,
    pub broadcaster_user_name: String,
    #[serde(rename = "type")]
    pub notification_type: String,
    pub started_at: String,
}

impl IntoDiscordNotification for StreamOnline {
    fn format_notification(&self) -> String {
        format!(
            "{} Went Live! https://twitch.com/{}",
            self.broadcaster_user_name, self.broadcaster_user_name
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event<T> {
    pub event: T,
}
