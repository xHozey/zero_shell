use std::{io::*, path::PathBuf};
use zero_shell::*;
mod commands;
use commands::*;
fn main() {
    let mut last_env = PathBuf::new();
    // save last path for cd
    'outer: loop {
        let path = format_prompt(&last_env);
        if path.is_some() {
            last_env = path.unwrap()
        }
        if let Err(err) = stdout().flush() {
            if err.kind() == ErrorKind::BrokenPipe {
                eprintln!("{}", err.to_string().to_ascii_lowercase());
                break 'outer;
            }
            eprintln!("{}", err.to_string().to_ascii_lowercase());
            continue;
        }
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(0) => break 'outer,
            Ok(_) => {}
            Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
        }
        let commands = parse_tokens(tokenizer(buffer.trim().to_string()));

        for (cmd, args) in commands {
            match cmd.as_str() {
                "pwd" => {
                    pwd(&last_env);
                }
                "echo" => {
                    echo(args);
                }
                "cat" => {
                    cat(args);
                }
                "cd" => {
                    cd(args);
                }
                "ls" => {
                    ls(args);
                }
                "cp" => {
                    cp(args);
                }
                "mkdir" => {
                    mkdir(args);
                }
                "mv" => {
                    mv(args);
                }
                "rm" => {
                    rm(args);
                }
                "exit" => {
                    break 'outer;
                }
                _ => {
                    eprintln!("command '{}' not found", cmd)
                }
            }
        }
    }
}
