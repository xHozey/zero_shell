use std::fs::read;
use std::io::stdin;
pub fn cat(s: String) {
    if s.is_empty() {
        loop {
            let mut buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(0) => {
                    break;
                },
                Ok(_) => {},
                Err(err) => eprintln!("cat: {}", err.to_string().to_ascii_lowercase())
            }
            println!("{}", buffer.trim())
        }
    } else {
        for s in s.split_whitespace() {
            match read(s) {
                Ok(content) => {
                    match String::from_utf8(content) {
                        Ok(v) => print!("{}", v),
                        Err(err) => eprint!("cat: {}", err.to_string().to_ascii_lowercase())
                    }
                },
                Err(err) => {
                    eprintln!("cat: {}", err.to_string().to_ascii_lowercase())
                }
            }
        }
    }
}