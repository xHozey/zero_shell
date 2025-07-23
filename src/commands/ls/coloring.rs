use colored::Colorize;

use crate::commands::ls::parser::Flags;
pub enum Color {
    Browen,
    Blue,
    Green,
    Skybleu,
    Red
}

pub fn coloring(file: &String, color: Color) -> String {
    let colored_file = match color {
        Color::Browen => file.truecolor(165, 42, 42).to_string(),
        Color::Blue => file.blue().to_string(),
        Color::Green => file.green().to_string(),
        Color::Skybleu => file.bright_blue().to_string(),
        Color::Red => file.red().to_string()
    };

    colored_file
}

pub fn color_devices(file_name: &String, color: Color) -> String {
    coloring(file_name, color)
}

pub fn color_dir(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f {
        colored_name.push('/');
    }
    colored_name
}

pub fn color_symlink(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f && !flags.l {
        colored_name.push('@');
    }
    colored_name
}

pub fn color_exec_file(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f {
        colored_name.push('*');
    }
    colored_name
}

pub fn color_pipe(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f {
        colored_name.push('|');
    }
    colored_name
}

pub fn color_socket(file_name: &String, color: Color, flags: &Flags) -> String {
    let mut colored_name = coloring(&file_name, color);
    if flags.f {
        colored_name.push('=');
    }
    colored_name
}
