use std::env;
use std::path::Path;
pub fn cd(s: String) {
    let path = Path::new(&s);
    if let Err(err) = env::set_current_dir(&path) {
        eprintln!("cd: {}", err.to_string().to_ascii_lowercase())
    }
}
