use crate::kick::protocol::{MessageHeaders, MessageType, StreamUpdate};
use crate::kick::verifier::VerifySignature;
use crate::shared::appstate::AppState;
use crate::shared::error::RouteError;
use crate::shared::event::{ChannelInfo, EventPayload, EventSource, InternalEvent};
use crate::twitch::verifier::VerifyHmac;
use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;

pub async fn webhook<T: VerifySignature, U: VerifyHmac>(
    State(appstate): State<AppState<T, U>>,
    headers: HeaderMap,
    body: String,
) -> Result<impl IntoResponse, RouteError> {
    let message_headers =
        MessageHeaders::from_headers(&headers).map_err(|_| RouteError::InvalidMessageType)?;
    match appstate.kick.public_key.verify(&message_headers, &body) {
        Ok(true) => {}
        _ => return Err(RouteError::InvalidSignature),
    }

    let Json(notification): Json<StreamUpdate> =
        Json::from_bytes(body.as_bytes()).map_err(|_| RouteError::BadRequest)?;
    match message_headers.message_type {
        MessageType::StreamUpdate => {
            let event = InternalEvent {
                payload: EventPayload::Online {
                    source: EventSource::Kick,
                    channel_info: ChannelInfo {
                        user_name: notification.broadcaster.username,
                    },
                },
            };
            appstate.sender.send(event).await.unwrap();
        }
    }
    Ok("success".to_string())
}
