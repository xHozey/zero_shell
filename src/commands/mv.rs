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
    let dis = Path::new(&args[args.len() - 1]);
    for i in 0..args.len() - 1 {
        if let Err(err) = rename(&args[i], dis.join(Path::new(&args[i]))) {
            eprintln!("mv: {}", err.to_string().to_ascii_lowercase())
        }
    }
}
