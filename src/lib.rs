use std::env;

pub fn parse_command(s: &str) -> (String, String) {
    // let mut res: (String, String)> = Vec::new();
    // for spt in s.split("&&") {
        match s.trim().split_once(' ') {
            Some((cmd, str)) => return (cmd.to_string(), str.to_string()),
            None => return (s.to_string(), "".to_string()),
        }
    // }
    // res
}

pub fn format_prompt() {
    match env::current_dir() {
        Ok(path) => match path.file_name() {
            Some(name) => print!("{} $ ", name.to_str().unwrap()),
            None => print!("{} $ ", path.display()),
        },
        Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
    }
}
