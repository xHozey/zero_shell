use std::fs::copy;
use std::path::Path;

pub fn cp(s: Vec<String>) {
    // let spt: Vec<&str> = s;
    if s.len() == 0 {
        eprintln!("cp: missing file operand");
        return;
    }
    if s.len() == 1 {
        eprintln!("cp: missing destination file operand after '{}'", s[0]);
        return;
    }
    if s.len() == 2 {
        let dis = Path::new(&s[1]);
        if dis.is_dir() {
            if let Err(err) = copy(s[0].clone(), dis.join(Path::new(&s[0]))) {
                eprintln!("cp: {}", err.to_string().to_ascii_lowercase())
            }
        } else {
            if let Err(err) = copy(s[0].clone(), dis) {
                eprintln!("cp: {}", err.to_string().to_ascii_lowercase())
            }
        }
    } else {
        let dis = Path::new(&s[s.len() - 1]);
        if !dis.is_dir() {
            eprintln!("cp: target '{}' is not a directory", s[s.len() - 1]);
            return;
        }
        for i in 0..s.len() - 1 {
            if Path::new(&s[i]).is_dir() {
                eprintln!("cp: omitting directory '{}'", s[i]);
                return;
            }
        }
        for i in 0..s.len() - 1 {
            let p = Path::new(&s[i]);
            if let Err(err) = copy(p, dis.join(Path::new(p))) {
                eprintln!("cp: {}", err.to_string().to_ascii_lowercase())
            }
        }
    }
}
