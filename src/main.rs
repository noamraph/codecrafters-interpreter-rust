use std::env;
use std::fs;
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
    Number(f64),

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
            Number(f) => format!("{:?}", f),
            _ => "null".into(),
        }
    }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
}

struct Scanner {
    source: Vec<char>,
    current: usize,
    line: usize,
    had_error: bool,
}

impl Scanner {
    fn new(source: &str) -> Self {
        Scanner {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            had_error: false,
        }
    }

    fn has_more(&self) -> bool {
        self.current < self.source.len()
    }

    /// This should only be called if you know there's a next char
    fn advance(&mut self) -> char {
        assert!(self.has_more());
        let c = self.source[self.current];
        self.current += 1;
        if c == '\n' {
            self.line += 1;
        }
        c
    }

    fn peek(&self) -> Option<char> {
        if self.has_more() {
            Some(self.source[self.current])
        } else {
            None
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 < self.source.len() {
            Some(self.source[self.current + 1])
        } else {
            None
        }
    }

    fn is_match(&mut self, c: char) -> bool {
        let is_match = self.peek() == Some(c);
        if is_match {
            self.advance();
        }
        is_match
    }

    fn error(&mut self, msg: &str) {
        eprintln!("[line {}] Error: {}", self.line, msg);
        self.had_error = true;
    }

    fn substr(&self, start: usize, end: usize) -> String {
        self.source[start..end].iter().collect()
    }
}

/// Consume at least one char. Return a Token if consumed a token.
fn scan_token(scanner: &mut Scanner) -> Option<Token> {
    let start = scanner.current;
    let c = scanner.advance();
    let token_type = match c {
        ' ' | '\t' | '\n' => return None,
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
            if scanner.is_match('=') {
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
            if scanner.is_match('/') {
                while scanner.has_more() {
                    let c = scanner.advance();
                    if c == '\n' {
                        break;
                    }
                }
                return None;
            } else {
                Slash
            }
        }

        '"' => {
            loop {
                if !scanner.has_more() {
                    scanner.error("Unterminated string.");
                    return None;
                }
                let c = scanner.advance();
                if c == '"' {
                    break;
                }
            }
            StringLiteral(scanner.substr(start + 1, scanner.current - 1))
        }

        '0'..='9' => {
            while scanner.peek().is_some_and(|c| c.is_ascii_digit()) {
                scanner.advance();
            }
            if scanner.peek() == Some('.')
                && scanner.peek_next().is_some_and(|c| c.is_ascii_digit())
            {
                scanner.advance();
            }
            while scanner.peek().is_some_and(|c| c.is_ascii_digit()) {
                scanner.advance();
            }
            Number(
                scanner
                    .substr(start, scanner.current)
                    .parse::<f64>()
                    .unwrap(),
            )
        }

        _ => {
            scanner.error(&format!("Unexpected character: {}", c));
            return None;
        }
    };
    let lexeme = scanner.substr(start, scanner.current);
    Some(Token { token_type, lexeme })
}

fn tokenize(contents: &str) -> (Vec<Token>, bool) {
    let mut tokens = Vec::<Token>::new();
    let mut scanner = Scanner::new(contents);
    while scanner.has_more() {
        if let Some(token) = scan_token(&mut scanner) {
            tokens.push(token);
        }
    }
    tokens.push(Token {
        token_type: Eof,
        lexeme: "".into(),
    });
    (tokens, scanner.had_error)
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
