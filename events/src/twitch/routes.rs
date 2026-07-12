use crate::shared::appstate::AppState;
use crate::shared::event::{EventKind, EventSource, InternalEvent};
use crate::shared::event_router::NotificationRouter;
use crate::twitch;
use crate::twitch::protocol::MessageHeaders;
use crate::twitch::protocol::{Event, MessageType, StreamOnline};
use crate::twitch::verifier;
use std::collections::HashMap;

use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::str::FromStr;
use tracing::info;

async fn verify_signature(headers: &HeaderMap, twitch_secret: &str, body: &str) -> bool {
    let headers: twitch::protocol::MessageHeaders =
        twitch::protocol::MessageHeaders::from_headers(headers);
    verifier::verify_twitch_request(&headers, twitch_secret, body)
}

fn handle_verification(body: String) -> Result<String, (StatusCode, String)> {
    info!("Verification Message");
    let Json(payload): Json<twitch::protocol::ChallengeBody> = Json::from_bytes(body.as_bytes())
        .map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                "Unable to Parse Challenge Body".to_string(),
            )
        })?;
    Ok(payload.challenge)
}

async fn handle_notification(body: &str, notifications: &NotificationRouter) {
    info!("Notification Message");
    let Json(event_wrapper): Json<Event<StreamOnline>> =
        Json::from_bytes(body.as_bytes()).expect("unable to parse notification");
    let event: InternalEvent = InternalEvent {
        source: EventSource::Twitch,
        metadata: HashMap::new(),
        payload: EventKind::TwitchStreamOnline {
            event: event_wrapper.event,
        },
    };
    notifications.route_event(event).await;
}

pub async fn event_sub(
    State(appstate): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !verify_signature(&headers, &appstate.twitch.twitch_secret, &body).await {
        return Err((
            StatusCode::BAD_REQUEST,
            "Unable to validate request".to_string(),
        ));
    }

    let message_headers = MessageHeaders::from_headers(&headers);

    let Ok(message_type) = MessageType::from_str(&message_headers.message_type) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid Message Type".to_string()));
    };

    match message_type {
        MessageType::Verification => {
            return handle_verification(body.clone());
        }
        MessageType::Notification => {
            handle_notification(&body, &appstate.notifications).await;
        }
        MessageType::Revocation => {
            info!("Revocation Message");
        }
    }
    Ok("success".to_string())
}
