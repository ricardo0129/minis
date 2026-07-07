use axum::Json;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router, routing};
use hmac::{Hmac, KeyInit, Mac};
use http_body_util::BodyExt;
use sha2::Sha256;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod discord;
mod twitch;

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
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}

fn hmac(secret: &str, message: &str) -> Hmac<Sha256> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("bad hmac secret");
    mac.update(message.as_bytes());
    mac
}

const SIGNATURE_PREFIX_LENGTH: usize = "sha256:".len();

fn get_secret() -> String {
    "hello12345hello".to_string()
}

async fn event_sub(req: Request) -> Result<impl IntoResponse, (StatusCode, String)> {
    let secret: String = get_secret();
    let headers: twitch::protocol::MessageHeaders =
        twitch::protocol::MessageHeaders::from_headers(req.headers());

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
        val if val == twitch::protocol::MessageType::Verification.as_str() => {
            println!("Verification Message");
            let Json(payload): Json<twitch::protocol::ChallengeBody> =
                Json::from_bytes(&bytes).expect("unable to parse challenge");
            return Ok(payload.challenge);
        }
        val if val == twitch::protocol::MessageType::Notification.as_str() => {
            println!("Notification Message");
            let Json(notification): Json<twitch::protocol::Event> =
                Json::from_bytes(&bytes).expect("unable to parse notification");
            println!("{:?}", &notification.event);
            discord::api::post_message(
                &serde_json::to_string(&notification.event).expect("faiedl to string"),
            )
            .await;
        }
        val if val == twitch::protocol::MessageType::Revocation.as_str() => {
            println!("Revocation Message");
        }
        _ => return Err((StatusCode::BAD_REQUEST, "Unknown Request Type".to_string())),
    }
    Ok("success".to_string())
}
