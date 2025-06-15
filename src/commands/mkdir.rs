use std::fs::create_dir;

pub fn mkdir(s: String) {
    for p in s.split_whitespace() {
        if let Err(err) = create_dir(p) {
            eprintln!("mkdir: {}", err.to_string().to_ascii_lowercase())
        }
    }
}
