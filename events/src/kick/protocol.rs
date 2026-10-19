use crate::kick::constants;
use crate::shared::utils::header_to_string;
use axum::http::HeaderMap;

use crate::kick::error::KickError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub enum MessageType {
    StreamUpdate,
}

impl FromStr for MessageType {
    type Err = KickError;
    fn from_str(a: &str) -> Result<Self, KickError> {
        match a {
            "livestream.status.updated" => Ok(MessageType::StreamUpdate),
            _ => Err(KickError::InvalidMessageType),
        }
    }
}

pub struct MessageHeaders {
    pub message_id: String,
    pub message_timestamp: String,
    pub message_signature: String,
    pub message_type: MessageType,
}

impl MessageHeaders {
    pub fn from_headers(headers: &HeaderMap) -> Result<Self, KickError> {
        Ok(Self {
            message_id: header_to_string(&headers[constants::KICK_MESSAGE_ID]),
            message_timestamp: header_to_string(&headers[constants::KICK_MESSAGE_TIMESTAMP]),
            message_signature: header_to_string(&headers[constants::KICK_MESSAGE_SIGNATURE]),
            message_type: MessageType::from_str(
                headers[constants::KICK_MESSAGE_TYPE].to_str().unwrap(),
            )
            .map_err(|_| KickError::InvalidMessageType)?,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamUpdate {
    pub broadcaster: Broadcaster,
    pub is_live: bool,
    pub title: String,
    pub started_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Broadcaster {
    pub is_anonymous: bool,
    pub user_id: u64,
    pub username: String,
    pub channel_slug: String,
}
