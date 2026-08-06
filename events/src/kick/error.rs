use std::fmt;
#[derive(Debug)]
pub enum KickError {
    InvalidMessageType,
    FailedValidation,
}

impl fmt::Display for KickError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KickError::InvalidMessageType => write!(f, "Invalid Kick Message Type"),
            KickError::FailedValidation => write!(f, "Failed Kick Message Validation"),
        }
    }
}
