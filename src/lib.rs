use std::{env, io::*};

#[derive(PartialEq)]
enum State {
    Normal,
    SingleQuote,
    DoubleQuote,
    Escape,
}
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
    let mut state = State::Normal;

    loop {
        let mut chars = input.chars().peekable();
        let mut prev_char = None;

        while let Some(c) = chars.next() {
            match state {
                State::Normal => match c {
                    '"' => state = State::DoubleQuote,
                    '\'' => state = State::SingleQuote,
                    '\\' => state = State::Escape,
                    ';' => {
                        if !token_buffer.is_empty() {
                            tokens.push(Token::new(TokenKind::Word, token_buffer.clone()));
                            token_buffer.clear();
                        }
                        tokens.push(Token::new(TokenKind::Operator, ";".to_string()));
                    }
                    c if c.is_whitespace() => {
                        if !token_buffer.is_empty() {
                            tokens.push(Token::new(TokenKind::Word, token_buffer.clone()));
                            token_buffer.clear();
                        }
                    }
                    _ => token_buffer.push(c),
                },
                State::DoubleQuote => {
                    if c == '"' && prev_char != Some('\\') {
                        state = State::Normal;
                    } else if prev_char == Some('\\') {
                        match c {
                            'n' => token_buffer.push('\n'),
                            't' => token_buffer.push('\t'),
                            'r' => token_buffer.push('\r'),
                            '\\' => token_buffer.push('\\'),
                            '"' => token_buffer.push('"'),
                            '\'' => token_buffer.push('\''),
                            _ => {
                                token_buffer.push('\\');
                                token_buffer.push(c);
                            }
                        }
                        prev_char = None;
                    } else if c == '\\' {
                        prev_char = Some('\\');
                    } else {
                        token_buffer.push(c);
                        prev_char = None;
                    }
                }
                State::SingleQuote => {
                    if c == '\'' {
                        state = State::Normal;
                    } else {
                        token_buffer.push(c);
                    }
                }
                State::Escape => {
                    token_buffer.push(c);
                    state = State::Normal;
                }
            }
        }

        if state == State::DoubleQuote || state == State::SingleQuote {
            if state == State::DoubleQuote {
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
                current_cmd.push(token.value);
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

pub fn format_prompt() {
    match env::current_dir() {
        Ok(path) => match path.file_name() {
            Some(name) => print!("{} $ ", name.to_str().unwrap()),
            None => print!("{} $ ", path.display()),
        },
        Err(err) => eprintln!("{}", err.to_string().to_ascii_lowercase()),
    }
}
