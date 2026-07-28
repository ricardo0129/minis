use crate::kick::api::retrieve_public_key;
use crate::kick::constants;

#[derive(Clone)]
pub struct KickService {
    pub public_key: String,
}

impl KickService {
    pub async fn new() -> Self {
        Self {
            public_key: retrieve_public_key(format!("{}/public-key", constants::KICK_API)).await,
        }
    }
}
