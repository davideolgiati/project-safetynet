use std::fmt::Display;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use flate2::Compression;
use flate2::write::GzEncoder;
use chrono::{DateTime, Utc};
use regex::RegexSet;

use crate::compression_level::CompressionLevel;
use crate::config::{Item, Nickname, WorkingPath};
use crate::file_filter::{build_regex_registry, get_files_in_directory};
use crate::info;
use crate::progress_bar::{display_progress_bar, new_progress_bar, progress_index, update_progress_bar};

pub struct Archive(PathBuf);

impl Archive {
    fn new(nickname: &Nickname, output_directory: &WorkingPath) -> Archive {
        let unix_ts: DateTime<Utc> = SystemTime::now().into();
        let now = unix_ts.to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let archive_name = format!("{}-{}.tar.gz", nickname, now);


        Archive(Path::new(&output_directory.to_string()).join(archive_name))
    }

    fn get_encoder(self, compression_level: &CompressionLevel) -> Result<GzEncoder<File>, io::Error> {
        let archive_path = File::create(self.0)?;

        let compression = match compression_level {
            CompressionLevel::Best => {
                info!("Using max compression algorithm - level 9");
                Compression::best()
            },
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

fn write_archive(enc: GzEncoder<File>, input_path: &WorkingPath, include_rgx: &RegexSet, exclude_rgx: &RegexSet) -> Result<(), std::io::Error> {
    info!("Starting {} compression ... ", input_path);

    let mut tar = tar::Builder::new(enc);
    let files = get_files_in_directory(input_path, include_rgx, exclude_rgx);

    let mut cnt = 0usize;
    let mut progress_bar = new_progress_bar(70);
    let file_count = files.len();
    let mut last_index = 0;

    display_progress_bar(&progress_bar);

    for file in files {
        let mut data: File = File::open(&file).unwrap();
        let relative_path = format!("./{}", file.strip_prefix(&input_path.to_string()).unwrap());
        tar.append_file(relative_path, &mut data).unwrap();
        cnt += 1;

        let index = progress_index(cnt, file_count, &progress_bar);
        if index != last_index {
            update_progress_bar(index, &mut progress_bar);
            display_progress_bar(&progress_bar);
            last_index = index;
        }
    }
    io::stdout().flush().unwrap();

    tar.finish()?;
    Ok(())
}

pub fn create_archive(config: &Item) -> Result<(), std::io::Error> {
    let default_path = match WorkingPath::new(".") {
        Ok(path) => path,
        Err(err) => panic!("{}", err)
    };

    let output_directory = match &config.output_directory {
        Some(dir) => dir,
        None => &default_path
    };

    let compression_level = match &config.compression_level {
        Some(value) => value,
        None => &CompressionLevel::Fast
    };
    
    let archive = Archive::new(&config.nickname, output_directory);
    info!("Using {} as output path", archive);

    let enc = archive.get_encoder(compression_level)?;
    let include_rgx = build_regex_registry(&config.include, vec![".*".to_string()]);
    let exclude_rgx = build_regex_registry(&config.exclude, Vec::new());
    write_archive(enc, &config.input_path, &include_rgx, &exclude_rgx)
}
