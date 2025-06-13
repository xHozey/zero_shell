use std::path::Path;
use std::fs::{copy};

pub fn cp(arg: String) {
    let spt: Vec<&str> = arg.split_whitespace().collect();
    if spt.len() == 0 {
        println!("cp: missing file operand");
        return
    }
    if spt.len() == 1 {
        println!("missing destination file operand after '{}'", spt[0]);
        return
    }
    if spt.len() == 2 {
        let dis = Path::new(spt[1]);
        if dis.is_dir() {
            match copy(spt[0], dis.join(Path::new(spt[0]))) {
                Ok(_) => {} 
                Err(err) => println!("{}", err.to_string())
            }
        } else {
            match copy(spt[0], dis) {
                Ok(_) => {} 
                Err(err) => println!("{}", err.to_string())
            }
        }
    } else {
        let dis = Path::new(spt[spt.len()-1]);
        if !dis.is_dir() {
            println!("cp: target '{}' is not a directory", spt[spt.len()-1]);
            return
        }
        for i in 0..spt.len()-1 {
            if Path::new(spt[i]).is_dir() {
                println!("cp: omitting directory '{}'", spt[i]);
                return
            }
        }
        for i in 0..spt.len()-1 {
            let p = Path::new(spt[i]);
            match copy(p, dis.join(Path::new(p))) {
                Ok(_) => {} 
            Err(err) => println!("{}", err.to_string())
        }
                
        }
        
    }
}