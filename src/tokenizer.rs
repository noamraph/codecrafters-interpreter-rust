#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TokenType {
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
    Identifier,
    StringLiteral,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

use TokenType::*;

fn get_keyword(name: &str) -> Option<TokenType> {
    match name {
        "and" => Some(And),
        "class" => Some(Class),
        "else" => Some(Else),
        "false" => Some(False),
        "fun" => Some(Fun),
        "for" => Some(For),
        "if" => Some(If),
        "nil" => Some(Nil),
        "or" => Some(Or),
        "print" => Some(Print),
        "return" => Some(Return),
        "super" => Some(Super),
        "this" => Some(This),
        "true" => Some(True),
        "var" => Some(Var),
        "while" => Some(While),
        _ => None,
    }
}

impl TokenType {
    pub fn name(&self) -> &'static str {
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

            Identifier => "IDENTIFIER",
            StringLiteral => "STRING",
            Number => "NUMBER",

            And => "AND",
            Class => "CLASS",
            Else => "ELSE",
            False => "FALSE",
            Fun => "FUN",
            For => "FOR",
            If => "IF",
            Nil => "NIL",
            Or => "OR",
            Print => "PRINT",
            Return => "RETURN",
            Super => "SUPER",
            This => "THIS",
            True => "TRUE",
            Var => "VAR",
            While => "WHILE",

            Eof => "EOF",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn literal_str(&self) -> String {
        match self.token_type {
            StringLiteral => self.lexeme[1..self.lexeme.len() - 1].to_string(),
            Number => {
                let x = self.lexeme.parse::<f64>().unwrap();
                format!("{:?}", x)
            }
            _ => "null".into(),
        }
    }
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
            StringLiteral
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
            Number
        }

        '_' | 'a'..='z' | 'A'..='Z' => {
            while scanner
                .peek()
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                scanner.advance();
            }
            if let Some(kw) = get_keyword(&scanner.substr(start, scanner.current)) {
                kw
            } else {
                Identifier
            }
        }

        _ => {
            scanner.error(&format!("Unexpected character: {}", c));
            return None;
        }
    };
    let lexeme = scanner.substr(start, scanner.current);
    Some(Token {
        token_type,
        lexeme,
        line: scanner.line,
    })
}

pub fn tokenize(contents: &str) -> (Vec<Token>, bool) {
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
        line: scanner.line,
    });
    (tokens, scanner.had_error)
}
