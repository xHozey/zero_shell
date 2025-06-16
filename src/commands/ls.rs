use std::{fs};
use std::env;

pub fn ls(s: String) {


    let args: Vec<&str> = s.split_whitespace().collect();
    if args.len() == 0 {
        ls_print(false);
    } else {
        print!("{}", s);
    }
}

fn ls_print(a:bool) {
    match env::current_dir() {
            Ok(path) => {
                let paths = fs::read_dir(&path).unwrap();
                let mut arr = Vec::new();
                for path_result in paths {
                    let path = path_result.unwrap().file_name().into_string().unwrap();
                    arr.push(path);
                }
                arr.sort();
                for file in arr {
                    if file.chars().nth(0) == Some('.') && !a {
                        continue;
                    }
                    println!("{}", file);
                }
            },
            Err(err) => eprintln!("pwd: {}", err.to_string().to_ascii_lowercase()),
        }
}

