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
    let mut res = String::new();
    for path in &paths {
        if !path.exists() {
            eprintln!(
                "ls: cannot access '{}': No such file or directory",
                path.display()
            );
            continue;
        }
        if path.is_dir() {
            match fs::read_dir(path) {
                Ok(entries) => {
                   let mut entries: Vec<_> = entries.collect();
                    entries.sort_by_key(|e| {
                         e.as_ref()
                        .map(|e| {
                            let name = e.file_name().to_string_lossy().to_string();
                            let sort_key = name.strip_prefix('.').unwrap_or(&name);
                            sort_key.to_lowercase()  
                        })
                        .unwrap_or_default()
                     });
                    let mut buffer = String::new();
                    if all {
                        buffer.push_str(". .. ")
                    }
                    for entry in entries {
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
                    if paths.len() > 1 {
                        res.push_str(&format!("{}:\n", path.display()));
                        res.push_str(&format!("{}\n\n", buffer.trim()));
                    } else {
                        res.push_str(buffer.trim());
                    }
                },
                Err(err) => eprintln!("ls: {}", err.to_string().to_ascii_lowercase())
            }
        } else {
        }
    }
    println!("{}", res.trim())
}
