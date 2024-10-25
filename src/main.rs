use std::env;
use std::fs;
use std::process::ExitCode;

enum Token {
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

    Eof,
}

use Token::*;

fn token_type_name(token: Token) -> &'static str {
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
        Eof => "EOF",
    }
}

fn tokenize(contents: &str) -> Vec<(Result<Token, char>, usize, usize)> {
    let mut r = Vec::<(Result<Token, char>, usize, usize)>::new();
    for (i, char) in contents.chars().enumerate() {
        if char == ' ' || char == '\n' || char == '\t' {
            continue;
        }
        let token = match char {
            '(' => Ok(LeftParen),
            ')' => Ok(RightParen),
            '{' => Ok(LeftBrace),
            '}' => Ok(RightBrace),
            ',' => Ok(Comma),
            '.' => Ok(Dot),
            '-' => Ok(Minus),
            '+' => Ok(Plus),
            ';' => Ok(Semicolon),
            '/' => Ok(Slash),
            '*' => Ok(Star),

            _ => Err(char),
        };
        r.push((token, i, i + 1));
    }
    r.push((Ok(Eof), contents.len(), contents.len()));
    r
}

fn cmd_tokenize(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap();
    let tokens = tokenize(&file_contents);
    let mut was_err = false;
    for (token, start, end) in tokens {
        match token {
            Ok(token) => {
                println!(
                    "{} {} null",
                    token_type_name(token),
                    &file_contents[start..end],
                )
            }
            Err(char) => {
                eprintln!("[line 1] Error: Unexpected character: {}", char);
                was_err = true;
            }
        }
    }
    if was_err {
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
