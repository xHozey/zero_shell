use std::{env, io::*, path::PathBuf};

#[derive(PartialEq)]
// enum State {
//     Normal,
//     SingleQuote,
//     DoubleQuote,
//     Escape,
// }
#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    fn new(kind: TokenKind, value: String) -> Self {
        Self { kind, value }
    }
}

#[derive(Debug, PartialEq)]
enum TokenKind {
    Word,
    Operator,
}

pub fn tokenizer(mut input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut token_buffer = String::new();
    let mut active_quote: Option<char> = None;
    let mut escape = false;

    loop {
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            if escape {
                match c {
                    'n' => {
                        if active_quote.is_some() {
                            token_buffer.push('\n');
                        } else {
                            token_buffer.push('n');
                        }
                    }
                    't' => {
                        if active_quote.is_some() {
                            token_buffer.push('\t');
                        } else {
                            token_buffer.push('t');
                        }
                    }
                    'r' => {
                        if active_quote.is_some() {
                            token_buffer.push('\r');
                        } else {
                            token_buffer.push('r');
                        }
                    }
                    '\\' => match chars.peek() {
                        Some('n') => {
                            token_buffer.push('\n');
                            chars.next();
                        }

                        Some('t') => {
                            token_buffer.push('\t');
                            chars.next();
                        }
                        Some('r') => {
                            token_buffer.push('\r');
                            chars.next();
                        }
                        _ => {
                            token_buffer.push('\\');
                        }
                    },
                    '\'' => token_buffer.push('\''),
                    '"' => token_buffer.push('"'),
                    _ => {
                        token_buffer.push('\\');
                        token_buffer.push(c);
                    }
                }
                escape = false;
                continue;
            }

            match c {
                '\\' => {
                    escape = true;
                }
                c if c.is_whitespace() => {
                    if active_quote.is_some() {
                        token_buffer.push(c);
                    } else if !token_buffer.is_empty() {
                        tokens.push(Token::new(TokenKind::Word, token_buffer.clone()));
                        token_buffer.clear();
                    }
                }
                '\'' | '"' => {
                    if let Some(q) = active_quote {
                        if q == c {
                            active_quote = None;
                        } else {
                            token_buffer.push(c);
                        }
                    } else {
                        active_quote = Some(c);
                    }
                }
                ';' => {
                    if active_quote.is_some() {
                        token_buffer.push(c);
                    } else {
                        if !token_buffer.is_empty() {
                            tokens.push(Token::new(TokenKind::Word, token_buffer.clone()));
                            token_buffer.clear();
                        }
                        tokens.push(Token::new(TokenKind::Operator, ";".to_string()));
                    }
                }
                _ => {
                    token_buffer.push(c);
                }
            }
        }

        if active_quote.is_some() {
            if Some('"') == active_quote {
                print!("dquote> ");
            } else {
                print!("quote> ");
            }
            if let Err(err) = stdout().flush() {
                eprintln!("{}", err.to_string().to_ascii_lowercase());
                continue;
            }

            let mut more_input = String::new();
            if let Err(err) = stdin().read_line(&mut more_input) {
                eprintln!("{}", err.to_string().to_ascii_lowercase())
            }

            input = more_input;
            token_buffer.push('\n');
        } else {
            break;
        }
    }

    if !token_buffer.is_empty() {
        tokens.push(Token::new(TokenKind::Word, token_buffer));
    }

    tokens
}

pub fn parse_tokens(args: Vec<Token>) -> Vec<(String, Vec<String>)> {
    let mut res = Vec::new();
    let mut current_cmd = Vec::new();

    for token in args {
        match token.kind {
            TokenKind::Word => {
                if token.value == "~" {
                    current_cmd.push(env::var("HOME").unwrap_or("~".to_string()));
                } else {
                    current_cmd.push(token.value);
                }
            }
            TokenKind::Operator => {
                if token.value == ";" {
                    if !current_cmd.is_empty() {
                        let cmd = current_cmd[0].clone();
                        let args = current_cmd[1..].to_vec();
                        res.push((cmd, args));
                        current_cmd.clear();
                    }
                }
            }
        }
    }

    if !current_cmd.is_empty() {
        let cmd = current_cmd[0].clone();
        let args = current_cmd[1..].to_vec();
        res.push((cmd, args));
    }

    res
}

pub fn format_prompt(last_path: &PathBuf) -> Option<PathBuf> {
    match env::current_dir() {
        Ok(path) => {
            if path.file_name().is_some() {
                let name = path.file_name().unwrap().to_str().unwrap_or("");
                print!("{} $ ", name);
                return Some(path);
            }

            None
        }
        Err(_) => {
            let name = last_path.file_name().unwrap().to_str().unwrap_or("");
            print!("{} $ ", name);
            None
        }
    }
}
