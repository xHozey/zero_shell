use std::path::{Path, PathBuf};

use terminal_size::{terminal_size, Width};

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

pub fn get_terminal_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        20
    }
}

pub fn get_max_width(infos: &Vec<Vec<String>>) -> Vec<usize> {
    let mut cols_width = Vec::new();
    for j in 0..infos[0].len() {
        let mut max_width = 0;
        for i in 0..infos.len() {
            max_width = max_width.max(infos[i][j].len());
        }
        cols_width.push(max_width);
    }
    cols_width
}
