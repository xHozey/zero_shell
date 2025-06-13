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
        match read(file) {
            Ok(content) => {
                match String::from_utf8(content) {
                    Ok(v) => println!("{}", v),
                    Err(err) => println!("{}", err.to_string())
                }
            },
            Err(err) => {
                println!("{:?}", err.to_string())
            }
        }
    }
}