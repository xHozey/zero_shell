use std::fs::create_dir;

pub fn mkdir(args: Vec<String>) {
    for p in args {
        if let Err(err) = create_dir(p) {
            eprintln!("mkdir: {}", err.to_string().to_ascii_lowercase())
        }
    }
}
