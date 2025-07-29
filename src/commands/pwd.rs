use std::path::PathBuf;

pub fn pwd(path: &PathBuf) {
    println!("{}", path.display());
}
