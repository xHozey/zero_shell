use std::env;
use std::path::{Path, PathBuf};

pub fn cd(input: Vec<String>, last_path: &PathBuf) -> PathBuf {
    
    if input.len() > 1 {
        eprintln!("cd: too many arguments");
        return last_path.clone();
    }

    let s = input.join(" ");
    let trimmed = s.trim();

    let target_path = if trimmed == "-" {
        println!("{}", last_path.display());
        last_path.clone()
    } else if trimmed.is_empty() || trimmed == "~" {
        match env::var("HOME") {
            Ok(home) => PathBuf::from(home),
            Err(_) => {
                eprintln!("cd: HOME not set");
                return last_path.clone();
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
                return last_path.clone();
            }
        }
    } else {
        PathBuf::from(trimmed)
    };

    if let Err(_) = env::current_dir() {
        if target_path == Path::new("..") {
            eprintln!("cd: can't cd to {}", target_path.display());
            return last_path.clone();
        }
    }
    let current_path = env::current_dir().unwrap();
    if let Err(err) = env::set_current_dir(&target_path) {
        eprintln!("cd: {}", err.to_string().to_ascii_lowercase());
    }
    return current_path;
}
