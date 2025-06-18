use regex::Regex;
use std::os::unix::fs::MetadataExt;
use std::{fs};
use std::fs::metadata;
use std::os::unix::fs::PermissionsExt;
use std::env;

pub fn ls(s: String) {
    let args: Vec<&str> = s.split_whitespace().collect();
    if args.len() == 0 {
        ls_print(false);
    } else {
        let re = Regex::new(r"^(-[aFl]{1,3}(\s+|$)){1,3}$").unwrap();
        if re.is_match(&s.trim()) {
            for v in s.trim().chars() {
                if v == 'a' {
                    ls_print(true);
                }
            }
        }
    }
}

fn ls_print(a:bool) {
    match env::current_dir() {
            Ok(path) => {
                let paths = fs::read_dir(&path).unwrap();
                let mut arr = Vec::new();
                for path_result in paths {
                    // let meta = metadata(path_result.unwrap().path());
                    //  let perm = meta.unwrap().permissions();
                    // let perm_str : String= format!("{:?}", perm);
                    // println!("{:?}", &perm_str[46..perm_str.len()-5]);
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

