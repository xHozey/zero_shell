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
        let mut command = buffer.trim().to_string();
        let mut count = 0;
        let mut new_line_check = false;
        'newscan: loop {
            if quotes_closed(&command) && !command.ends_with('\\'){
                break 'newscan;
            }else if command.ends_with('\\') {
                add_buffer_format();
                let mut add_buffer = String::new();
                match stdin().read_line(&mut add_buffer) {
                    Ok(0) => break 'outer,
                    Ok(_) =>  command += add_buffer.trim_end_matches('\n'),
                    Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
                }
                break 'newscan;
            }else {
                add_buffer_format();
                let mut add_buffer = String::new();
                match stdin().read_line(&mut add_buffer) {
                    Ok(0) => break 'outer,
                    Ok(_) => {
                        new_line_check = true;
                        if count != 0 {
                            command += add_buffer.as_str();
                        }else {
                            command += &("\n".to_owned()+&add_buffer);
                        }
                    }
                    Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
                }
            }
            count+=1;
        }
        let mut args = match parse_arg(command.to_string()) {
            Ok(data) => data,
            Err(err ) => {
                eprint!("{}", err.to_ascii_lowercase());
                Vec::new()
            },
        };
        println!("{:?}",args);
        // let mut commands = parse_command(&args);
        //     match commands.0.as_str() {
        //         "pwd" => {
        //             pwd();
        //         }
        //         "echo" => {
        //             echo(commands.1);
        //         }
        //         "cat" => {
        //             cat(commands.1);
        //         }
        //         "cd" => {
        //             cd(commands.1);
        //         }
        //         "ls" => {
        //             ls(commands.1);
        //         }
        //         "cp" => {
        //             cp(commands.1);
        //         }
        //         "mkdir" => {
        //             mkdir(commands.1);
        //         }
        //         "mv" => {
        //             mv(commands.1);
        //         }
        //         "rm" => {
        //             rm(commands.1);
        //         }
        //         "exit" => {
        //             break 'outer;
        //         }
        //         _ => {
        //             eprintln!("command '{}' not found", commands.0)
        //         }
        //     }
    }
}
