pub fn parse_arg(input: String) -> Result< Vec<String>, String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut active_quote: Option<char> = None;
    let mut escape = false;
let mut chars = input.trim_start().chars().peekable();
while let Some(c) = chars.next() {
    if escape {
        match c {
            'n' => if active_quote.is_some() {
                    current.push('\n');
                    chars.next();
                }else {
                    current.push('n');
                }   ,
            't' => if active_quote.is_some() {
                    current.push('\t');
                    chars.next();
                }else {
                    current.push('t');
                },
            'r' => if active_quote.is_some() {
                    current.push('\r');
                    chars.next();
                }else {
                    current.push('r');
                },
            '\\' => match chars.peek() {
                Some('n') => {
                    current.push('\n');
                    chars.next();
                }
               
                Some('t')  => {
                    current.push('\t');
                    chars.next();
                }
                Some('r')  => {
                    current.push('\r');
                    chars.next();
                }
                _ => current.push('\\'),
            },
            '\'' => current.push('\''),
            '"'  => current.push('"'),
            _    => current.push(c),
        }
        escape = false;
        continue;
    }

    match c {
        '\\' => {
            escape = true;
        }
        ' ' => {
            if active_quote.is_some() {
                current.push(c);
            } else {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
            }
        }
        '\'' | '"' => {
            if let Some(q) = active_quote {
                if q == c {
                    active_quote = None;
                    if !current.is_empty() {
                        result.push(current.clone());
                        current.clear();
                    }
                } else {
                    current.push(c);
                }
            } else {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
                active_quote = Some(c);
            }
        }
        '`' | ';' => {
            if active_quote.is_some() || escape {
                current.push(c);
            } else {
                return Err(format!(" [{}] not allowed", c));
            }
        }
        _ => {
            current.push(c);
        }
    }
}

if !current.is_empty() {
    result.push(current);
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
    !in_single_quote && !in_double_quote
}

