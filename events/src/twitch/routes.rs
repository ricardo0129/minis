use crate::shared::appstate::AppState;
use crate::shared::event::{EventKind, EventSource, InternalEvent};
use crate::twitch;
use crate::twitch::error::TwitchError;
use crate::twitch::protocol::{
    ChallengeBody, ChannelUpdate, Event, MessageHeaders, MessageType, StreamOnline,
    SubscriptionType, SubscriptionWrapper,
};
use crate::twitch::verifier;
use std::collections::HashMap;

use crate::shared::error::RouteError;
use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
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

fn handle_verification(body: String) -> Result<String, (StatusCode, String)> {
    info!("Verification Message");
    let Json(payload): Json<ChallengeBody> = Json::from_bytes(body.as_bytes()).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            "Unable to Parse Challenge Body".to_string(),
        )
    })?;
    Ok(payload.challenge)
}

async fn handle_notification(body: &str, appstate: &AppState) -> Result<(), TwitchError> {
    info!("Notification Message");
    let Json(subscription_wrapper): Json<SubscriptionWrapper> =
        Json::from_bytes(body.as_bytes()).map_err(|_| TwitchError::EventParsing)?;

    let payload = match subscription_wrapper.subscription.subscription_type {
        SubscriptionType::StreamOnlineType => {
            let Json(event_wrapper): Json<Event<StreamOnline>> =
                Json::from_bytes(body.as_bytes()).map_err(|_| TwitchError::EventParsing)?;
            EventKind::TwitchStreamOnline(event_wrapper.event)
        }
        SubscriptionType::ChannelUpdateType => {
            let Json(event_wrapper): Json<Event<ChannelUpdate>> =
                Json::from_bytes(body.as_bytes()).map_err(|_| TwitchError::EventParsing)?;
            EventKind::TwitchChannelUpdate(event_wrapper.event)
        }
    };
    let event = InternalEvent {
        source: EventSource::Twitch,
        metadata: HashMap::new(),
        payload,
    };
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
            return handle_verification(body.clone()).map_err(|_| RouteError::InvalidSignature);
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
