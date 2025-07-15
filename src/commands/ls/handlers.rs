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
