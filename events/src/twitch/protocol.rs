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

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Event {
    StreamOnline {
        #[allow(dead_code)]
        started_at: String,
        broadcaster_user_name: String,
    },
    ChannelUpdate {
        broadcaster_user_name: String,
        title: String,
        category_name: String,
    },
    StreamOffline {
        broadcaster_user_name: String,
    },
}

#[derive(Clone, Debug, Deserialize)]
pub struct Notification {
    pub event: Event,
}
