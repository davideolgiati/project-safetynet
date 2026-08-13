use std::fs;
use std::path::Path;
use regex::RegexSet;

use crate::trace;

enum PathType {
    Directory,
    File,
    Skip
}

fn is_valid(path: &str, include_rgx: &RegexSet, exclude_rgx: &RegexSet) -> bool {
    include_rgx.is_match(path) &&
    !exclude_rgx.is_match(path)
}

pub fn build_regex_registry(rules: &Option<Vec<String>>, default: Vec<String>) -> RegexSet {
    let patterns = rules.as_ref().unwrap_or(&default);
    RegexSet::new(patterns.iter().map(|r| format!("(?i){}", r))).unwrap()
}

fn identify_path(path: &Path, include: &RegexSet, exclude: &RegexSet) -> PathType {
    if path.is_dir() {
        return PathType::Directory
    }

    if !is_valid(&path.to_string_lossy(), include, exclude) {
        return PathType::Skip
    }

    PathType::File
}

pub fn get_files_in_directory(path: &str, include: &RegexSet, exclude: &RegexSet) -> Vec<String> {
    let directory = Path::new(path);
    let mut files = Vec::new();
    let mut dirs = vec![directory.to_path_buf()];
    let debug = false; // future use

    while let Some(dir) = dirs.pop() {
        for path in fs::read_dir(&dir).unwrap() {
            let entry = path.unwrap().path();

            match identify_path(&entry, include, exclude) {
                PathType::Directory => dirs.push(entry),
                PathType::File => files.push(entry.to_string_lossy().into_owned()),
                PathType::Skip => {
                    if debug {
                        trace!("Path \"{}\" skipped", entry.to_string_lossy())
                    }
                }
            }
        }
    }

    files
}
