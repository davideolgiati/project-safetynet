use regex::regex;
use serde::Deserialize;
use std::fmt::Display;

use crate::config::ConfigValueError;

pub struct Nickname(String);

impl<'de> Deserialize<'de> for Nickname {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Nickname::new(&raw).map_err(serde::de::Error::custom)
    }
}

impl Nickname {
    fn new(raw: &str) -> std::result::Result<Nickname, ConfigValueError> {
        if regex!(r"^[a-z|\d]+$").is_match(raw) {
            Ok(Nickname(raw.into()))
        } else {
            Err(ConfigValueError::InvalidNickname(raw.into()))
        }
    }
}

impl Display for Nickname {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
