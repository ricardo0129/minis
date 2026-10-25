use crate::shared::appstate::AppState;
use crate::shared::event::{ChannelInfo, EventPayload, EventSource, InternalEvent, TitleInfo};
use crate::twitch::error::TwitchError;
use crate::twitch::protocol::{ChallengeBody, Event, MessageHeaders, MessageType, Notification};
use crate::twitch::verifier::VerifyHmac;

use crate::shared::error::RouteError;
use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use std::str::FromStr;
use tracing::info;

fn handle_verification(body: String) -> Result<String, RouteError> {
    info!("Verification Message");
    let Json(payload): Json<ChallengeBody> =
        Json::from_bytes(body.as_bytes()).map_err(|_| RouteError::InvalidSignature)?;
    Ok(payload.challenge)
}

async fn handle_notification<T, U>(
    body: &str,
    appstate: &AppState<T, U>,
) -> Result<(), TwitchError> {
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

pub async fn event_sub<T, U: VerifyHmac>(
    State(appstate): State<AppState<T, U>>,
    headers: HeaderMap,
    body: String,
) -> Result<impl IntoResponse, RouteError> {
    let message_headers = MessageHeaders::from_headers(&headers);

    match appstate.twitch.hmac_key.verify(&message_headers, &body) {
        Ok(true) => {}
        _ => return Err(RouteError::InvalidSignature),
    }

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

#[cfg(test)]
mod test {

    use super::*;
    use crate::kick::state::KickService;
    use crate::twitch::state::TwitchService;
    use axum::{Router, routing};
    use tokio::sync::mpsc::channel;

    use crate::twitch::constants::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use tower_http::trace::TraceLayer;

    const TEST_ENDPOINT: &str = "/twitch/eventsub";

    #[derive(Clone)]
    struct MockTwitchKey {
        key: String,
    }
    impl VerifyHmac for MockTwitchKey {
        fn verify(&self, _headers: &MessageHeaders, body: &str) -> Result<bool, TwitchError> {
            Ok(self.key == body)
        }
    }
    fn test_app() -> Router {
        let kick_service = KickService {
            public_key: "123".to_string(),
        };
        let twitch_service = TwitchService {
            hmac_key: MockTwitchKey {
                key: "mock_key".into(),
            },
        };

        let (sender, _) = channel(1);
        let app_state = AppState {
            kick: kick_service,
            twitch: twitch_service,
            sender,
        };
        let router = Router::new()
            .route(TEST_ENDPOINT, routing::post(event_sub))
            .layer(TraceLayer::new_for_http())
            .with_state(app_state);
        router
    }

    #[tokio::test]
    #[should_panic]
    async fn test_event_sub_missing_headers() {
        let app = test_app();
        app.oneshot(Request::post(TEST_ENDPOINT).body(Body::from("")).unwrap())
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn test_event_sub_valid_revocation_request() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::post(TEST_ENDPOINT)
                    .header(TWITCH_MESSAGE_ID, "1")
                    .header(TWITCH_MESSAGE_TIMESTAMP, "2026")
                    .header(TWITCH_MESSAGE_SIGNATURE, "sha256:123")
                    .header(TWITCH_MESSAGE_TYPE, "revocation")
                    .body(Body::from("mock_key"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
