use std::fs::copy;
use std::path::Path;

pub fn cp(args: Vec<String>) {
    if args.len() == 0 {
        eprintln!("cp: missing file operand");
        return;
    }
    if args.len() == 1 {
        eprintln!("cp: missing destination file operand after '{}'", args[0]);
        return;
    }
    if args.len() == 2 {
        let dis = Path::new(&args[1]);
        if dis.is_dir() {
            if let Err(err) = copy(&args[0], dis.join(Path::new(&args[0]))) {
                eprintln!("cp: {}", err.to_string().to_ascii_lowercase())
            }
        } else {
            if let Err(err) = copy(&args[0], dis) {
                eprintln!("cp: {}", err.to_string().to_ascii_lowercase())
            }
        }
    } else {
        let dis = Path::new(&args[args.len() - 1]);
        if !dis.is_dir() {
            eprintln!("cp: target '{}' is not a directory", args[args.len() - 1]);
            return;
        }
        for i in 0..args.len() - 1 {
            if Path::new(&args[i]).is_dir() {
                eprintln!("cp: omitting directory '{}'", args[i]);
                return;
            }
        }
        for i in 0..args.len() - 1 {
            let p = Path::new(&args[i]);
            if let Err(err) = copy(p, dis.join(Path::new(p))) {
                eprintln!("cp: {}", err.to_string().to_ascii_lowercase())
            }
        }
    }
}
