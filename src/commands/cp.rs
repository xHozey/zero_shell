use std::path::Path;
use std::fs::{copy};

pub fn cp(arg: String) {
    let spt: Vec<&str> = arg.split_whitespace().collect();
    if spt.len() == 0 {
        eprintln!("cp: missing file operand");
        return
    }
    if spt.len() == 1 {
        eprintln!("missing destination file operand after '{}'", spt[0]);
        return
    }
    if spt.len() == 2 {
        let dis = Path::new(spt[1]);
        if dis.is_dir() {
            if let Err(err) = copy(spt[0], dis.join(Path::new(spt[0]))) {
                eprintln!("{}", err.to_string())
            }
        } else {
            if let Err(err) = copy(spt[0], dis) {
                eprintln!("{}", err.to_string())
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
             eprintln!("{}", err.to_string())
        }
                
        }
        
    }
}