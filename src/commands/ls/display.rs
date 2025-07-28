use crate::commands::ls::{coloring::*, handlers::DirInfo, parser::Flags};
use terminal_size::{terminal_size, Width};

pub fn display(files_info: Vec<Vec<String>>, dirs_info: Vec<DirInfo>, flags: Flags) {
    if !files_info.is_empty() {
        if flags.l {
            display_listed_format(&files_info, None, &flags);
        } else {
            for infos in files_info {
                let path_str = infos[0].clone();
                let name = colored_output(&path_str, None, &flags);
                print!("{}  ", name)
            }
            println!()
        }

        if !dirs_info.is_empty() {
            println!();
            for dir in &dirs_info {
                println!("{}:", dir.dir_name)
            }
        }
    }

    if !dirs_info.is_empty() {
        for (idx, dir) in dirs_info.iter().enumerate() {
            if dirs_info.len() >= 2 {
                println!("{}:", &dir.dir_name);
            }
            if flags.l {
                println!("total {}", dir.total_blocks);
                display_listed_format(&dir.entries, Some(&dir.dir_name), &flags)
            } else {
                display_normal_format(&dir.entries, Some(&dir.dir_name), &flags)
            }
            if idx != dirs_info.len() - 1 {
                println!();
            }
        }
    }
}

fn get_max_width(infos: &Vec<Vec<String>>) -> Vec<usize> {
    let mut cols_width = Vec::new();
    for j in 0..infos[0].len() {
        let mut max_width = 0;
        for i in 0..infos.len() {
            max_width = max_width.max(infos[i][j].len());
        }
        cols_width.push(max_width);
    }
    cols_width
}

fn display_listed_format(infos: &Vec<Vec<String>>, dir_name: Option<&str>, flags: &Flags) {
    if infos.is_empty() {
        return;
    }
    let cols_width = get_max_width(&infos);
    for infos in infos {
        for (idx, w) in cols_width.iter().enumerate() {
            if idx == 0 {
                //left aligned
                print!("{:<width$}", infos[idx], width = *w);
            } else if idx == cols_width.len() - 1 {
                let file_name = colored_output(&infos[idx], dir_name, flags);
                print!(" {}", file_name);
            } else if idx == 1 || idx == 4 {
                print!(" {:>width$}", infos[idx], width = *w);
            } else {
                //right aligned with space
                print!(" {:<width$}", infos[idx], width = *w);
            }
        }
        println!();
    }
}

fn display_normal_format(infos: &Vec<Vec<String>>, dir_name: Option<&str>, flags: &Flags) {
    if infos.is_empty() {
        return;
    }

    let terminal_width = get_terminal_width();
    let max_width = get_max_width(infos)[0];
    let cols_nbr = terminal_width / max_width;
    let rows_nbr = ((infos.len() as f64) / (cols_nbr as f64)).ceil() as usize;
    
    for row in 0..rows_nbr {
        for col in 0..cols_nbr {
            let idx = col * rows_nbr + row;

            if idx < infos.len() {
                let file_name = &infos[idx][0];
                let colored_name = colored_output(file_name, dir_name, flags);
                
                print!("{}", colored_name);

                if col < cols_nbr - 1 && idx + rows_nbr < infos.len() {
                    let padding = max_width - file_name.len() + 1;
                    print!("{}", " ".repeat(padding));
                }
            }
        }
        println!();
    }
}

fn get_terminal_width() -> usize {
    if let Some((Width(w), _)) = terminal_size() {
        w as usize
    } else {
        20
    }
}
