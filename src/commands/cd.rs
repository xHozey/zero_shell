use std::env;
use std::path::Path;
pub fn cd(args: Vec<String>) {
    if args.is_empty() {
        if let Some(home) = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")) {
            if let Err(err) = env::set_current_dir(Path::new(&home)) {
                eprintln!("cd: {}", err.to_string().to_ascii_lowercase())
            }
        } else {
            eprintln!("cd: home directory not found")
        }
        return;
    }
    if args.len() > 1 {
        eprintln!("cd: string not in pwd: {}", args[0]);
        return;
    }
    if let Err(err) = env::set_current_dir(Path::new(&args[0])) {
        eprintln!("cd: {}", err.to_string().to_ascii_lowercase())
    }
}
