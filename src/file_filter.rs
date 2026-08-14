use regex::RegexSet;
use std::fs;
use std::path::Path;

use crate::trace;

enum PathType {
    Directory,
    File(String),
    Skip(String),
}

enum FilterOutcome {
    MatchExclude(String),
    NoMatchInclude,
    Valid,
}

fn is_valid(path: &str, include_rgx: &RegexSet, exclude_rgx: &RegexSet) -> FilterOutcome {
    if !include_rgx.is_match(path) {
        return FilterOutcome::NoMatchInclude;
    }

    let matches: Vec<_> = exclude_rgx
        .matches(path)
        .into_iter()
        .map(|index| exclude_rgx.patterns()[index].as_str())
        .collect();

    if matches.is_empty() {
        return FilterOutcome::Valid;
    }

    FilterOutcome::MatchExclude(format!(
        "{} matched by the following exclude regexes: {}",
        path,
        matches.join(", ")
    ))
}

pub fn build_regex_registry(rules: &Option<Vec<String>>, default: Vec<String>) -> RegexSet {
    let patterns = rules.as_ref().unwrap_or(&default);
    RegexSet::new(patterns.iter().map(|r| format!("(?i){}", r))).unwrap() // fix me
}

fn identify_path(path: &Path, include: &RegexSet, exclude: &RegexSet) -> PathType {
    if path.is_dir() {
        return PathType::Directory;
    }

    let file = path.to_string_lossy();

    match is_valid(&file, include, exclude) {
        FilterOutcome::MatchExclude(msg) => PathType::Skip(msg),
        FilterOutcome::NoMatchInclude => PathType::Skip(format!("{} did not match any include rule", file)),
        FilterOutcome::Valid => PathType::File(file.to_string())
    }
}

pub fn get_files_in_directory(path: &str, include: &RegexSet, exclude: &RegexSet) -> Vec<String> {
    let directory = Path::new(path);
    let mut files = Vec::new();
    let mut dirs = vec![directory.to_path_buf()];
    let debug = false; // future use

    while let Some(dir) = dirs.pop() {
        for path in fs::read_dir(&dir).unwrap() {
            let entry = match path {
                Ok(data) => data.path(),
                Err(ref err) => {
                    trace!("Error while working on {:?}: {}", path, err);
                    continue;
                }
            };

            match identify_path(&entry, include, exclude) {
                PathType::Directory => dirs.push(entry),
                PathType::File(path) => files.push(path),
                PathType::Skip(msg) => {
                    if debug {
                        trace!("{}", msg)
                    }
                }
            }
        }
    }

    files
}
