use crate::commands::ls::{
    handlers::handle_files,
    parser::{parse_ls_args, Flags},
};

pub fn ls(args: Vec<String>) {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let mut flags = Flags {
        a: false,
        l: false,
        f: false,
    };

    if let Err(()) = parse_ls_args(args, &mut files, &mut dirs, &mut flags) {
        println!("Error parssing arguments");
        return;
    };

    let files_infos = handle_files(&files, &flags);
    println!("informat{:?}", files_infos);

    let dir_infos = handle_dir(&dirs, &flags);
    println!(" dir_infos : {:?}", dir_infos);

    println!("{:?}  {:?}    {:?}", files, dirs, flags)
}
