use std::{
    fs::{read_dir, DirEntry},
    os::unix::fs::MetadataExt,
    path::Path,
};

use crate::commands::ls::{get_info::get_detailed_info, parser::Flags};

#[derive(Debug)]
#[allow(dead_code)]
pub struct DirInfo {
    pub entries: Vec<Vec<String>>,
    pub total_blocks: u64,
    pub dir_name: String,
}

pub fn handle_files(files: &Vec<String>, flags: &Flags) -> Result<Vec<Vec<String>>, ()> {
    let mut infos = Vec::new();
    for file in files {
        let file_path = Path::new(file);

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

pub fn handle_dir(dirs: &Vec<String>, flags: &Flags) -> Result<Vec<DirInfo>, ()> {
    let mut infos = Vec::new();

    for dir in dirs {
        let dir_path = Path::new(dir);

        let entries = match get_dir_entries(&dir_path, &flags) {
            Ok(files) => files,
            Err(()) => return Err(()),
        };

        infos.push(entries)
    }

    Ok(infos)
}

pub fn get_dir_entries(dir_path: &Path, flags: &Flags) -> Result<DirInfo, ()> {
    let mut result = Vec::new();
    let mut total_blocks = 0;

    match read_dir(dir_path) {
        Ok(dir_entries) => {
            let mut entries: Vec<_> = dir_entries.filter_map(|entry| entry.ok()).collect();
            entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

            for entry in entries {
                if let Some(file_name) = entry.file_name().to_str() {
                    if !flags.a && file_name.starts_with(".") {
                        continue;
                    }

                    total_blocks += get_total_blocks(&entry);

                    if flags.l {
                        let details = match get_detailed_info(&entry.path()) {
                            Ok(info) => info,
                            Err(_) => return Err(()),
                        };
                        result.push(details);
                    } else {
                        result.push(vec![file_name.to_string()]);
                    }
                }
            }
        }
        Err(_) => return Err(()),
    };

    Ok(DirInfo {
        entries: result,
        total_blocks,
        dir_name: dir_path.display().to_string(),
    })
}

fn get_total_blocks(entry: &DirEntry) -> u64 {
    if let Ok(metadata) = entry.metadata() {
        let blocks = metadata.blocks();
        (blocks + 1) / 2 // +1 for rounding
    } else {
        0
    }
}
