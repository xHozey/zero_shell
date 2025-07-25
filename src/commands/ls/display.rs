use std::{
    fs::symlink_metadata,
    os::unix::fs::{FileTypeExt, MetadataExt},
    path::Path,
};

use crate::commands::ls::{coloring::*, handlers::DirInfo, parser::Flags};

pub fn display(files_info: Vec<Vec<String>>, dirs_info: Vec<DirInfo>, flags: Flags) {
    if !files_info.is_empty() {
        if flags.l {
            format_output(&files_info, None, &flags);
        } else {
            for infos in files_info {
                let path_str = infos[0].clone();
                let name = colored_output(&path_str, None, &flags);
                print!("{}  ", name)
            }
            println!()
        }

        if !dirs_info.is_empty() {
            println!();
            for dir in &dirs_info {
                println!("{}:", dir.dir_name)
            }
        }
    }

    if !dirs_info.is_empty() {
        for (idx, dir) in dirs_info.iter().enumerate() {
            if dirs_info.len() >= 2 {
                println!("{}:", &dir.dir_name);
            }
            if flags.l {
                println!("total {}", dir.total_blocks);
                format_output(&dir.entries, Some(&dir.dir_name), &flags)
            } else {
                for infos in &dir.entries {
                    let path_str = infos[0].clone();

                    let name = colored_output(&path_str, None, &flags);
                    print!("{}  ", name);
                }
                println!();
            }
            if idx != dirs_info.len() - 1 {
                println!();
            }
        }
    }
}

fn get_max_width(infos: &Vec<Vec<String>>) -> Vec<usize> {
    let mut cols_width = Vec::new();
    for j in 0..infos[0].len() {
        let mut max_width = 0;
        for i in 0..infos.len() {
            if j < infos[i].len() {
                max_width = max_width.max(infos[i][j].len());
            }
        }
        cols_width.push(max_width);
    }
    cols_width
}

fn colored_output(file: &String, dir_name: Option<&str>, flags: &Flags) -> String {
    dbg!(file);
    if file.contains(" -> ") {
        let parts: Vec<&str> = file.split(" -> ").collect();
        if parts.len() == 2 {
            let symlink_name = parts[0].to_string();
            let target_path = parts[1];

            let colored_name = color_symlink(&symlink_name, Color::Skybleu, flags);
            return format!("{} -> {}", colored_name, target_path);
        }
    }

    // Construct full path for metadata
    let full_path = match dir_name {
        Some(dir) => {
            if file == "." {
                dir.to_string()
            } else if file == ".." {
                Path::new(dir)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| dir.to_string())
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

    // Use full path for metadata
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
    } else if file_type.is_socket() {
        color_socket(&file_name, Color::Red, flags)
    } else if file_type.is_block_device() || file_type.is_char_device() {
        color_devices(&file_name, Color::Brown)
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

fn format_output(infos: &Vec<Vec<String>>, dir_name: Option<&str>, flags: &Flags) {
    let cols_width = get_max_width(&infos);
    for infos in infos {
        for (idx, w) in cols_width.iter().enumerate() {
            if idx == 0 {
                //left aligned
                print!("{:<width$}", infos[idx], width = *w);
            } else if idx == cols_width.len() - 1 {
                let file_name = colored_output(&infos[idx], dir_name, flags);
                print!(" {}", file_name);
            } else if idx == 1 || idx == 4 {
                print!("{:>width$}", infos[idx], width = *w);
            } else {
                //right aligned with space
                print!(" {:<width$}", infos[idx], width = *w);
            }
        }
        println!();
    }
}
