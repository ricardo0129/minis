use crate::twitch::verifier::VerifyHmac;

#[derive(Clone)]
pub struct TwitchService<T: VerifyHmac> {
    pub hmac_key: T,
}

impl<T: VerifyHmac> TwitchService<T> {
    pub fn new(hmac_key: T) -> Self {
        Self { hmac_key }
    }
}
