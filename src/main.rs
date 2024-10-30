use std::env;
use std::fs;
use std::process::ExitCode;

pub mod interpreter;
pub mod parser;
pub mod tokenizer;

use interpreter::evaluate;
use parser::{parse_expr, parse_program, Stmt};
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
    let Ok(expr) = parse_expr(&tokens) else {
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
    let Ok(expr) = parse_expr(&tokens) else {
        return ExitCode::from(65);
    };
    let maybe_val = evaluate(&expr);
    match maybe_val {
        Ok(val) => {
            println!("{}", val);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}\n[line {}]", err.msg, err.line);
            ExitCode::from(70)
        }
    }
}

fn cmd_run(filename: &str) -> ExitCode {
    let file_contents = fs::read_to_string(filename).unwrap();
    let (tokens, had_error) = tokenize(&file_contents);
    if had_error {
        return ExitCode::from(65);
    }
    let Ok(program) = parse_program(&tokens) else {
        return ExitCode::from(65);
    };
    for stmt in program.stmts {
        match stmt {
            Stmt::Print(e) => {
                let maybe_val = evaluate(&e);
                match maybe_val {
                    Ok(val) => {
                        println!("{}", val);
                    }
                    Err(err) => {
                        eprintln!("{}\n[line {}]", err.msg, err.line);
                        return ExitCode::from(70);
                    }
                }
            }
            Stmt::Expr(e) => {
                let maybe_val = evaluate(&e);
                if let Err(err) = maybe_val {
                    eprintln!("{}\n[line {}]", err.msg, err.line);
                    return ExitCode::from(70);
                }
            }
        }
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
        "evaluate" => cmd_evaluate(filename),
        "run" => cmd_run(filename),
        _ => {
            panic!("Unknown command: {}", command);
        }
    }
}
