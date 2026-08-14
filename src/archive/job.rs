use std::time::SystemTime;

use crate::Result;
use crate::archive::Archive;
use crate::config::compression_level::CompressionLevel;
use crate::config::Item;
use crate::archive::file_filter::build_regex_registry;
use crate::info;
use crate::error::SafetyNetError;
use crate::archive::tar_writer::write_archive;
use crate::config::working_path::WorkingPath;

pub fn create_archive(config: &Item) -> Result<()> {
    let default_path = match WorkingPath::new(".") {
        Ok(path) => path,
        Err(err) => {
            return Err(SafetyNetError(err.to_string()))
        }
    };

    let output_directory = match &config.output_directory {
        Some(dir) => dir,
        None => &default_path,
    };

    let compression_level = match &config.compression_level {
        Some(value) => value,
        None => &CompressionLevel::Fast,
    };

    let archive = Archive::new(&config.nickname, output_directory);
    info!("Using {} as output path", archive);

    let enc = archive.get_encoder(compression_level)?;

    let include_rgx = match build_regex_registry(&config.include, vec![".*".to_string()]) {
        Ok(set) => set,
        Err(err) => {
            return Err(SafetyNetError::from(err))
        }
    };

    let exclude_rgx = match build_regex_registry(&config.exclude, Vec::new()) {
        Ok(set) => set,
        Err(err) => {
            return Err(SafetyNetError::from(err))
        }
    };

    write_archive(enc, &config.input_path, &include_rgx, &exclude_rgx)
}

pub fn archive_item(config: &Item) -> Result<bool> {
    let start = SystemTime::now();

    create_archive(config)?;

    let elapsed = SystemTime::now()
        .duration_since(start)?;

    info!(
        "Job \"{}\" done in {}ms",
        config.nickname,
        elapsed.as_millis()
    );

    Ok(true)
}
