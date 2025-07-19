use std::io::*;
use zero_shell::*;
mod commands;
use commands::*;

fn main() {
    'outer: loop {
        format_prompt();
        if let Err(err) = stdout().flush() {
            eprintln!("{}", err.to_string().to_ascii_lowercase())
        }
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(0) => break 'outer,
            Ok(_) => {}
            Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
        }
        let commands = parse_tokens(tokenizer(buffer.trim().to_string()));

        for (cmd, args) in commands {
            let args = args.join(" ").trim().to_string();
            match cmd.as_str() {
                "pwd" => {
                    pwd();
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
