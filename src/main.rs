use std::io::*;
use zero_shell::*;
mod commands;
use commands::*;

fn main() {
    'outer: loop {
        format_prompt();
        stdout().flush().unwrap();
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(0) => {
                break 'outer
            },
            Ok(_) => {},
            Err(err) => println!("{}", err.to_string())
        }
        let commands = parse_command(buffer.trim());
        for (cmd, str) in commands {
            match cmd.as_str() {
                "pwd" => {
                    pwd();
                }
                "echo" => {
                    echo(str);
                }
                "cat" => {
                    cat(str);
                }
                "cd" => {
                    cd(str);
                }
                "ls" => {
                    ls(str);
                }
                "cp" => {
                    cp(str);
                }
                "mkdir" => {
                    mkdir(str);
                }
                "mv" => {
                    mv(str);
                }
                "rm" => {
                    rm(str);
                }
                "exit" => {
                    break 'outer;
                }
                _ => {
                    println!("Command '{}' not found", cmd)
                }

            }
        }
    }
}