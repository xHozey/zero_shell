use std::io::*;
use zero_shell::*;
mod commands;
use commands::*;
mod inputparse;
use inputparse::*;

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
        let mut commands = parse_command(buffer.trim());
        // loop {
        //     if is_done(commands.1.to_string()){
        //         break;
        //     }
        //     add_buffer_format();
        //     let mut add_buffer = String::new();
        //     match stdin().read_line(&mut add_buffer) {
        //         Ok(0) => break 'outer,
        //         Ok(_) => {}
        //         Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
        //     }
        //     commands.1 += add_buffer.as_str();
        // }
        let args = parse_arg(commands.1);
        println!("{:?}",args);
        // for (cmd, args) in commands {
            // match commands.0.to_string() {
            //     "pwd" => {
            //         pwd();
            //     }
            //     "echo" => {
            //         echo(args);
            //     }
            //     "cat" => {
            //         // cat(args);
            //     }
            //     "cd" => {
            //         // cd(args);
            //     }
            //     "ls" => {
            //         // ls(args);
            //     }
            //     "cp" => {
            //         // cp(args);
            //     }
            //     "mkdir" => {
            //         // mkdir(args);
            //     }
            //     "mv" => {
            //         // mv(args);
            //     }
            //     "rm" => {
            //         // rm(args);
            //     }
            //     "exit" => {
            //         break 'outer;
            //     }
            //     _ => {
            //         eprintln!("command '{}' not found", commands.0)
            //     }
            // }
        // }
    }
}
