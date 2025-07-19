use crate::commands::ls::{display::display, handlers::*, parser::*};

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

    let files_infos = match handle_files(&mut files, &flags) {
        Ok(infos) => infos,
        Err(_) => Vec::new(),
    };

    let dir_infos = match handle_dir(&dirs, &flags) {
        Ok(infos) => infos,
        Err(_) => Vec::new(),
    };

    display(files_infos, dir_infos, flags);
}
