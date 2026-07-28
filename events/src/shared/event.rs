use crate::kick::protocol::StreamUpdate;
use crate::twitch::protocol::StreamOnline;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum EventSource {
    Twitch,
    Kick,
}

#[derive(Clone, Debug)]
pub enum EventKind {
    TwitchStreamOnline { event: StreamOnline },
    KickStreamUpdate { event: StreamUpdate },
}

#[derive(Clone, Debug)]
pub struct InternalEvent {
    #[allow(dead_code)]
    pub source: EventSource,
    #[allow(dead_code)]
    pub metadata: HashMap<String, String>,
    pub payload: EventKind,
}
