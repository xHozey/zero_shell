use std::fs::create_dir;

pub fn mkdir(s: Vec<String>) {
    for p in s{
        if let Err(err) = create_dir(p) {
            eprintln!("mkdir: {}", err.to_string().to_ascii_lowercase())
        }
    }
}
