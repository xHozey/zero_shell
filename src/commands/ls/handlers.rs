use std::{fs::read_dir, path::Path};

use crate::commands::ls::{
    get_info::{get_detailed_info, get_total_blocks},
    parser::Flags, utils::{cleaner, get_absolute_parent},
};

#[derive(Debug)]
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
            entries.sort_by(|a, b| {
                let a_new = cleaner(a.file_name().to_string_lossy().to_string());
                let b_new = cleaner(b.file_name().to_string_lossy().to_string());

                a_new.cmp(&b_new)
            });

            if flags.a {
                add_dots(
                    flags,
                    &mut result,
                    &mut total_blocks,
                    dir_path,
                    ".".to_string(),
                );
                add_dots(
                    flags,
                    &mut result,
                    &mut total_blocks,
                    &get_absolute_parent(dir_path).unwrap_or(dir_path.to_path_buf()),
                    "..".to_string(),
                );
            }

            for entry in entries {
                if let Some(file_name) = entry.file_name().to_str() {
                    if !flags.a && file_name.starts_with(".") {
                        continue;
                    }

                    if flags.l {
                        total_blocks += get_total_blocks(&entry.path());

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

fn add_dots(
    flags: &Flags,
    result: &mut Vec<Vec<String>>,
    total_blocks: &mut u64,
    dots: &Path,
    dot_name: String,
) {
    if flags.l {
        *total_blocks += get_total_blocks(dots);

        let mut details = match get_detailed_info(dots) {
            Ok(info) => info,
            Err(_) => return,
        };
        details.pop();
        details.push(dot_name);
        result.push(details);
    } else {
        result.push(vec![dot_name]);
    }
}
