use crate::kick::error::KickError;
use crate::kick::protocol::MessageHeaders;
use base64::prelude::*;
use rsa::RsaPublicKey;
use rsa::pkcs1v15::{Signature, VerifyingKey};
use rsa::sha2::Sha256;
use rsa::signature::Verifier;

pub trait VerifySignature {
    fn verify_signature(&self, headers: &MessageHeaders, body: &str) -> Result<bool, KickError>;
}

impl VerifySignature for RsaPublicKey {
    fn verify_signature(&self, headers: &MessageHeaders, body: &str) -> Result<bool, KickError> {
        let decoded_signature = BASE64_STANDARD
            .decode(&headers.message_signature)
            .map_err(|_| KickError::FailedValidation)?;
        let body = format!(
            "{}.{}.{}",
            headers.message_id, headers.message_timestamp, body
        );
        let verifying_key = VerifyingKey::<Sha256>::new(self.clone());
        let s = Signature::try_from(decoded_signature.as_ref())
            .map_err(|_| KickError::FailedValidation)?;
        Ok(verifying_key.verify(body.as_bytes(), &s).is_ok())
    }
}
