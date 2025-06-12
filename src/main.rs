use std::io::*;

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer);
        println!("$ {}", buffer.trim());
    }
}