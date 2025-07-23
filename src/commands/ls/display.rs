use std::{
    fs::symlink_metadata,
    os::unix::fs::{FileTypeExt, MetadataExt},
    path::Path,
};

use crate::commands::ls::{coloring::*, handlers::DirInfo, parser::Flags};

pub fn display(files_info: Vec<Vec<String>>, dirs_info: Vec<DirInfo>, flags: Flags) {
    if !files_info.is_empty() {
        if flags.l {
            format_output(files_info, &flags);
        } else {
            for infos in files_info {
                let file_name = infos[0].clone();
                if flags.f {
                    let name = add_file_type_indicator(&file_name, &flags);
                    print!("{}  ", name)
                } else {
                    print!("{}  ", file_name)
                }
            }
            println!()
        }
    }

    if !dirs_info.is_empty() {
        for dir in dirs_info {
            if flags.l {
                println!("total {}", dir.total_blocks);

                format_output(dir.entries, &flags);
            } else {
                println!("{}", dir.dir_name)
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

fn add_file_type_indicator(file: &String, flags: &Flags) -> String {
    let path = Path::new(file);
    let metadata = match symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(_) => return file.to_string(),
    };
    let file_type = metadata.file_type();
    let mode = metadata.mode();

    if file_type.is_dir() {
        color_dir(file, Color::Blue, flags)
    } else if file_type.is_symlink() {
        color_symlink(file, Color::Skybleu, flags)
    } else if file_type.is_fifo() {
        color_pipe(file, Color::Browen, flags)
    } else if file_type.is_socket() {
        color_socket(file, Color::Red, flags)
    } else if file_type.is_block_device() || file_type.is_char_device() {
        color_devices(file, Color::Browen)
    } else {
        if mode & 0o111 != 0 {
            color_exec_file(file, Color::Green, flags)
        } else {
            file.to_string()
        }
    }
}

fn format_output(infos: Vec<Vec<String>>, flags: &Flags) {
    let cols_width = get_max_width(&infos);
    for infos in infos {
        for (idx, w) in cols_width.iter().enumerate() {
            if idx == 0 {
                //left aligned
                print!("{:<width$}", infos[idx], width = *w);
            } else if idx == cols_width.len() - 1 {
                if flags.f {
                    let file_name = add_file_type_indicator(&infos[idx], flags);
                    print!(" {}", file_name);
                } else {
                    // no extra spacing
                    print!(" {}", infos[idx]);
                }
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
