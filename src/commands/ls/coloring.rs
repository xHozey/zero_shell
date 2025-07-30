use std::{
    fs::{metadata, symlink_metadata},
    os::unix::fs::{FileTypeExt, MetadataExt},
    path::{Path, PathBuf},
};

use colored::Colorize;

use crate::commands::ls::parser::Flags;
pub enum Color {
    Brown,
    Blue,
    Green,
    Skybleu,
    Red,
}

pub fn coloring(file: &String, color: Color) -> String {
    let colored_file = match color {
        Color::Brown => file.truecolor(150, 105, 25).to_string(),
        Color::Blue => file.blue().to_string(),
        Color::Green => file.green().to_string(),
        Color::Skybleu => file.truecolor(0, 255, 255).to_string(),
        Color::Red => file.truecolor(255, 0, 255).to_string(),
    };
    colored_file
}

pub fn color_devices(file_name: &String, color: Color) -> String {
    coloring(file_name, color).on_black().to_string()
}

pub fn color_dir(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f {
        colored_name.push('/');
    }
    colored_name
}

pub fn color_symlink(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f && !flags.l {
        colored_name.push('@');
    }
    colored_name
}

pub fn color_exec_file(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f {
        colored_name.push('*');
    }
    colored_name
}

pub fn color_pipe(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f {
        colored_name.push('|');
    }
    colored_name
}

pub fn color_socket(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f {
        colored_name.push('=');
    }
    colored_name
}

pub fn colored_output(file: &String, dir_name: Option<&str>, flags: &Flags) -> String {
    if file.contains(" -> ") {
        let parts: Vec<&str> = file.split(" -> ").collect();
        let symlink_name = parts[0].to_string();
        let target_path = parts[1];

        let colored_name = color_symlink(&symlink_name, Color::Skybleu, flags);
        return format!(
            "{} -> {}",
            colored_name,
            coloring_target(target_path, flags, dir_name)
        );
    }

    let full_path = match dir_name {
        Some(dir) => {
            if file == "." {
                ".".to_string()
            } else {
                Path::new(dir).join(file).to_string_lossy().to_string()
            }
        }
        None => file.clone(),
    };

    let path = Path::new(&full_path);

    // Use the original filename for display
    let file_name = match path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => file.clone(),
    };

    let metadata = match symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return file_name,
    };

    let file_type = metadata.file_type();
    let mode = metadata.mode();

    if file_type.is_dir() {
        color_dir(&file_name, Color::Blue, flags)
    } else if file_type.is_fifo() {
        color_pipe(&file_name, Color::Brown, flags)
    } else if file_type.is_block_device() || file_type.is_char_device() {
        color_devices(&file_name, Color::Brown)
    } else if file_type.is_socket() {
        color_socket(&file_name, Color::Red, flags)
    } else if file_type.is_symlink() {
        color_symlink(&file_name, Color::Skybleu, flags)
    } else {
        if mode & 0o111 != 0 {
            color_exec_file(&file_name, Color::Green, flags)
        } else {
            file_name
        }
    }
}

fn coloring_target(target: &str, flags: &Flags, dir_name: Option<&str>) -> String {
    let mut target_path = PathBuf::from(target);
    if target_path.is_relative() {
        target_path = match dir_name {
            Some(dir) => Path::new(dir).join(target_path),
            None => target_path,
        }
    }

    let metadata = match metadata(target_path) {
        Ok(metadata) => metadata,
        Err(_) => return target.to_string(),
    };

    let file_name = target.to_string();
    let file_type = metadata.file_type();
    let mode = metadata.mode();

    if file_type.is_dir() {
        color_dir(&file_name, Color::Blue, flags)
    } else if file_type.is_fifo() {
        color_pipe(&file_name, Color::Brown, flags)
    } else if file_type.is_block_device() || file_type.is_char_device() {
        color_devices(&file_name, Color::Brown)
    } else if file_type.is_socket() {
        color_socket(&file_name, Color::Red, flags)
    } else if file_type.is_symlink() {
        color_symlink(&file_name, Color::Skybleu, flags)
    } else {
        if mode & 0o111 != 0 {
            color_exec_file(&file_name, Color::Green, flags)
        } else {
            file_name
        }
    }
}
