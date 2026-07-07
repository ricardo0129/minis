use axum::Json;
use axum::extract::Request;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header::HeaderMap;
use axum::response::IntoResponse;
use axum::{Router, routing};
use hmac::{Hmac, KeyInit, Mac};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const TWITCH_MESSAGE_ID: &str = "twitch-eventsub-message-id";
const TWITCH_MESSAGE_TIMESTAMP: &str = "twitch-eventsub-message-timestamp";
const TWITCH_MESSAGE_SIGNATURE: &str = "twitch-eventsub-message-signature";
const TWITCH_MESSAGE_TYPE: &str = "twitch-eventsub-message-type";

#[allow(dead_code)]
pub struct AppState {
    twitch_secret: String,
}

impl AppState {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            twitch_secret: std::env::var("TWITCH_SECRET").expect("Missing Twitch Secret"),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

const DISCORD_API: &str = "https://discord.com/api/v10";

async fn post_message(content: &str) {
    println!("post message");
    let channel_id = std::env::var("CHANNEL_ID").unwrap();

    let url = format!("{DISCORD_API}/channels/{channel_id}/messages");
    let discord_token = std::env::var("DISCORD_TOKEN").expect("missing");

    let client = reqwest::Client::new();
    let request = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bot {}", discord_token))
        .json(&Message {
            content: content.to_string(),
        })
        .build()
        .expect("failed to send");
    let res = reqwest::Client::execute(&client, request)
        .await
        .expect("bad request");
    println!("{:?}", res);
    println!("Status: {}", res.status());
    println!("Body: {}", res.text().await.expect("bad"));
}

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
    let app = Router::new().route("/eventsub", routing::post(event_sub));

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

struct MessageHeaders {
    message_id: String,
    message_timestamp: String,
    message_signature: String,
    message_type: String,
}

fn header_to_string(header: &HeaderValue) -> String {
    header.to_str().expect("Bad Header").to_string()
}

impl MessageHeaders {
    fn from_headers(headers: &HeaderMap) -> Self {
        Self {
            message_id: header_to_string(&headers[TWITCH_MESSAGE_ID]),
            message_timestamp: header_to_string(&headers[TWITCH_MESSAGE_TIMESTAMP]),
            message_signature: header_to_string(&headers[TWITCH_MESSAGE_SIGNATURE]),
            message_type: header_to_string(&headers[TWITCH_MESSAGE_TYPE]),
        }
    }
}

const SIGNATURE_PREFIX_LENGTH: usize = "sha256:".len();

#[derive(Debug, Serialize, Deserialize)]
struct Notification {
    id: String,
    broadcaster_user_id: String,
    broadcaster_user_login: String,
    broadcaster_user_name: String,
    #[serde(rename = "type")]
    notification_type: String,
    started_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    event: Notification,
}

fn get_secret() -> String {
    "hello12345hello".to_string()
}

async fn event_sub(req: Request) -> Result<impl IntoResponse, (StatusCode, String)> {
    let secret: String = get_secret();
    let headers: MessageHeaders = MessageHeaders::from_headers(req.headers());

    let (_, body) = req.into_parts();
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(_) => {
            println!("Bad Body");
            return Err((StatusCode::BAD_REQUEST, "Bad Request".to_string()));
        }
    };
    let body_str = match std::str::from_utf8(&bytes) {
        Ok(body) => body,
        Err(_) => {
            return Err((StatusCode::BAD_REQUEST, "Bad Request".to_string()));
        }
    };
    println!("body: {}", &body_str);

    let message = headers.message_id + &headers.message_timestamp + body_str;
    let mac = hmac(&secret, &message);
    let s = hex::decode(&headers.message_signature[SIGNATURE_PREFIX_LENGTH..])
        .expect("Signature Decode Failed");
    mac.verify_slice(&s).expect("unable to verify");
    match headers.message_type {
        val if val == MessageType::Verification.as_str() => {
            println!("Verification Message");
            let Json(payload): Json<ChallengeBody> =
                Json::from_bytes(&bytes).expect("unable to parse challenge");
            return Ok(payload.challenge);
        }
        val if val == MessageType::Notification.as_str() => {
            println!("Notification Message");
            let Json(notification): Json<Event> =
                Json::from_bytes(&bytes).expect("unable to parse notification");
            println!("{:?}", &notification.event);
            post_message(&serde_json::to_string(&notification.event).expect("faiedl to string"))
                .await;
        }
        val if val == MessageType::Revocation.as_str() => {
            println!("Revocation Message");
        }
        _ => return Err((StatusCode::BAD_REQUEST, "Unknown Request Type".to_string())),
    }
    Ok("success".to_string())
}

enum MessageType {
    Verification,
    Notification,
    Revocation,
}

impl MessageType {
    fn as_str(&self) -> &'static str {
        match self {
            MessageType::Verification => "webhook_callback_verification",
            MessageType::Notification => "notification",
            MessageType::Revocation => "revocation",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ChallengeBody {
    challenge: String,
}
