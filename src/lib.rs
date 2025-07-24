use std::env;
use std::io::*;

pub fn parse_command(s: &str) -> (String, String) {
        match s.trim().split_once(' ') {
            Some((cmd, str)) => return (cmd.to_string(), str.to_string()),
            None => return (s.to_string(), "".to_string()),
        }
}

pub fn format_prompt() {
    match env::current_dir() {
        Ok(path) => match path.file_name() {
            Some(name) => print!("{} $ ", name.to_str().unwrap()),
            None => print!("{} $ ", path.display()),
        },
        Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
    }
}

pub fn add_buffer_format() {
    print!(">");
    if let Err(err) = stdout().flush() {
        eprintln!("{}", err.to_string().to_ascii_lowercase())
    }
}