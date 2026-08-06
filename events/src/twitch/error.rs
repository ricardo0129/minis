use std::fmt;

#[derive(Debug)]
pub enum TwitchError {
    FailedValidation,
    EventParsing,
}

impl fmt::Display for TwitchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TwitchError::FailedValidation => write!(f, "Failed to Validate Twitch Request"),
            TwitchError::EventParsing => write!(f, "Failed Parsing Twitch Event"),
        }
    }
}
