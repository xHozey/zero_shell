use std::{
    fs::symlink_metadata,
    os::unix::fs::{FileTypeExt, MetadataExt},
    path::Path,
};

use crate::commands::ls::{handlers::DirInfo, parser::Flags};

pub fn display(files_info: Vec<Vec<String>>, dirs_info: Vec<DirInfo>, flags: Flags) {
    if !files_info.is_empty() {
        if flags.l {
            format_output(files_info);
        } else {
            for infos in files_info {
                let file_name = infos[0].clone();
                if flags.f {
                    let type_indicator = add_file_type_indicator(&file_name);
                    print!("{}{}  ", file_name, type_indicator)
                } else {
                    print!("{}  ", file_name)
                }
            }
            println!()
        }
    }

    // if !dirs_info.is_empty() {
    //     for dir in dirs_info {
    //         if flags.l {
    //             println!("total {}", dir.total_blocks);

    //             format_output(dir.entries);
    //         } else {
    //             println!("{}" , dir.dir_name)
    //         }
    //     }
    // }
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

fn add_file_type_indicator(file: &String) -> String {
    let path = Path::new(file);
    let metadata = symlink_metadata(path).unwrap();
    let file_type = metadata.file_type();
    let mode = metadata.mode();
    if file_type.is_dir() {
        return "/".to_string();
    } else if file_type.is_symlink() {
        return "@".to_string();
    } else if file_type.is_fifo() {
        return "|".to_string();
    } else if file_type.is_socket() {
        return "=".to_string();
    } else {
        if mode & 0o111 != 0 {
            return "*".to_string();
        } else {
            "".to_string()
        }
    }
}

fn format_output(infos: Vec<Vec<String>>) {
    let cols_width = get_max_width(&infos);
    for infos in infos {
        for (idx, w) in cols_width.iter().enumerate() {
            if idx == 0 {
                //left aligned
                print!("{:>width$}", infos[idx], width = *w);
            } else if idx == cols_width.len() - 1 {
                // no extra spacing
                print!(" {}", infos[idx]);
            } else {
                //right aligned with space
                print!(" {:<width$}", infos[idx], width = *w);
            }
        }
        println!();
    }
}
