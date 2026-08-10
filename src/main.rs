mod config;
mod archive;
mod compression_level;
mod file_filter;
mod progress_bar;

use std::env;
use std::time::SystemTime;

use config::{Item, load_configuration, validate_configuration_entry};
use archive::create_archive;

fn logo() {
    let logo = include_str!("assets/logo.txt");
    println!("{logo}");
}

fn help() {
    let help = include_str!("assets/help.txt");
    println!("{help}");   
}

fn archive_item(config: &Item) -> Result<bool, std::io::Error> {
    let start = SystemTime::now();

    create_archive(config)?;

    let elapsed = SystemTime::now()
        .duration_since(start)
        .expect("Error while computing processing time");

    println!("[i] Job \"{}\" done in {}ms", config.nickname, elapsed.as_millis());
    
    Ok(true)
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    
    if !args.iter().any(|a| a == "--no-logo") {
        logo();
    }

    if args.iter().any(|a| a == "--help") {
        help();

        return Ok(())
    }

    let config_path =  match args.iter().rfind(|entry| entry.starts_with("--config-path")) {
        Some(array) => array.split("=").skip(1).fold(String::new(), |acc, entry| {
            if !acc.is_empty() {
                return format!("{}={}", acc, entry).to_string();
            }

            entry.to_string()
        }),
        None => "config.json".to_string()
    };
    
    let configs = load_configuration(&config_path);

    let valid_config_entries: Vec<&Item> = configs
        .iter()
        .filter(|config: &&config::Item| validate_configuration_entry(config))
        .collect();

    println!("[i] Found {}/{} valid configurations", valid_config_entries.len(), configs.len());

    for config in valid_config_entries {
        println!("[i] Started Job \"{}\"", config.nickname);
        archive_item(config)?;
    }
    
    Ok(())
}
