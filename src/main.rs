use std::io::*;
use zero_shell::*;
mod commands;
use commands::*;

fn main() {
    'outer: loop {
        print!("$ ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        let _ = stdin().read_line(&mut buffer);
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