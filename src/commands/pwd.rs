use std::env;

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(err) => println!("{:?}", err.to_string())
    }
}