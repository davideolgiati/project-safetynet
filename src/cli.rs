pub fn logo() {
    let logo = include_str!("assets/logo.txt");
    println!("{logo}");
}

pub fn help() {
    let help = include_str!("assets/help.txt");
    println!("{help}");
}

pub fn config_path(args: &[String]) -> String {
    match args.iter().rfind(|entry| entry.starts_with("--config-path")) {
        Some(array) => array.split("=").skip(1).fold(String::new(), |acc, entry| {
            if !acc.is_empty() {
                return format!("{}={}", acc, entry).to_string();
            }

            entry.to_string()
        }),
        None => "config.json".to_string()
    }
}