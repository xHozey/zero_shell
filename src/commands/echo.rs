use std::collections::HashMap;
pub fn echo(s: String) {

    println!("{:?}",parse_arg(s));








    // let mut result = String::new();
    // // let tmp = s.replace("\\n", "\n").replace("\\t", "\t").replace("\\r", "\r");
    // let chars: Vec<char> = s.chars().collect();
    // let mut i = 0;
    // while i < chars.len() {
    //     if chars[i] == '\\' && i + 1 < chars.len() && chars[i+1] == '\\' {
    //         if i + 2 < chars.len() {
    //             match chars[i + 2] {
    //                 'n' => {
    //                     result.push('\n');
    //                     i += 2; 
    //                 }
    //                 't' => {
    //                     result.push('\t');
    //                     i += 2; 
    //                 }
    //                 'r' => {
    //                     result.push('\r');
    //                     i += 2;
    //                 }
    //                 '"' => {
    //                     result.push('"');
    //                     i += 2;
    //                 }
    //                 '\'' => {
    //                     result.push('\'');
    //                     i += 2;
    //                 }
    //                 _ => {
    //                     result.push('\\');
    //                     result.push(chars[i + 1]);
    //                     i += 1;
    //                 }
    //             }
    //         } else {
    //             result.push('\\');
    //         }
    //     } else {
    //         result.push(chars[i]);
    //     }

    //     i += 1;
    // }
    // let yu = result.trim_matches('\"').trim_matches('\'');
    // println!("{}", yu.replace("\\", ""));
}







pub fn parse_arg(str :String) -> Vec<(String,bool)> {
    let mut result = vec![];
    let mut tmp = String::new();
    let mut quote = HashMap::new();
    for i in str.chars() {
       match i {
        '\'' => {
            match quote.get(&'\'') {
                Some(true) => {
                    quote.insert('\'', false);
                    result.push((tmp.to_string(),true));
                    tmp = String::new();
                },
                Some(false) => { 
                    if quote.get(&'\"') == Some(&true){
                        tmp.push(i);
                    }else {
                        quote.insert(i, true);
                    }
                },
                None =>{
                    if quote.get(&'\"') == Some(&true) {
                        tmp.push(i);
                    }else {
                        quote.insert('\'', true);
                    }
                }
                };
        },
        '\"' => {
            match quote.get(&'\"') {
                Some(true) => {
                    quote.insert('\"', false);
                    result.push((tmp.to_string(),true));
                    tmp = String::new();
                },
                Some(false) => { 
                    if quote.get(&'\'') == Some(&true){
                        tmp.push(i);
                    }else {
                        quote.insert(i, true);
                    }
                },
                None =>{
                    if quote.get(&'\'') == Some(&true) {
                    tmp.push(i);
                }else {
                    quote.insert('\"', true);
                }}
                };
        },
        '\\' => {
            match quote.get(&'\\') {
                Some(true) => {
                    tmp.push(i);
                    if quote.get(&'\"') == Some(&false) && quote.get(&'\'') == Some(&false) {
                        quote.insert('\\', false);
                    }
                },
                Some(false) => { 
                    if quote.get(&'\"') == Some(&true) || quote.get(&'\'') == Some(&true) {
                        tmp.push(i);
                    }else {
                        quote.insert(i, false);
                    }
                },
                None =>{quote.insert('\\', true);}
                };
        },
        _ => {
            tmp.push(i);
        }
       };
           
    }
    if !tmp.is_empty(){
        result.push((tmp.to_string(),false));
    }
    result
}