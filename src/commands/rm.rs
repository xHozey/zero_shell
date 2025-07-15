use std::fs;
use std::path::Path;
pub fn rm(s: Vec<String>) {
    let mut flag = false;
    if s.contains(&String::from("-r")) {
        flag = true
    }
    for file in s {
        if file == "-r" {
            continue;
        }
        let f = Path::new(&file);
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
