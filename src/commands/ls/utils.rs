use std::path::{Path, PathBuf};

pub fn cleaner(s: String) -> String {
    let mut cleaned_str = String::new();

    for ch in s.chars() {
        if ch.is_alphanumeric() {
            cleaned_str.push_str(&ch.to_lowercase().to_string());
        }
    }
    cleaned_str
}

pub fn get_absolute_parent(dir_path: &Path) -> Option<PathBuf> {
    match dir_path.canonicalize() {
        Ok(absolute_path) => absolute_path.parent().map(|p| p.to_path_buf()),
        Err(_) => None,
    }
}