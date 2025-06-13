use std::fs::read;
use std::io::stdin;
pub fn cat(file: String) {
    if file.is_empty() {
        loop {
            let mut buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(0) => {
                    break;
                },
                Ok(_) => {},
                Err(err) => println!("{}", err.to_string())
            }
            println!("{}", buffer.trim())
        }
    } else {
        for s in file.split_whitespace() {
            match read(s) {
                Ok(content) => {
                    match String::from_utf8(content) {
                        Ok(v) => print!("{}", v),
                        Err(err) => print!("{}", err.to_string())
                    }
                },
                Err(err) => {
                    println!("{:?}", err.to_string())
                }
            }
        }
    }
}