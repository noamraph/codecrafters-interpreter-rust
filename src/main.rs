use std::env;
use std::fs;
use std::process::ExitCode;

pub mod parser;
pub mod tokenizer;

use parser::{parse, ParseError};
use tokenizer::tokenize;

fn cmd_tokenize(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap();
    let (tokens, had_error) = tokenize(&file_contents);
    for token in tokens {
        println!(
            "{} {} {}",
            token.token_type.name(),
            token.lexeme,
            token.literal_str()
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
    let maybe_expr = parse(&tokens);
    match maybe_expr {
        Ok(expr) => {
            println!("{}", expr);
            ExitCode::SUCCESS
        }
        Err(ParseError()) => ExitCode::from(65),
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
        "parse" => cmd_parse(filename),
        _ => {
            panic!("Unknown command: {}", command);
        }
    }
}
