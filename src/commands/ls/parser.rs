use std::path::Path;

#[derive(Debug, Clone)]
pub struct Flags {
    pub a: bool,
    pub l: bool,
    pub f: bool,
}
pub fn parse_ls_args(
    args: Vec<String>,
    files: &mut Vec<String>,
    dirs: &mut Vec<String>,
    flags: &mut Flags,
) -> Result<(), ()> {
    for arg in args {
        if arg.starts_with("-") {
            for char in arg.chars().skip(1) {
                match char {
                    'a' => {
                        flags.a = true;
                    }
                    'l' => {
                        flags.l = true;
                    }
                    'F' => {
                        flags.f = true;
                    }
                    _ => return Err(()),
                }
            }
        } else {
            if Path::new(&arg).is_dir() {
                dirs.push(arg);
            } else if Path::new(&arg).exists() {
                files.push(arg);
            } else {
                return Err(());
            }
        }
    }

    if dirs.is_empty() && files.is_empty() {
        dirs.push(".".to_string());
    }

    Ok(())
}