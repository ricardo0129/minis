use crate::kick::error::KickError;
use crate::kick::protocol::{MessageHeaders, MessageType, StreamUpdate};
use crate::shared::appstate::AppState;
use crate::shared::error::RouteError;
use crate::shared::event::{ChannelInfo, EventPayload, EventSource, InternalEvent};
use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use base64::prelude::*;
use rsa::RsaPublicKey;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;

pub fn verify_signature(
    public_key: &RsaPublicKey,
    headers: &MessageHeaders,
    body: &str,
) -> Result<bool, KickError> {
    let decoded_signature = BASE64_STANDARD
        .decode(&headers.message_signature)
        .map_err(|_| KickError::FailedValidation)?;
    let body = format!(
        "{}.{}.{}",
        headers.message_id, headers.message_timestamp, body
    );
    let verifying_key = VerifyingKey::<Sha256>::new(public_key.clone());
    let s =
        Signature::try_from(decoded_signature.as_ref()).map_err(|_| KickError::FailedValidation)?;
    Ok(verifying_key.verify(body.as_bytes(), &s).is_ok())
}

pub async fn webhook(
    State(appstate): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> Result<impl IntoResponse, RouteError> {
    let message_headers =
        MessageHeaders::from_headers(&headers).map_err(|_| RouteError::InvalidMessageType)?;
    match verify_signature(&appstate.kick.public_key, &message_headers, &body) {
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
