use std::env;
use std::path::Path;
pub fn cd(dir: String) {
    let path = Path::new(&dir);
    if let Err(err) = env::set_current_dir(&path) {
        eprintln!("{}", err.to_string())
    }
}