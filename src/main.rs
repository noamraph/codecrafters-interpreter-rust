use std::env;
use std::fs;

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

fn tokenize(contents: &str) -> Vec<(Token, usize, usize)> {
    let mut r = Vec::<(Token, usize, usize)>::new();
    for (i, char) in contents.chars().enumerate() {
        if char == ' ' || char == '\n' || char == '\t' {
            continue;
        }
        let token = match char {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            ',' => Comma,
            '.' => Dot,
            '-' => Minus,
            '+' => Plus,
            ';' => Semicolon,
            '/' => Slash,
            '*' => Star,

            _ => panic!("Unexpected char {:?}", char),
        };
        r.push((token, i, i + 1));
    }
    r.push((Eof, contents.len(), contents.len()));
    r
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap();
            let tokens = tokenize(&file_contents);
            for (token, start, end) in tokens {
                println!(
                    "{} {} null",
                    token_type_name(token),
                    &file_contents[start..end],
                )
            }
        }
        _ => {
            panic!("Unknown command: {}", command);
        }
    }
}
