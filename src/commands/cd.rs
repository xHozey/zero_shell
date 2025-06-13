use std::env;
use std::path::Path;
pub fn cd(dir: String) {
    let path = Path::new(&dir);
    match env::set_current_dir(&path) {
        Ok(_) => {},
        Err(err) => println!("{}", err.to_string())
    }
}