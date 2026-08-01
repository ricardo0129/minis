use crate::kick::api::retrieve_public_key;
use crate::kick::constants;
use rsa::RsaPublicKey;
use rsa::pkcs8::DecodePublicKey;

#[derive(Clone)]
pub struct KickService {
    pub public_key: RsaPublicKey,
}

impl KickService {
    pub async fn new() -> Self {
        let pub_key: String =
            retrieve_public_key(format!("{}/public-key", constants::KICK_API)).await;
        let public_key = RsaPublicKey::from_public_key_pem(&pub_key).expect("Bad Public Key");
        Self { public_key }
    }
}
