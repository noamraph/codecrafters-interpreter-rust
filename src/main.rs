use std::env;
use std::fs;
use std::process::ExitCode;

pub mod interpreter;
pub mod parser;
pub mod tokenizer;

use interpreter::evaluate;
use parser::parse;
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
    let Ok(expr) = parse(&tokens) else {
        return ExitCode::from(65);
    };
    println!("{}", expr);
    ExitCode::SUCCESS
}

fn cmd_evaluate(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap();
    let (tokens, had_error) = tokenize(&file_contents);
    if had_error {
        return ExitCode::from(65);
    }
    let Ok(expr) = parse(&tokens) else {
        return ExitCode::from(65);
    };
    let val = evaluate(&expr);
    println!("{}", val);
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
        "evaluate" => cmd_evaluate(filename),
        _ => {
            panic!("Unknown command: {}", command);
        }
    }
}
