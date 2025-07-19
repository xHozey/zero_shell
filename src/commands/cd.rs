use std::env;
use std::path::PathBuf;

pub fn cd(input: String) {
    let trimmed = input.trim();

    let target_path = if trimmed.is_empty() || trimmed == "~" {
        match env::var("HOME") {
            Ok(home) => PathBuf::from(home),
            Err(_) => {
                eprintln!("cd: HOME not set");
                return;
            }
        }
    } else if trimmed.starts_with("~/") {
        match env::var("HOME") {
            Ok(home) => {
                let mut home_path = PathBuf::from(home);
                home_path.push(trimmed.trim_start_matches("~/"));
                home_path
            }
            Err(_) => {
                eprintln!("cd: HOME not set");
                return;
            }
        }
    } else {
        PathBuf::from(trimmed)
    };

    if let Err(err) = env::set_current_dir(&target_path) {
        eprintln!("cd: {}", err.to_string().to_ascii_lowercase());
    }
}
