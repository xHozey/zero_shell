use std::path::Path;
use std::fs::{copy};

pub fn cp(s: String) {
    let spt: Vec<&str> = s.split_whitespace().collect();
    if spt.len() == 0 {
        eprintln!("cp: missing file operand");
        return
    }
    if spt.len() == 1 {
        eprintln!("cp: missing destination file operand after '{}'", spt[0]);
        return
    }
    if spt.len() == 2 {
        let dis = Path::new(spt[1]);
        if dis.is_dir() {
            if let Err(err) = copy(spt[0], dis.join(Path::new(spt[0]))) {
                eprintln!("cp: {}", err.to_string().to_ascii_lowercase())
            }
        } else {
            if let Err(err) = copy(spt[0], dis) {
                eprintln!("cp: {}", err.to_string().to_ascii_lowercase())
            }
        }
    } else {
        let dis = Path::new(spt[spt.len()-1]);
        if !dis.is_dir() {
            eprintln!("cp: target '{}' is not a directory", spt[spt.len()-1]);
            return
        }
        for i in 0..spt.len()-1 {
            if Path::new(spt[i]).is_dir() {
                eprintln!("cp: omitting directory '{}'", spt[i]);
                return
            }
        }
        for i in 0..spt.len()-1 {
            let p = Path::new(spt[i]);
            if let Err(err) = copy(p, dis.join(Path::new(p))) {
             eprintln!("cp: {}", err.to_string().to_ascii_lowercase())
        }
                
        }
        
    }
}