use chrono::{DateTime, Utc};
use flate2::Compression;
use flate2::write::GzEncoder;
use regex::RegexSet;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::Result;
use crate::compression_level::CompressionLevel;
use crate::config::{Item, Nickname, WorkingPath};
use crate::file_filter::{build_regex_registry, get_files_in_directory};
use crate::info;
use crate::progress_bar::{
    display_progress_bar, flush_stdout, new_progress_bar, progress_index, update_progress_bar,
};
use crate::safetynet_error::SafetyNetError;

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

fn write_archive(
    enc: GzEncoder<File>,
    input_path: &WorkingPath,
    include_rgx: &RegexSet,
    exclude_rgx: &RegexSet,
) -> Result<()> {
    info!("Starting {} compression ... ", input_path);

    let mut tar = tar::Builder::new(enc);
    let files = get_files_in_directory(input_path, include_rgx, exclude_rgx);

    let mut cnt = 0usize;
    let mut progress_bar = new_progress_bar(70);
    let file_count = files.len();
    let mut last_index = 0;

    display_progress_bar(&progress_bar);

    for file in files {
        let mut data = File::open(&file)?;
        let file_path = Path::new(&file);
        let relative = file_path.strip_prefix(input_path)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let relative_path = format!("./{}", relative.display());
        tar.append_file(relative_path, &mut data)?;
        cnt += 1;

        let index = progress_index(cnt, file_count, &progress_bar);
        if index != last_index {
            update_progress_bar(index, &mut progress_bar);
            display_progress_bar(&progress_bar);
            last_index = index;
        }
    }

    flush_stdout();

    tar.finish()?;
    Ok(())
}

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
