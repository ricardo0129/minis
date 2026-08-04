use crate::kick::api::retrieve_public_key;
use crate::kick::constants;
use rsa::RsaPublicKey;

#[derive(Clone)]
pub struct KickService {
    pub public_key: RsaPublicKey,
}

impl KickService {
    pub async fn new() -> Self {
        let public_key: RsaPublicKey =
            retrieve_public_key::<RsaPublicKey>(format!("{}/public-key", constants::KICK_API))
                .await;
        Self { public_key }
    }
}
