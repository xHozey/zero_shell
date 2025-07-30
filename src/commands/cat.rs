use std::fs::read;
use std::io::{self, stdin, Write};

pub fn cat(s: Vec<String>) {
    let s = s.join(" ");
    if s.is_empty() {
        read_input()
    } else {
        for filename in s.split_whitespace() {
            if filename == "-" {
                read_input();
                continue;
            }
            match read(filename) {
                Ok(content) => match String::from_utf8(content.clone()) {
                    Ok(text) => print!("{}", text),
                    Err(_) => {
                        if let Err(e) = io::stdout().write_all(&content) {
                            eprintln!("cat: failed to write bytes: {}", e);
                        }
                    }
                },
                Err(err) => {
                    eprintln!("cat: {}", err.to_string().to_ascii_lowercase());
                }
            }
            println!()
        }
    }
}

fn read_input() {
    loop {
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => print!("{}", buffer),
            Err(err) => eprintln!("cat: {}", err.to_string().to_ascii_lowercase()),
        }
    }
}
