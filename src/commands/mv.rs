use std::fs::rename;
use std::path::Path;

pub fn mv(args: Vec<String>) {
    if args.len() == 0 {
        eprintln!("mv: missing file operand");
        return;
    }
    
    if args.len() == 1 {
        eprintln!("mv: missing destination file operand after '{}'", args[0]);
        return;
    }

    if args.len() == 2 {
        let src = Path::new(&args[0]);
        let dst = Path::new(&args[1]);
        if dst.is_dir() {
            if let Err(err) = rename(src, dst.join(Path::new(&src))) {
                eprintln!("mv: {}", err.to_string().to_ascii_lowercase())
            }
        } else {
            if let Err(err) = rename(src, &dst) {
                eprintln!("mv: {}", err);
            }
        }

        return;
    }

    let dis = Path::new(&args[args.len() - 1]);
    if !dis.is_dir() {
        eprintln!(
            "mv: target '{}' is not a directory",
            dis.file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("<invalid utf8>")
        );
        return;
    }
    for i in 0..args.len() - 1 {
        if let Err(err) = rename(&args[i], dis.join(Path::new(&args[i]))) {
            eprintln!("mv: {}", err.to_string().to_ascii_lowercase())
        }
    }
}
