use std::{fmt, io};

pub type Result<T> = std::result::Result<T, SafetyNetError>;

#[derive(Debug, Clone)]
pub struct SafetyNetError(pub String);

impl fmt::Display for SafetyNetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From::<io::Error> for SafetyNetError {
    fn from(value: io::Error) -> Self {
        SafetyNetError(value.to_string())
    }
}

impl From<std::time::SystemTimeError> for SafetyNetError {
    fn from(value: std::time::SystemTimeError) -> Self {
        SafetyNetError(value.to_string())
    }
}

impl From<regex::Error> for SafetyNetError {
    fn from(value: regex::Error) -> Self {
        SafetyNetError(value.to_string())
    }
}

impl From<serde_json::Error> for SafetyNetError {
    fn from(value: serde_json::Error) -> Self {
        SafetyNetError(value.to_string())
    }
}