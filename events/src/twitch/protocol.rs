use crate::twitch::constants;
use axum::http::HeaderValue;
use axum::http::header::HeaderMap;
use serde::{Deserialize, Serialize};

pub enum MessageType {
    Verification,
    Notification,
    Revocation,
}

impl MessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::Verification => "webhook_callback_verification",
            MessageType::Notification => "notification",
            MessageType::Revocation => "revocation",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeBody {
    pub challenge: String,
}

fn header_to_string(header: &HeaderValue) -> String {
    header.to_str().expect("Bad Header").to_string()
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Event<T> {
    pub event: T,
}
