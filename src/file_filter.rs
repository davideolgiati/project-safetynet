use std::fs;
use std::path::Path;
use regex::Regex;

pub fn get_files_indirectory(directory: &Path) -> Vec<String> {
    let mut files = Vec::new();
    let mut dirs = vec![directory.to_path_buf()];

    while let Some(dir) = dirs.pop() {
        for path in fs::read_dir(&dir).unwrap() {
            let entry = path.unwrap().path();
            if entry.is_dir() {
                dirs.push(entry);
            } else {
                files.push(entry.to_str().unwrap().to_owned())
            }
        }
    }

    files
}

pub fn build_regex_registry(rules: &Option<Vec<String>>, default: Vec<Regex>) -> Vec<Regex> {
    match rules {
        Some(rules) => rules
            .iter()
            .map(|rule| Regex::new(&format!("(?i){}", rule)).unwrap())
            .collect(),
        None => default
    }
}

pub fn filter_files(files: Vec<String>, include_rgx: &[Regex], exclude_rgx: &[Regex]) -> Vec<String> {
    files
        .into_iter()
        .filter(|file| include_rgx.iter().any(|rule| rule.is_match(file)))
        .filter(|file| !exclude_rgx.iter().any(|rule| rule.is_match(file)))
        .collect()
}
