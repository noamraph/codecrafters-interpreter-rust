use std::env;
use std::fs;

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
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            for char in file_contents.chars() {
                match char {
                    '(' => {
                        println!("LEFT_PAREN ( null");
                    }
                    ')' => {
                        println!("RIGHT_PAREN ) null");
                    }
                    ' ' | '\n' | '\t' => {}
                    _ => panic!("Unexpected char {:?}", char),
                }
            }
            println!("EOF  null");
        }
        _ => {
            panic!("Unknown command: {}", command);
        }
    }
}
