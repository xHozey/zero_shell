use std::fs::create_dir;

pub fn mkdir(path: String) {
    for p in path.split_whitespace() {
        match create_dir(p) {
            Ok(_) => {},
            Err(err) => println!("{}", err.to_string())
        }
    }
}