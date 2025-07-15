use regex::Regex;
use std::{fs};
use std::env;

pub fn ls(s: String) {
    let res = ls_parse(s);
    println!("{:?}", res);
    // let args: Vec<&str> = s.split_whitespace().collect();
    // if args.len() == 0 {
    //     ls_print(false);
    // } else {
    //     let re = Regex::new(r"^(-[aFl]{1,3}(\s+|$)){1,3}$").unwrap();
    //     if re.is_match(&s.trim()) {
    //         for v in s.trim().chars() {
    //             if v == 'a' {
    //                 ls_print(true);
    //             }
    //         }
    //     }
    // }
}


fn ls_parse(usage: String) -> Result<(Vec<char>, Vec<String>), String> {
    let args: Vec<&str> = usage.split_whitespace().collect();
    let our_commands = "laF".to_string();
    let mut user_commands = Vec::new();
    let mut user_file_or_dir = Vec::new();
    for command in args {
        let mut flag = false;
        let mut f_or_dir = String::new();
        let new_command = command.replace("\"", "");
        let new_command2 = new_command.replace("\'", "");
        for (i, v) in new_command2.chars().enumerate() {
            if i == 0 && v == '-' {
                    flag = true;
                    continue;
            }
            if flag {
                if our_commands.contains(v) {
                    if !user_commands.contains(&v) {
                        user_commands.push(v);
                    }
                } else {
                    return Err(format!("ls: invalid option -- '{}'", v));
                }
            } else {
                f_or_dir.push(v);
            }
        }
        if f_or_dir.len() != 0 {
            user_file_or_dir.push(f_or_dir);
        }
    }
    println!("{:?}", user_commands);
    Ok((user_commands, user_file_or_dir))

}

// fn ls_print(a:bool) {
//     match env::current_dir() {
//             Ok(path) => {

//                 let paths = fs::read_dir(&path).unwrap();
//                 let mut arr = Vec::new();
//                 for path_result in paths {
//                     // let meta = metadata(path_result.unwrap().path());
//                     //  let perm = meta.unwrap().permissions();
//                     // let perm_str : String= format!("{:?}", perm);
//                     // println!("{:?}", &perm_str[46..perm_str.len()-5]);
//                     let path = path_result.unwrap().file_name().into_string().unwrap();

//                     arr.push(path);
//                 }
//                 arr.sort();
//                 for file in arr {
//                     if file.chars().nth(0) == Some('.') && !a {
//                         continue;
//                     }
//                     print!("{}", file);
//                 }
//             },
//             Err(err) => eprintln!("ls: {}", err.to_string().to_ascii_lowercase()),
//         }
// }

