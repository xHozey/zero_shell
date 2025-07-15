use crate::commands::ls::parser::{parse_ls_args, Flags};

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

    println!("{:?}  {:?}    {:?}", files, dirs, flags)
}
