use std::path::Path;
use std::fs;
use serde::Deserialize;
use regex::regex;

use crate::compression_level::CompressionLevel;


#[derive(Deserialize)]
pub struct Item {
    pub nickname: String,
    pub input_path: String,
    pub output_directory: Option<String>,
    pub compression_level: Option<CompressionLevel>,
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>
}

fn is_nickname_valid(nickname: &str) -> bool {
    regex!(r"^[a-z|\d]+$").is_match(nickname)
}

fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn load_configuration(path: &str) -> Vec<Item> {
    println!("[i] Loading configurations from: {}", path);
    let config_content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Failed to read {}", path));

    let config_entries: Vec<Item> = serde_json::from_str(&config_content)
        .expect("Failed to parse JSON configuration");

    println!("[i] Loaded {} entries", config_entries.len());

    config_entries
}

pub fn validate_configuration_entry(config: &Item) -> bool {
    print!("[i] Validating {} configuration ... ", config.nickname);

    if !is_nickname_valid(&config.nickname) {
        print!("[!] Nickname: \"{}\" is not valid", config.nickname);
        return false;
    }

    if !path_exists(&config.input_path) {
        print!(
            "[!] Given input path \"{}\" for \"{}\" is missing on filesystem",
            config.input_path, config.nickname
        );
        return false;
    }

    if let Some(dir) = &config.output_directory
        && !path_exists(dir) {
            print!(
                "[!] Given output directory \"{}\" for \"{}\" is missing on filesystem",
                dir, config.nickname
            );
            return false;
        }

    println!("valid!");

    true
}
