use std::{fs, path::Path};

pub fn ls(s: String) {
    let args: Vec<&str> = s.split_whitespace().collect();
    let mut long = false;
    let mut all = false;
    let mut classify = false;
    let mut paths = Vec::new();
    for arg in args {
        if arg.starts_with('-') {
            for ch in arg.chars().skip(1) {
                match ch {
                    'l' => long = true,
                    'a' => all = true,
                    'F' => classify = true,
                    _ => eprintln!("ls: unknown flag: -{}", ch),
                }
            }
        } else {
            paths.push(Path::new(arg))
        }
    }
    if paths.is_empty() {
        paths.push(Path::new("."))
    }
    for path in paths {
        if !path.exists() {
            eprintln!(
                "ls: cannot access '{}': No such file or directory",
                path.display()
            );
            continue;
        }
        if path.is_dir() {
            match fs::read_dir(path) {
                Ok(t) => {
                    let mut buffer = String::new();
                    for entry in t {
                        if let Ok(entry) = entry {
                            let file_name = entry.file_name();
                            let file_name_str = file_name.to_string_lossy();
                            if !all && file_name_str.starts_with(".") {
                                continue;
                            }
                            buffer.push_str(&file_name_str);
                            buffer.push(' ');
                        }
                    }
                    println!("{}", buffer.trim())
                },
                Err(err) => eprintln!("ls: {}", err.to_string().to_ascii_lowercase())
            }
        } else {
        }
    }
}
