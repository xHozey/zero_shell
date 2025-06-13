use std::fs::read;
pub fn cat(file: String) {
    match read(file) {
        Ok(content) => {
            match String::from_utf8(content) {
                Ok(v) => println!("{}", v),
                Err(_) => println!("couldn't read file content")
            }
        },
        Err(err) => {
            println!("{:?}", err.to_string())
        }
    }
}