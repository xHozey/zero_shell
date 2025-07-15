use std::env;
use std::path::Path;
pub fn cd(args: Vec<String>) {
    for s in args {
        let path = Path::new(&s);
        if let Err(err) = env::set_current_dir(&path) {
            eprintln!("cd: {}", err.to_string().to_ascii_lowercase())
        }
    }
}
