use std::fs::create_dir;

pub fn mkdir(path: String) {
    for p in path.split_whitespace() {
        if let Err(err) = create_dir(p) {
            eprintln!("{}", err.to_string())
        }
    }
}