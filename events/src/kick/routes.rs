use crate::kick::protocol::MessageHeaders;
use crate::shared::appstate::AppState;
use crate::shared::event::{EventKind, EventSource, InternalEvent};
use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use base64::prelude::*;
use rsa::RsaPublicKey;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::pkcs8::DecodePublicKey;
use rsa::sha2::Sha256;
use rsa::signature::Verifier;
use std::collections::HashMap;

pub fn verify_signature(api_key: &str, headers: &MessageHeaders, body: &str) -> bool {
    let decoded_signature = BASE64_STANDARD
        .decode(&headers.message_signature)
        .expect("Unable to decode signature");
    let body = format!(
        "{}.{}.{}",
        headers.message_id, headers.message_timestamp, body
    );
    let pub_key = RsaPublicKey::from_public_key_pem(api_key).expect("Bad Public Key");
    let verifying_key = VerifyingKey::<Sha256>::new(pub_key);
    let s = Signature::try_from(decoded_signature.as_ref()).expect("Signature error");
    verifying_key.verify(body.as_bytes(), &s).is_ok()
}

pub async fn webhook(
    State(appstate): State<AppState>,
    headers: HeaderMap,
    body: String,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let message_headers = MessageHeaders::from_headers(&headers);
    if !verify_signature(&appstate.kick.public_key, &message_headers, &body) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Unable to verify request".to_string(),
        ));
    }
    let Json(payload) = Json::from_bytes(body.as_bytes()).expect("bad Body");
    if message_headers.message_type == "livestream.status.updated" {
        let internal_event = InternalEvent {
            source: EventSource::Kick,
            metadata: HashMap::new(),
            payload: EventKind::KickStreamUpdate { event: payload },
        };
        appstate.notifications.route_event(internal_event).await;
    } else {
        return Err((StatusCode::BAD_REQUEST, "Bad Request".to_string()));
    }
    Ok("success".to_string())
}
