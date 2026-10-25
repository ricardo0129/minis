use crate::kick::api::FromPublicKeyPem;
use crate::kick::state::KickService;
use crate::kick::verifier::VerifySignature;
use crate::shared::event::InternalEvent;
use crate::twitch::state::TwitchService;
use crate::twitch::verifier::VerifyHmac;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct AppState<T, U> {
    pub twitch: TwitchService<U>,
    pub kick: KickService<T>,
    pub sender: Sender<InternalEvent>,
}

impl<T: VerifySignature + FromPublicKeyPem, U: VerifyHmac> AppState<T, U> {
    pub async fn new(
        event_channel: Sender<InternalEvent>,
        kick_service: KickService<T>,
        twitch_service: TwitchService<U>,
    ) -> Self {
        Self {
            twitch: twitch_service,
            kick: kick_service,
            sender: event_channel,
        }
    }
}
