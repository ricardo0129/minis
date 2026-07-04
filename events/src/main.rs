use axum::body;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router, response::Html, routing, routing::get};
use hex_literal::hex;
use hmac::digest::CtOutput;
use hmac::{Hmac, KeyInit, Mac};
use http_body_util::BodyExt;
use sha2::{Digest, Sha256};

const HMAX_PREFIX: &str = "sha256=";
const TWITCH_MESSAGE_ID: &str = "twitch-eventsub-message-id";
const TWITCH_MESSAGE_TIMESTAMP: &str = "twitch-eventsub-message-timestamp";
const TWITCH_MESSAGE_SIGNATURE: &str = "twitch-eventsub-message-signature";

#[tokio::main]
async fn main() {
    // build our application with a route
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

async fn event_sub(req: Request) -> Result<impl IntoResponse, (StatusCode, String)> {
    let secret: &str = "hello12345hello";
    let message_id = req.headers()[TWITCH_MESSAGE_ID]
        .to_str()
        .expect("awda")
        .to_string();
    let message_timestamp = req.headers()[TWITCH_MESSAGE_TIMESTAMP]
        .to_str()
        .expect("awd")
        .to_string();

    let signature = req.headers()[TWITCH_MESSAGE_SIGNATURE]
        .to_str()
        .expect("unable to get sigantreu")
        .to_string();
    println!(
        "message id: {} message timestamp: {}",
        message_id, message_timestamp
    );

    let (parts, body) = req.into_parts();
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            println!("bad");
            return Err((StatusCode::BAD_REQUEST, format!("Bad Request")));
        }
    };
    let body = match std::str::from_utf8(&bytes) {
        Ok(body) => body,
        Err(err) => {
            return Err((StatusCode::BAD_REQUEST, "12".to_string()));
        }
    };
    let message = message_id + &message_timestamp + body;
    println!("message: {}", &message);
    let mac = hmac(secret, &message);
    println!("sig: {:?}", mac.clone().finalize().as_bytes());
    println!("string {}", &signature[7..]);
    let s = hex::decode(&signature[7..]).expect("invalid hex");
    println!("actual sig: {:?}", s);
    mac.verify_slice(&s).expect("unable to verify");
    Ok("success")
}
