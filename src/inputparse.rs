use std::collections::HashSet;

pub fn parse_arg(input: String) -> Result<String, String> {
    let mut result = String::new();
    let mut current = String::new();
    let mut active_quote: Option<char> = None;
    let mut escape = false;

    for c in input.trim_start().chars() {
        if escape {
            current.push(c);
            escape = false;
            continue;
        }

        match c {
            '\\' => {
                escape = true;
            }
            '\'' | '"' => {
                if let Some(q) = active_quote {
                    if q == c {
                        active_quote = None;
                        if !current.is_empty() {
                            result.push_str(&current.clone());
                            current.clear();
                        }
                    } else {
                        current.push(c);
                    }
                } else {
                    if !current.is_empty() {
                        result.push_str(&current.clone());
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
        result.push_str(&current);
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

