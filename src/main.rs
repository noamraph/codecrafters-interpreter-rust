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

    // Literals
    Identifier(String),
    StringLiteral(String),
    Number(String),

    Eof,
}

use TokenType::*;

impl TokenType {
    fn name(&self) -> &'static str {
        match self {
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

            Identifier(_) => "IDENTIFIER",
            StringLiteral(_) => "STRING",
            Number(_) => "NUMBER",

            Eof => "EOF",
        }
    }

    fn literal_str(&self) -> String {
        match self {
            StringLiteral(s) => s.clone(),
            _ => "null".into(),
        }
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

fn error(line: usize, had_error: &mut bool, msg: &str) {
    eprintln!("[line {}] Error: {}", line, msg);
    *had_error = true;
}

fn scan_token<I: Iterator<Item = char>>(
    iter: &mut Peekable<I>,
    line: &mut usize,
    had_error: &mut bool,
) -> Option<Token> {
    let c = iter.next()?;
    let mut lexeme = c.to_string();
    let token_type = match c {
        ' ' | '\t' => return None,
        '\n' => {
            *line += 1;
            return None;
        }
        '(' => LeftParen,
        ')' => RightParen,
        '{' => LeftBrace,
        '}' => RightBrace,
        ',' => Comma,
        '.' => Dot,
        '-' => Minus,
        '+' => Plus,
        ';' => Semicolon,
        '*' => Star,

        '!' | '=' | '>' | '<' => {
            if is_match(iter, '=') {
                lexeme.push('=');
                match c {
                    '!' => BangEqual,
                    '=' => EqualEqual,
                    '>' => GreaterEqual,
                    '<' => LessEqual,
                    _ => unreachable!(),
                }
            } else {
                match c {
                    '!' => Bang,
                    '=' => Equal,
                    '>' => Greater,
                    '<' => Less,
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
                return None;
            } else {
                Slash
            }
        }

        '"' => {
            let mut value = String::new();
            loop {
                let Some(c) = iter.next() else {
                    error(*line, had_error, "Unterminated string.");
                    return None;
                };
                lexeme.push(c);
                if c == '\n' {
                    *line += 1;
                }
                if c == '"' {
                    break;
                }
                value.push(c);
            }
            StringLiteral(value)
        }

        _ => {
            error(*line, had_error, &format!("Unexpected character: {}", c));
            return None;
        }
    };
    Some(Token { token_type, lexeme })
}

fn tokenize(contents: &str) -> (Vec<Token>, bool) {
    let mut line = 1;
    let mut had_error = false;
    let mut tokens = Vec::<Token>::new();
    let mut iter = contents.chars().peekable();
    while iter.peek().is_some() {
        if let Some(token) = scan_token(&mut iter, &mut line, &mut had_error) {
            tokens.push(token);
        }
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
            "{} {} {}",
            token.token_type.name(),
            token.lexeme,
            token.token_type.literal_str()
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
