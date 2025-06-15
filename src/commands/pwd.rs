use std::env;

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => eprintln!("pwd: {}", err.to_string().to_ascii_lowercase())
    }
}