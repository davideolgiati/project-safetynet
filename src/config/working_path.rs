use serde::Deserialize;
use std::fmt::Display;
use std::path::Path;

use crate::config::ConfigValueError;

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
