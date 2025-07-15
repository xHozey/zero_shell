use std::path::Path;

use crate::commands::ls::parser::Flags;

pub fn handle_files(files: &Vec<String>, flags: &Flags) -> Result<Vec<Vec<String>>, ()> {
    let mut infos = Vec::new();
    for file in files {
        let file_path = Path::new(file);
        println!("{:?}", file_path);

        if flags.l {
            let detailed_info = get_detailed_info(&file_path);
            infos.push(vec![detailed_info]);
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

        if flags.a {
            let hiden_files = match get_hidden_files(&dir_path, &flags) {
                Ok(files) => files,
                Err(()) => return Err(()),
            };
            infos.push(hiden_files);
        }

        let all_files = match get_files(&dir_path, &flags) {
            Ok(files) => files,
            Err(()) => return Err(()),
        };
        infos.push(all_files);
    }
    println!("infoos {:?}", &infos);
    Ok(infos)
}