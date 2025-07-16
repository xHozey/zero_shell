use regex::Regex;
use std::{fs};
use std::env;

pub fn ls(s: String) {
    let res = ls_parse(s);
    println!("{:?}", res);
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