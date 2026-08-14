pub mod job;
mod file_filter;
mod tar_writer;

use chrono::{DateTime, Utc};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::fmt::Display;
use std::fs::File;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::Result;
use crate::config::compression_level::CompressionLevel;
use crate::info;
use crate::config::nickname::Nickname;
use crate::config::working_path::WorkingPath;

pub struct Archive(PathBuf);

impl Archive {
    fn new(nickname: &Nickname, output_directory: &WorkingPath) -> Archive {
        let unix_ts: DateTime<Utc> = SystemTime::now().into();
        let now = unix_ts.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let archive_name = format!("{}-{}.tar.gz", nickname, now);

        Archive(output_directory.as_ref().join(archive_name))
    }

    fn get_encoder(self, level: &CompressionLevel) -> Result<GzEncoder<File>> {
        let archive_path = File::create(self.0)?;

        let compression = match level {
            CompressionLevel::Best => {
                info!("Using max compression algorithm - level 9");
                Compression::best()
            }
            CompressionLevel::Fast => {
                info!("Using fastest compression algorithm - level 1");
                Compression::fast()
            }
        };

        Ok(GzEncoder::new(archive_path, compression))
    }
}

impl Display for Archive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display())
    }
}