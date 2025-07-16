use std::{fs::read_dir, path::Path};

use crate::commands::ls::{get_info::get_detailed_info, parser::Flags};

pub fn handle_files(files: &Vec<String>, flags: &Flags) -> Result<Vec<Vec<String>>, ()> {
    let mut infos = Vec::new();
    for file in files {
        let file_path = Path::new(file);
        println!("{:?}", file_path);

        if flags.l {
            let detailed_info = match get_detailed_info(&file_path) {
                Ok(info) => info,
                Err(_) => return Err(()),
            };
            infos.push(detailed_info);
        } else {
            infos.push(vec![file.to_string()]);
        }
    }
    Ok(infos)
}

pub fn handle_dir(dirs: &Vec<String>, flags: &Flags) -> Result<Vec<Vec<String>>, ()> {
    let mut infos = Vec::new();

    for dir in dirs {
        let dir_path = Path::new(dir);

        let all_files = if flags.a {
            match get_hidden_files(&dir_path, &flags) {
                Ok(files) => files,
                Err(()) => return Err(()),
            }
        } else {
            match get_files(&dir_path, &flags) {
                Ok(files) => files,
                Err(()) => return Err(()),
            }
        };

        infos = all_files
    }

    println!("infoos {:?}", &infos);
    Ok(infos)
}

pub fn get_hidden_files(dir_path: &Path, flags: &Flags) -> Result<Vec<Vec<String>>, ()> {
    let mut entries = Vec::new();

    match read_dir(dir_path) {
        Ok(dir_entries) => {
            let mut direntries: Vec<_> = dir_entries.filter_map(|entry| entry.ok()).collect();
            direntries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

            for dir_entry in direntries {
                if let Some(file_name) = dir_entry.file_name().to_str() {
                    println!("{:?}", &dir_entry);
                    if flags.l {
                        let details = match get_detailed_info(&dir_entry.path()) {
                            Ok(info) => info,
                            Err(_) => return Err(()),
                        };
                        entries.push(details);
                    } else {
                        entries.push(vec![file_name.to_string()]);
                    }
                }
            }
        }
        Err(_) => return Err(()),
    };
    println!("{:?}", &entries);

    Ok(entries)
}

pub fn get_files(dir_path: &Path, flags: &Flags) -> Result<Vec<Vec<String>>, ()> {
    let mut entries = Vec::new();
    match read_dir(dir_path) {
        Ok(dir_entries) => {
            let mut direntries: Vec<_> = dir_entries.filter_map(|entry| entry.ok()).collect();
            direntries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

            for dir_entry in direntries {
                dbg!(&dir_entry);
                if let Some(file_name) = dir_entry.file_name().to_str() {
                    if !file_name.starts_with(".") {
                        if flags.l {
                            let details = match get_detailed_info(&dir_entry.path()) {
                                Ok(info) => info,
                                Err(_) => return Err(()),
                            };
                            entries.push(details);
                            println!("{:?}", entries)
                        } else {
                            entries.push(vec![file_name.to_string()]);
                        }
                    }
                }
            }
        }
        Err(_) => return Err(()),
    };
    println!("final {:?}", &entries);
    Ok(entries)
}
