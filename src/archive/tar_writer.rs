use flate2::write::GzEncoder;
use regex::RegexSet;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::Result;
use crate::archive::file_filter::get_files_in_directory;
use crate::config::working_path::WorkingPath;
use crate::info;
use crate::progress_bar::ProgressBar;

pub(super) fn write_archive(
    enc: GzEncoder<File>,
    input_path: &WorkingPath,
    include_rgx: &RegexSet,
    exclude_rgx: &RegexSet,
) -> Result<()> {
    info!("Starting {} compression ... ", input_path);

    let mut tar = tar::Builder::new(enc);
    let files = get_files_in_directory(input_path, include_rgx, exclude_rgx);

    let file_count = files.len();
    let mut progress_bar = ProgressBar::new(70, file_count);

    progress_bar.show();

    for file in files {
        let mut data = File::open(&file)?;
        let file_path = Path::new(&file);
        let relative = file_path
            .strip_prefix(input_path)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        let relative_path = format!("./{}", relative.display());
        tar.append_file(relative_path, &mut data)?;

        progress_bar.progress();
        progress_bar.show();
    }

    tar.finish()?;
    Ok(())
}
