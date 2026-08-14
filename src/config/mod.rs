pub mod compression_level;
pub mod nickname;
pub mod working_path;

use serde::Deserialize;
use std::fmt::Display;
use std::fs;

use crate::Result;
use crate::config::compression_level::CompressionLevel;
use crate::info;
use crate::config::nickname::Nickname;
use crate::config::working_path::WorkingPath;

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