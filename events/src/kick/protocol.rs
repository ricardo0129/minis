use crate::kick::constants;
use crate::shared::utils::header_to_string;
use axum::http::HeaderMap;

use crate::shared::notifier::IntoNotification;
use serde::{Deserialize, Serialize};

pub struct MessageHeaders {
    pub message_id: String,
    pub message_timestamp: String,
    pub message_signature: String,
    pub message_type: String,
}

impl MessageHeaders {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        Self {
            message_id: header_to_string(&headers[constants::KICK_MESSAGE_ID]),
            message_timestamp: header_to_string(&headers[constants::KICK_MESSAGE_TIMESTAMP]),
            message_signature: header_to_string(&headers[constants::KICK_MESSAGE_SIGNATURE]),
            message_type: header_to_string(&headers[constants::KICK_MESSAGE_TYPE]),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamUpdate {
    pub broadcaster: Broadcaster,
    pub is_live: bool,
    pub title: String,
    pub started_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Broadcaster {
    pub is_anonymous: bool,
    pub user_id: String,
    pub username: String,
    pub is_verified: bool,
    pub profile_picture: String,
    pub channel_slug: String,
}

impl IntoNotification for StreamUpdate {
    fn format_notification(&self) -> String {
        let username = &self.broadcaster.username;
        format!("{} Went Live! https://twitch.com/{}", username, username)
    }
}
