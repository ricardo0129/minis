use crate::shared::appstate::AppState;
use crate::shared::event::{ChannelInfo, EventPayload, EventSource, InternalEvent, TitleInfo};
use crate::twitch;
use crate::twitch::error::TwitchError;
use crate::twitch::protocol::{ChallengeBody, Event, MessageHeaders, MessageType, Notification};
use crate::twitch::verifier;

use crate::shared::error::RouteError;
use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use std::str::FromStr;
use tracing::info;

async fn verify_signature(
    headers: &HeaderMap,
    twitch_secret: &str,
    body: &str,
) -> Result<bool, TwitchError> {
    let headers: twitch::protocol::MessageHeaders =
        twitch::protocol::MessageHeaders::from_headers(headers);
    verifier::verify_twitch_request(&headers, twitch_secret, body)
}

fn handle_verification(body: String) -> Result<String, RouteError> {
    info!("Verification Message");
    let Json(payload): Json<ChallengeBody> =
        Json::from_bytes(body.as_bytes()).map_err(|_| RouteError::InvalidSignature)?;
    Ok(payload.challenge)
}

async fn handle_notification(body: &str, appstate: &AppState) -> Result<(), TwitchError> {
    info!("Notification Message");
    let Json(notification): Json<Notification> =
        Json::from_bytes(body.as_bytes()).map_err(|_| TwitchError::EventParsing)?;

    let payload = match notification.event {
        Event::StreamOnline {
            broadcaster_user_name,
            started_at: _,
        } => EventPayload::Online {
            source: EventSource::Twitch,
            channel_info: ChannelInfo {
                user_name: broadcaster_user_name,
            },
        },
        Event::StreamOffline {
            broadcaster_user_name,
        } => EventPayload::Offline {
            source: EventSource::Twitch,
            channel_info: ChannelInfo {
                user_name: broadcaster_user_name,
            },
        },
        Event::ChannelUpdate {
            broadcaster_user_name,
            title,
            category_name,
        } => EventPayload::Update {
            source: EventSource::Twitch,
            channel_info: ChannelInfo {
                user_name: broadcaster_user_name,
            },
            title_info: TitleInfo {
                category_name,
                title,
            },
        },
    };
    let event = InternalEvent { payload };
    appstate.sender.send(event).await.unwrap();
    Ok(())
}

pub async fn event_sub(
    State(appstate): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> Result<impl IntoResponse, RouteError> {
    match verify_signature(&headers, &appstate.twitch.twitch_secret, &body).await {
        Ok(true) => {}
        _ => return Err(RouteError::InvalidSignature),
    }

    let message_headers = MessageHeaders::from_headers(&headers);

    let Ok(message_type) = MessageType::from_str(&message_headers.message_type) else {
        return Err(RouteError::InvalidMessageType);
    };

    match message_type {
        MessageType::Verification => {
            return handle_verification(body.clone());
        }
        MessageType::Notification => {
            handle_notification(&body, &appstate)
                .await
                .map_err(|_| RouteError::BadRequest)?;
        }
        MessageType::Revocation => {
            info!("Revocation Message");
        }
    }
    Ok("success".to_string())
}
