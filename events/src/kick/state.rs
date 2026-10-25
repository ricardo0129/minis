use crate::kick::api::FromPublicKeyPem;
use crate::kick::verifier::VerifySignature;

#[derive(Clone)]
pub struct KickService<T> {
    pub public_key: T,
}

impl<T: VerifySignature + FromPublicKeyPem> KickService<T> {
    pub async fn new(key: T) -> Self {
        Self { public_key: key }
    }
}
