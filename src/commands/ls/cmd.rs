use crate::commands::ls::{handlers::*, parser::*};

pub fn ls(args: Vec<String>) {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let mut flags = Flags {
        a: false,
        l: false,
        f: false,
    };

    if let Err(e) = parse_ls_args(args, &mut files, &mut dirs, &mut flags) {
        println!("{}", e);
        return;
    };

    if let Ok(files_infos) = handle_files(&files, &flags) {
        for file in files_infos {
            println!("{:?}", file);
        }
    }

    if let Ok(dir_infos) = handle_dir(&dirs, &flags) {
        for dir in dir_infos {
            println!("total {}", dir.total_blocks);
            println!("{}:", dir.dir_name);
            for file in dir.entries {
                println!("{:?}", file);
            }
        }
    };
}
