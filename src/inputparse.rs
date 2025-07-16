use std::collections::HashMap;
pub fn parse_arg(str :String) -> Result<Vec<String>, String >{
    let mut result = vec![];
    let mut tmp = String::new();
    let mut quote = HashMap::new();
    let trim_str = str.trim_start();
    for i in trim_str.chars() {
       match i {
        '\'' => {
            if !tmp.is_empty(){
                result.push(tmp.to_string());
                tmp = String::new();
            }
            match quote.get(&'\'') {
                Some(true) => {
                    quote.insert('\'', false);
                    if !tmp.is_empty(){
                    result.push(tmp.to_string());
                    tmp = String::new();
                }
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
            if !tmp.is_empty(){
                result.push(tmp.to_string());
                tmp = String::new();
            }
            match quote.get(&'\"') {
                Some(true) => {
                    quote.insert('\"', false);
                    if !tmp.is_empty(){
                    result.push(tmp.to_string());
                    tmp = String::new();
                    }
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
                None =>{
                    if quote.get(&'\'') == Some(&true) {
                        tmp.push(i);
                    }else {
                        quote.insert('\\', true);
                    }
                }
                };
        },
        '`' => {
            if quote.get(&'\\') == Some(&true){
                tmp.push(i);
            }else {
                return Err(" [`] not allowd".to_string());
            }
        }
        _ => {
            tmp.push(i);
        }
       };
           
    }
    if !tmp.is_empty(){
        result.push(tmp.to_string());
    }
    Ok(result)
}



pub fn quotes_closed(s: &str) -> bool {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;

    for c in s.chars() {
        if escaped {
            escaped = false;
            continue;
        }

        match c {
            '\\' => {
                if !in_single_quote {
                    escaped = true;
                }
            }
            '"' => {
                if !in_single_quote {
                    in_double_quote = !in_double_quote;
                }
            }
            '\'' => {
                if !in_double_quote {
                    in_single_quote = !in_single_quote;
                }
            }
            _ => {}
        }
    }

    // done فقط إلا ما كاين لا quote لا dquote محلولين
    !in_single_quote && !in_double_quote
}

