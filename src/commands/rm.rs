use std::fs;
use std::path::Path;

pub fn rm(args: Vec<String>) {
    if args.is_empty() || (args.len() == 1 && args[0] == "-r") {
        eprintln!("rm: missing operand");
        return;
    }
    let mut flag = false;
    if args.contains(&"-r".to_string()) {
        flag = true
    }
    
    for file in args {
        if file == "-r" {
            continue;
        }

        let f = Path::new(&file);
        if flag && (f == Path::new(".") || f == Path::new("..")) {
            eprintln!(
                "rm: refusing to remove '.' or '..' directory: skipping '{}'",
                file
            );
            continue;
        }

        if f.exists() {
            if flag {
                if f.is_dir() {
                    if let Err(err) = fs::remove_dir_all(f) {
                        eprintln!("rm: {}", err.to_string().to_ascii_lowercase())
                    }
                } else {
                    if let Err(err) = fs::remove_file(f) {
                        eprintln!("rm: {}", err.to_string().to_ascii_lowercase());
                    }
                }
            } else {
                if f.is_dir() {
                    eprintln!("rm: cannot remove '{}': Is a directory", file)
                } else {
                    if let Err(err) = fs::remove_file(f) {
                        eprintln!("rm: {}", err.to_string().to_ascii_lowercase());
                    }
                }
            }
        } else {
            eprintln!("rm: cannot remove '{}': No such file or directory", file)
        }
    }
}
