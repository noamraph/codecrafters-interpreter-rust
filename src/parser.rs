use std::fmt;

use crate::tokenizer::{Token, TokenType};

pub enum Expr {
    Literal(usize, Literal),
    Variable(usize, Variable),
    Unary(usize, Unary),
    Binary(usize, Binary),
    Grouping(usize, Grouping),
}

pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

pub struct Variable(pub String);

pub struct Unary {
    pub op: UnaryOperator,
    pub expr: Box<Expr>,
}

pub enum UnaryOperator {
    Negative,
    Not,
}

pub struct Binary {
    pub left: Box<Expr>,
    pub op: BinaryOperator,
    pub right: Box<Expr>,
}

pub enum BinaryOperator {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Grouping(pub Box<Expr>);

pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var {
        name: String,
        initializer: Option<Expr>,
    },
}

pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(_, literal) => literal.fmt(f),
            Self::Variable(_, variable) => variable.fmt(f),
            Self::Unary(_, unary) => unary.fmt(f),
            Self::Binary(_, binary) => binary.fmt(f),
            Self::Grouping(_, grouping) => grouping.fmt(f),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{:?}", x),
            Self::String(s) => write!(f, "{}", s),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(variable {})", self.0)
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => write!(f, "-"),
            Self::Not => write!(f, "!"),
        }
    }
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {})", self.op, self.expr)
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::Less => write!(f, "<"),
            Self::LessEqual => write!(f, "<="),
            Self::Greater => write!(f, ">"),
            Self::GreaterEqual => write!(f, ">="),
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.op, self.left, self.right)
    }
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(group {})", self.0)
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr(e) => write!(f, "(expr {})", e),
            Stmt::Print(e) => write!(f, "(print {})", e),
            Stmt::Var { name, initializer } => {
                if let Some(e) = initializer {
                    write!(f, "(var {} {})", name, e)
                } else {
                    write!(f, "(var {})", name)
                }
            }
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "(")?;
        for stmt in &self.stmts {
            writeln!(f, "  {}", stmt)?;
        }
        writeln!(f, ")")
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub struct ParseError();

impl Parser {
    fn new(tokens: &[Token]) -> Self {
        Parser {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    fn previous(&self) -> &Token {
        assert!(self.current > 0);
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.peek().token_type == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> Result<&Token, ParseError> {
        if self.is_at_end() {
            return Err(self.error(self.peek(), "Not expecting end of file"));
        }
        self.current += 1;
        Ok(self.previous())
    }

    fn check_advance(&mut self, token_type: TokenType) -> Option<&Token> {
        if self.check(token_type) {
            self.current += 1;
            Some(self.previous())
        } else {
            None
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            Ok(self.advance()?)
        } else {
            Err(self.error(self.peek(), msg))
        }
    }

    fn error(&self, token: &Token, msg: &str) -> ParseError {
        let where_s: String = if token.token_type == TokenType::Eof {
            "end".into()
        } else {
            format!("'{}'", token.lexeme)
        };
        eprintln!("[line {}] Error at {}: {}", token.line, where_s, msg);
        ParseError()
    }

    fn line(&self) -> usize {
        self.peek().line
    }

    fn program(&mut self) -> Result<Program, ParseError> {
        let mut stmts = Vec::<Stmt>::new();
        while !self.is_at_end() {
            stmts.push(self.declaration()?);
        }
        Ok(Program { stmts })
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.check_advance(TokenType::Var).is_some() {
            let name = self
                .consume(TokenType::Identifier, "Expecting var name")?
                .lexeme
                .clone();
            let initializer = if self.check_advance(TokenType::Equal).is_some() {
                Some(self.expression()?)
            } else {
                None
            };
            self.consume(TokenType::Semicolon, "Expecting `;`")?;
            Ok(Stmt::Var { name, initializer })
        } else {
            self.stmt()
        }
    }

    fn stmt(&mut self) -> Result<Stmt, ParseError> {
        if self.check(TokenType::Print) {
            self.advance()?;
            let expr = self.expression()?;
            self.consume(TokenType::Semicolon, "Expecting `;`")?;
            Ok(Stmt::Print(expr))
        } else {
            let expr = self.expression()?;
            self.consume(TokenType::Semicolon, "Expecting `;`")?;
            Ok(Stmt::Expr(expr))
        }
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        loop {
            let op = match self.peek().token_type {
                TokenType::BangEqual => BinaryOperator::NotEqual,
                TokenType::EqualEqual => BinaryOperator::Equal,
                _ => break,
            };
            self.advance()?;
            let right = self.comparison()?;
            expr = Expr::Binary(
                self.line(),
                Binary {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
            );
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        loop {
            let op = match self.peek().token_type {
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                TokenType::Less => BinaryOperator::Less,
                TokenType::LessEqual => BinaryOperator::LessEqual,
                _ => break,
            };
            self.advance()?;
            let right = self.term()?;
            expr = Expr::Binary(
                self.line(),
                Binary {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
            )
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        loop {
            let op = match self.peek().token_type {
                TokenType::Minus => BinaryOperator::Sub,
                TokenType::Plus => BinaryOperator::Add,
                _ => break,
            };
            self.advance()?;
            let right = self.factor()?;
            expr = Expr::Binary(
                self.line(),
                Binary {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
            )
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        loop {
            let op = match self.peek().token_type {
                TokenType::Slash => BinaryOperator::Div,
                TokenType::Star => BinaryOperator::Mul,
                _ => break,
            };
            self.advance()?;
            let right = self.unary()?;
            expr = Expr::Binary(
                self.line(),
                Binary {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right),
                },
            )
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        let op = match self.peek().token_type {
            TokenType::Bang => Some(UnaryOperator::Not),
            TokenType::Minus => Some(UnaryOperator::Negative),
            _ => None,
        };
        if let Some(op) = op {
            self.advance()?;
            Ok(Expr::Unary(
                self.line(),
                Unary {
                    op,
                    expr: Box::new(self.unary()?),
                },
            ))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let token = self.advance()?.clone();
        let expr = match token.token_type {
            TokenType::Identifier => Expr::Variable(token.line, Variable(token.lexeme)),
            TokenType::Number => {
                let x = token.lexeme.parse::<f64>().unwrap();
                Expr::Literal(token.line, Literal::Number(x))
            }
            TokenType::StringLiteral => {
                let s = token.lexeme[1..token.lexeme.len() - 1].to_string();
                Expr::Literal(token.line, Literal::String(s))
            }
            TokenType::True => Expr::Literal(token.line, Literal::True),
            TokenType::False => Expr::Literal(token.line, Literal::False),
            TokenType::Nil => Expr::Literal(token.line, Literal::Nil),
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expecting `)`")?;
                Expr::Grouping(token.line, Grouping(Box::new(expr)))
            }
            _ => return Err(self.error(&token, "Unexpected token")),
        };
        Ok(expr)
    }
}

pub fn parse_expr(tokens: &[Token]) -> Result<Expr, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.expression()
}

pub fn parse_program(tokens: &[Token]) -> Result<Program, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.program()
}
