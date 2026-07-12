use crate::twitch::protocol::StreamOnline;
use std::collections::HashMap;
pub enum EventSource {
    Twitch,
}
pub enum EventKind {
    TwitchStreamOnline { event: StreamOnline },
}

pub struct InternalEvent {
    #[allow(dead_code)]
    pub source: EventSource,
    #[allow(dead_code)]
    pub metadata: HashMap<String, String>,
    pub payload: EventKind,
}
