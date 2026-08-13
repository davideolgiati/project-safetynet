use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use flate2::Compression;
use flate2::write::GzEncoder;
use chrono::{DateTime, Utc};
use regex::RegexSet;

use crate::compression_level::CompressionLevel;
use crate::config::Item;
use crate::file_filter::{build_regex_registry, get_files_in_directory};
use crate::info;
use crate::progress_bar::{display_progress_bar, new_progress_bar, progress_index, update_progress_bar};

fn new_archive_name() -> String {
    let now: DateTime<Utc> = SystemTime::now().into();
    now.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

fn generate_archive_file_name(nickname: &String, archive_ts: String) -> String {
    format!("{}-{}.tar.gz", nickname, archive_ts)
}

fn compose_archive_path(nickname: &String, output_directory: &str) -> PathBuf {
    let archive_ts = new_archive_name();
    let archive_name = generate_archive_file_name(nickname, archive_ts);
    let output_dir = Path::new(output_directory).join(archive_name);

    info!("Using {} as output directory", output_dir.display());

    output_dir
}

fn create_tar_gz(archive_path: &Path) -> Result<File, std::io::Error> {
    File::create(archive_path)
}

fn gzip_encoder(tar_gz: File, compression_level: &CompressionLevel) -> GzEncoder<File> {
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

    GzEncoder::new(tar_gz, compression)
}

fn write_archive(enc: GzEncoder<File>, input_path: &str, include_rgx: &RegexSet, exclude_rgx: &RegexSet) -> Result<(), std::io::Error> {
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
        let relative_path = format!("./{}", file.strip_prefix(input_path).unwrap());
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
    let output_dir = match &config.output_directory {
        Some(dir) => dir,
        None => "."
    };

    let compression_level = match &config.compression_level {
        Some(value) => value,
        None => &CompressionLevel::Fast
    };

    let archive_path = compose_archive_path(&config.nickname, output_dir);
    let tar_gz = create_tar_gz(&archive_path)?;
    let enc = gzip_encoder(tar_gz, compression_level);
    let include_rgx = build_regex_registry(&config.include, vec![".*".to_string()]);
    let exclude_rgx = build_regex_registry(&config.exclude, Vec::new());
    write_archive(enc, &config.input_path, &include_rgx, &exclude_rgx)
}
