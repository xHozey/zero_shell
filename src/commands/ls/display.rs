use crate::commands::ls::{handlers::DirInfo, parser::Flags};

pub fn display(files_info: Vec<Vec<String>>, dirs_info: Vec<DirInfo>, flags: Flags) {
    if !files_info.is_empty() {
        if flags.l {
            let cols_width = get_max_width(&files_info);
            for infos in files_info {
                for (idx, w) in cols_width.iter().enumerate() {
                    if idx == 0 {
                        //left aligned
                        print!("{:<width$}", infos[idx], width = *w);
                    } else if idx == cols_width.len() - 1 {
                        // no extra spacing
                        print!(" {}", infos[idx]);
                    } else {
                        //right aligned with space
                        print!(" {:>width$}", infos[idx], width = *w);
                    }
                }
                println!();
            }
        } else {
            
        }
    }
}

fn get_max_width(infos: &Vec<Vec<String>>) -> Vec<usize> {
    let mut cols_width = Vec::new();
    let mut j = 0;
    while j < infos[0].len() {
        let mut max_width = 0;
        for i in 0..infos.len() {
            if j < infos[i].len() {
                max_width = max_width.max(infos[i][j].len());
            }
        }
        j += 1;
        cols_width.push(max_width);
    }
    cols_width
}
