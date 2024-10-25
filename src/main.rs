use std::env;
use std::fs;
use std::iter::Peekable;
use std::process::ExitCode;

enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Eof,
}

use TokenType::*;

fn token_type_name(token: TokenType) -> &'static str {
    match token {
        LeftParen => "LEFT_PAREN",
        RightParen => "RIGHT_PAREN",
        LeftBrace => "LEFT_BRACE",
        RightBrace => "RIGHT_BRACE",
        Comma => "COMMA",
        Dot => "DOT",
        Minus => "MINUS",
        Plus => "PLUS",
        Semicolon => "SEMICOLON",
        Slash => "SLASH",
        Star => "STAR",

        Bang => "BANG",
        BangEqual => "BANG_EQUAL",
        Equal => "EQUAL",
        EqualEqual => "EQUAL_EQUAL",
        Greater => "GREATER",
        GreaterEqual => "GREATER_EQUAL",
        Less => "LESS",
        LessEqual => "LESS_EQUAL",

        Eof => "EOF",
    }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
}

fn is_match<I: Iterator<Item = char>>(iter: &mut Peekable<I>, c: char) -> bool {
    if iter.peek() == Some(&c) {
        iter.next();
        true
    } else {
        false
    }
}

fn scan_token<I: Iterator<Item = char>>(
    iter: &mut Peekable<I>,
    line: &mut usize,
    had_error: &mut bool,
) -> Option<Token> {
    loop {
        let c = iter.next()?;
        let mut lexeme = c.to_string();
        let maybe_token = match c {
            ' ' | '\t' => None,
            '\n' => {
                *line += 1;
                None
            }
            '(' => Some(LeftParen),
            ')' => Some(RightParen),
            '{' => Some(LeftBrace),
            '}' => Some(RightBrace),
            ',' => Some(Comma),
            '.' => Some(Dot),
            '-' => Some(Minus),
            '+' => Some(Plus),
            ';' => Some(Semicolon),
            '*' => Some(Star),

            '!' | '=' | '>' | '<' => {
                if is_match(iter, '=') {
                    lexeme.push('=');
                    match c {
                        '!' => Some(BangEqual),
                        '=' => Some(EqualEqual),
                        '>' => Some(GreaterEqual),
                        '<' => Some(LessEqual),
                        _ => unreachable!(),
                    }
                } else {
                    match c {
                        '!' => Some(Bang),
                        '=' => Some(Equal),
                        '>' => Some(Greater),
                        '<' => Some(Less),
                        _ => unreachable!(),
                    }
                }
            }

            '/' => {
                if is_match(iter, '/') {
                    loop {
                        match iter.next() {
                            Some('\n') => {
                                *line += 1;
                                break;
                            }
                            None => break,
                            _ => (),
                        }
                    }
                    None
                } else {
                    Some(Slash)
                }
            }

            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line, c);
                *had_error = true;
                None
            }
        };
        if let Some(token_type) = maybe_token {
            return Some(Token { token_type, lexeme });
        }
    }
}

fn tokenize(contents: &str) -> (Vec<Token>, bool) {
    let mut line = 1;
    let mut had_error = false;
    let mut tokens = Vec::<Token>::new();
    let mut iter = contents.chars().peekable();
    while let Some(token) = scan_token(&mut iter, &mut line, &mut had_error) {
        tokens.push(token);
    }
    tokens.push(Token {
        token_type: Eof,
        lexeme: "".into(),
    });
    (tokens, had_error)
}

fn cmd_tokenize(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap();
    let (tokens, had_error) = tokenize(&file_contents);
    for token in tokens {
        println!(
            "{} {} null",
            token_type_name(token.token_type),
            token.lexeme
        );
    }
    if had_error {
        ExitCode::from(65)
    } else {
        ExitCode::SUCCESS
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return ExitCode::FAILURE;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => cmd_tokenize(filename),
        _ => {
            panic!("Unknown command: {}", command);
        }
    }
}
