use crate::discord;
use crate::shared::appstate::AppState;
use crate::twitch;
use crate::twitch::protocol::MessageType;
use crate::twitch::protocol::StreamOnline;
use crate::twitch::verifier;

use axum::Json;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use http_body_util::BodyExt;
use std::str::FromStr;
use tracing::{debug, info};

pub async fn event_sub(
    State(appstate): State<AppState>,
    req: Request,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let headers: twitch::protocol::MessageHeaders =
        twitch::protocol::MessageHeaders::from_headers(req.headers());

    let (_, body) = req.into_parts();
    let bytes = body.collect().await.expect("Body Error").to_bytes();

    if !verifier::verify_twitch_request(&headers, &appstate.twitch.twitch_secret, &bytes) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Unable to verify source".to_string(),
        ));
    }

    let Ok(message_type) = MessageType::from_str(&headers.message_type) else {
        return Err((StatusCode::BAD_REQUEST, "Invalid Message Type".to_string()));
    };

    match message_type {
        MessageType::Verification => {
            info!("Verification Message");
            let Json(payload): Json<twitch::protocol::ChallengeBody> =
                Json::from_bytes(&bytes).expect("unable to parse challenge");
            return Ok(payload.challenge);
        }
        MessageType::Notification => {
            info!("Notification Message");
            let Json(notification): Json<twitch::protocol::Event<StreamOnline>> =
                Json::from_bytes(&bytes).expect("unable to parse notification");
            debug!("{:?}", &notification.event);
            // Todo Handle Error
            let _ = discord::api::post_message(
                &appstate.discord.discord_token,
                &appstate.discord.channel_id,
                &serde_json::to_string(&notification.event).expect("faiedl to string"),
            )
            .await;
        }
        MessageType::Revocation => {
            info!("Revocation Message");
        }
    }
    Ok("success".to_string())
}
