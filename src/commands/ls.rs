use std::path::Path;

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
    
}