#[derive(Clone)]
pub struct TwitchService<T> {
    pub hmac_key: T,
}
