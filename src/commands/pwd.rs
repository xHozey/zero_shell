use std::env;

pub fn pwd() {
    let path = env::current_dir().unwrap();
    println!("{}", path.display());
}