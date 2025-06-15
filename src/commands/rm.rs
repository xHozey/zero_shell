use std::fs;
use std::path::Path;
pub fn rm(s: String) {
    let mut flag = false;
    let args: Vec<&str> = s.split_whitespace().collect();
    if args.contains(&"-r") {
        flag = true
    } 
    for file in args {
        if file == "-r" {
            continue;
        }
        let f = Path::new(file);
        if f.exists() {
            if flag {
                if f.is_dir() {
                    if let Err(err) = fs::remove_dir_all(f) {
                        eprintln!("{}", err.to_string())
                    }
                } else {
                    if let Err(err) = fs::remove_file(f) {
                        eprintln!("{}", err.to_string());
                    }
                }
            } else {
                if f.is_dir() {
                    eprintln!("rm: cannot remove '{}': Is a directory", file)
                } else {
                    if let Err(err) = fs::remove_file(f) {
                        eprintln!("{}", err.to_string());
                    }
                }
            }
        } else {
            eprintln!("rm: cannot remove '{}': No such file or directory", file)
        }
    }
}