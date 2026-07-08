use crate::twitch::{self, constants};

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

fn hmac(secret: &str, message: &str) -> Hmac<Sha256> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("bad hmac secret");
    mac.update(message.as_bytes());
    mac
}

pub fn verify_twitch_request(
    headers: &twitch::protocol::MessageHeaders,
    twitch_secret: &str,
    body: &[u8],
) -> bool {
    let body_str: &str = std::str::from_utf8(body).expect("Error Parsing Body");
    let message = headers.message_id.clone() + &headers.message_timestamp + body_str;
    let mac = hmac(twitch_secret, &message);
    let s = hex::decode(&headers.message_signature[constants::SIGNATURE_PREFIX_LENGTH..])
        .expect("Signature Decode Failed");
    mac.verify_slice(&s).is_ok()
}
