use crate::twitch::protocol::MessageHeaders;
use crate::twitch::{constants, error::TwitchError};

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

pub trait VerifyHmac {
    fn verify(&self, headers: &MessageHeaders, body: &str) -> Result<bool, TwitchError>;
}

#[derive(Clone)]
pub struct HmacKey {
    pub key: String,
}

impl VerifyHmac for HmacKey {
    fn verify(&self, headers: &MessageHeaders, body: &str) -> Result<bool, TwitchError> {
        let message = headers.message_id.clone() + &headers.message_timestamp + body;
        let mac = hmac(&self.key, &message)?;
        let s = hex::decode(&headers.message_signature[constants::SIGNATURE_PREFIX_LENGTH..])
            .map_err(|_| TwitchError::FailedValidation)?;
        Ok(mac.verify_slice(&s).is_ok())
    }
}

fn hmac(secret: &str, message: &str) -> Result<Hmac<Sha256>, TwitchError> {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|_| TwitchError::FailedValidation)?;
    mac.update(message.as_bytes());
    Ok(mac)
}
