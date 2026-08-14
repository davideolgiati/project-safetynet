mod archive;
mod cli;
mod config;
mod error;
pub mod logger;
mod progress_bar;

use std::env;

use archive::job::archive_item;
use cli::{config_path, help, logo};
use config::load_configuration;
use error::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if !args.iter().any(|a| a == "--no-logo") {
        logo();
    }

    if args.iter().any(|a| a == "--help") {
        help();
        return Ok(())
    }

    let config_path = config_path(&args);

    info!("Using \"{}\" as configuration file", config_path);

    let configs = load_configuration(&config_path)?;

    for config in configs {
        info!("Started Job \"{}\"", config.nickname);
        archive_item(&config)?;
    }

    Ok(())
}
