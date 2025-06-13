use std::io::*;
use zero_shell::*;
mod commands;
use commands::*;

fn main() {
    'outer: loop {
        print!("$ ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(0) => {
                println!("");
                continue;
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

                }
                "cp" => {
                    cp(str);
                }
                "mkdir" => {
                    mkdir(str);
                }
                "exit" => {
                    break 'outer;
                }
                _ => {
                    println!("command not found!")
                }

            }
        }
    }
}