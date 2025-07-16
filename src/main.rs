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
        'newscan: loop {
            if quotes_closed(&commands.1) && !commands.1.ends_with('\\'){
                break 'newscan;
            }else if commands.1.ends_with('\\') {
                add_buffer_format();
            
                let mut add_buffer = String::new();
                match stdin().read_line(&mut add_buffer) {
                    Ok(0) => break 'outer,
                    Ok(_) => { commands.1 += add_buffer.as_str();}
                    Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
                }
               
                break 'newscan;
            }else {
                add_buffer_format();
                let mut add_buffer = String::new();
                match stdin().read_line(&mut add_buffer) {
                    Ok(0) => break 'outer,
                    Ok(_) => {commands.1 += add_buffer.as_str();}
                    Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
                }
                
            }
        }
        let args = match parse_arg(commands.1) {
            Ok(data) => data.join(""),
            Err(err ) => {
                    eprintln!("{}", err.to_ascii_lowercase());
                    String::new()
                },
        };
        // for cmd in  {
            match commands.0.as_str() {
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
                    eprintln!("command '{}' not found", commands.0)
                }
            }
        // }
    }
}
