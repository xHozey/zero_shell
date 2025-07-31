use crate::commands::ls::{
    coloring::*,
    handlers::DirInfo,
    parser::Flags,
    utils::{get_max_width, get_terminal_width},
};

pub fn display(files_info: Vec<Vec<String>>, dirs_info: Vec<DirInfo>, flags: Flags) {
    if !files_info.is_empty() {
        if flags.l {
            display_listed_format(&files_info, None, &flags);
        } else {
            display_normal_format(&files_info, None, &flags);
        }

        if !dirs_info.is_empty() {
            println!()
        }
    }

    if !dirs_info.is_empty() {
        for (idx, dir) in dirs_info.iter().enumerate() {
            if dirs_info.len() >= 2 || !files_info.is_empty() {
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
    let cols_nbr = (terminal_width / max_width).max(1);
    let rows_nbr = ((infos.len() as f64) / (cols_nbr as f64)).ceil() as usize;

    let mut columns: Vec<Vec<&Vec<String>>> = vec![vec![]; cols_nbr];
    for (i, info) in infos.iter().enumerate() {
        let col = i / rows_nbr;
        if col < cols_nbr {
            columns[col].push(info);
        }
    }

    let col_widths: Vec<usize> = columns
        .iter()
        .map(|col| col.iter().map(|entry| entry[0].len()).max().unwrap_or(0) + 1)
        .collect();

    for row in 0..rows_nbr {
        for (col_idx, col) in columns.iter().enumerate() {
            if let Some(info) = col.get(row) {
                let file_name = &info[0];
                let colored_name = colored_output(file_name, dir_name, flags);
                let padding = (col_widths[col_idx] - file_name.len()) + 1;
                print!("{}{}", colored_name, " ".repeat(padding));
            }
        }
        println!();
    }
}
