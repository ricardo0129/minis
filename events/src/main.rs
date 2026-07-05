use axum::Json;
use axum::RequestExt;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::http::header::HeaderMap;
use axum::response::IntoResponse;
use axum::{Router, routing};
use hmac::{Hmac, KeyInit, Mac};
use http_body_util::BodyExt;
use sha2::Sha256;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const TWITCH_MESSAGE_ID: &str = "twitch-eventsub-message-id";
const TWITCH_MESSAGE_TIMESTAMP: &str = "twitch-eventsub-message-timestamp";
const TWITCH_MESSAGE_SIGNATURE: &str = "twitch-eventsub-message-signature";

#[tokio::main]
async fn main() {
    // build our application with a route
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let app = Router::new()
        .route("/eventsub", routing::post(event_sub))
        .route("/challenge", routing::post(challenge));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}

fn hmac(secret: &str, message: &str) -> Hmac<Sha256> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("bad hmac secret");
    mac.update(message.as_bytes());
    mac
}

fn parse_message_headers(headers: &HeaderMap) -> (String, String, String) {
    let message_id = headers[TWITCH_MESSAGE_ID]
        .to_str()
        .expect("Missing message id");
    let message_timestamp = headers[TWITCH_MESSAGE_TIMESTAMP]
        .to_str()
        .expect("Missing message timestamp");
    let message_signature = headers[TWITCH_MESSAGE_SIGNATURE]
        .to_str()
        .expect("Missing message signature");
    (
        message_id.to_string(),
        message_timestamp.to_string(),
        message_signature.to_string(),
    )
}

const SIGNATURE_PREFIX_LENGTH: usize = 7;

async fn event_sub(req: Request) -> Result<impl IntoResponse, (StatusCode, String)> {
    let secret: &str = "hello12345hello";
    let (message_id, message_timestamp, message_signature) = parse_message_headers(req.headers());

    let (_, body) = req.into_parts();
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => {
            println!("Bad Body");
            return Err((StatusCode::BAD_REQUEST, "Bad Request".to_string()));
        }
    };
    let body = match std::str::from_utf8(&bytes) {
        Ok(body) => body,
        Err(_) => {
            return Err((StatusCode::BAD_REQUEST, "Bad Request".to_string()));
        }
    };
    println!("body: {}", &body);
    let message = message_id + &message_timestamp + body;
    let mac = hmac(secret, &message);
    let s = hex::decode(&message_signature[SIGNATURE_PREFIX_LENGTH..])
        .expect("Signature Decode Failed");
    mac.verify_slice(&s).expect("unable to verify");
    Ok("success")
}

const MESSAGE_TYPE: &str = "twitch-eventsub-message-type";

enum MessageType {
    Verification,
}

impl MessageType {
    fn as_str(&self) -> &'static str {
        match self {
            MessageType::Verification => "webhook_callback_verification",
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ChallengeBody {
    challenge: String,
}

async fn challenge(req: Request) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("{:?}", req);
    let message_type = req.headers()[MESSAGE_TYPE]
        .to_str()
        .expect("Missing message type");
    match message_type {
        val if val == MessageType::Verification.as_str() => {
            let Json(payload): Json<ChallengeBody> =
                req.extract().await.expect("Unable to parse Verification");
            return Ok(payload.challenge);
        }
        _ => {
            println!("unknown type");
            return Err((StatusCode::BAD_REQUEST, "Bad Request".to_string()));
        }
    }
}
