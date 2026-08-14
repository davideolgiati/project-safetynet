use regex::regex;
use serde::Deserialize;
use std::fmt::Display;
use std::fs;
use std::path::Path;

use crate::compression_level::CompressionLevel;
use crate::info;
use crate::Result;

pub enum ConfigValueError {
    InvalidNickname(String),
    InvalidWorkingPath(String)
}

impl Display for ConfigValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigValueError::InvalidNickname(nick) => {
                write!(f, "Invalid nickname '{}': must contain only lowercase letters and digits", nick)
            },
            ConfigValueError::InvalidWorkingPath(nick) => {
                write!(f, "Invalid path '{}': must exists on filesystem", nick)
            }
        }
    }
}

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

pub struct WorkingPath(String);

impl<'de> Deserialize<'de> for WorkingPath {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        WorkingPath::new(&raw).map_err(serde::de::Error::custom)
    }
}

impl WorkingPath {
    pub fn new(raw: &str) -> std::result::Result<WorkingPath, ConfigValueError> {
        if Path::new(raw).exists() {
            Ok(WorkingPath(raw.into()))
        } else {
            Err(ConfigValueError::InvalidWorkingPath(raw.into()))
        }
    }
}

impl Display for WorkingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<Path> for WorkingPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

#[derive(Deserialize)]
pub struct Item {
    pub nickname: Nickname,
    pub input_path: WorkingPath,
    pub output_directory: Option<WorkingPath>,
    pub compression_level: Option<CompressionLevel>,
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>
}

pub fn load_configuration(path: &str) -> Result<Vec<Item>> {
    info!("Loading configurations from: {}", path);

    let config_content = fs::read_to_string(path)?;
    let config_entries: Vec<Item> = serde_json::from_str(&config_content)?;

    info!("Loaded {} entries", config_entries.len());

    Ok(config_entries)
}
