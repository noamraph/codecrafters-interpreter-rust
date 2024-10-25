use std::env;
use std::fs;
use std::process::ExitCode;

mod tokenizer;

use tokenizer::{tokenize, TokenType};

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

fn cmd_parse(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap();
    let (tokens, had_error) = tokenize(&file_contents);
    if had_error {
        return ExitCode::from(65);
    }
    if tokens.len() == 2 {
        assert!(tokens[1].token_type == TokenType::Eof);
        let s: String = match &tokens[0].token_type {
            TokenType::True => "true".into(),
            TokenType::False => "false".into(),
            TokenType::Nil => "nil".into(),
            TokenType::Number(f) => format!("{:?}", f),
            TokenType::StringLiteral(s) => s.clone(),
            _ => todo!(),
        };
        println!("{}", s);
    } else {
        panic!("Unimplemented");
    }
    ExitCode::SUCCESS
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
        "parse" => cmd_parse(filename),
        _ => {
            panic!("Unknown command: {}", command);
        }
    }
}
