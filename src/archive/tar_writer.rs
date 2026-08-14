use flate2::write::GzEncoder;
use regex::RegexSet;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::Result;
use crate::archive::file_filter::get_files_in_directory;
use crate::info;
use crate::progress_bar::{
    display_progress_bar, flush_stdout, new_progress_bar, progress_index, update_progress_bar,
};
use crate::config::working_path::WorkingPath;

pub(super) fn write_archive(
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
