use std::fmt;

use crate::tokenizer::{Token, TokenType};

pub enum Expr {
    Literal(Literal),
    Unary(Unary),
    Binary(Binary),
    Grouping(Grouping),
}

pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

pub struct Unary {
    op: UnaryOperator,
    expr: Box<Expr>,
}

pub enum UnaryOperator {
    Negative,
    Not,
}

pub struct Binary {
    left: Box<Expr>,
    op: BinaryOperator,
    right: Box<Expr>,
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

pub struct Grouping(Box<Expr>);

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(literal) => literal.fmt(f),
            Self::Unary(unary) => unary.fmt(f),
            Self::Binary(binary) => binary.fmt(f),
            Self::Grouping(grouping) => grouping.fmt(f),
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

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

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

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        loop {
            let op = match self.peek().token_type {
                TokenType::BangEqual => BinaryOperator::NotEqual,
                TokenType::EqualEqual => BinaryOperator::Equal,
                _ => break,
            };
            self.advance();
            let right = self.comparison();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            });
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        loop {
            let op = match self.peek().token_type {
                TokenType::Greater => BinaryOperator::Greater,
                TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
                TokenType::Less => BinaryOperator::Less,
                TokenType::LessEqual => BinaryOperator::LessEqual,
                _ => break,
            };
            self.advance();
            let right = self.term();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            })
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        loop {
            let op = match self.peek().token_type {
                TokenType::Minus => BinaryOperator::Sub,
                TokenType::Plus => BinaryOperator::Add,
                _ => break,
            };
            self.advance();
            let right = self.factor();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            })
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        loop {
            let op = match self.peek().token_type {
                TokenType::Slash => BinaryOperator::Div,
                TokenType::Star => BinaryOperator::Mul,
                _ => break,
            };
            self.advance();
            let right = self.unary();
            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            })
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let op = match self.peek().token_type {
            TokenType::Bang => Some(UnaryOperator::Not),
            TokenType::Minus => Some(UnaryOperator::Negative),
            _ => None,
        };
        if let Some(op) = op {
            self.advance();
            Expr::Unary(Unary {
                op,
                expr: Box::new(self.unary()),
            })
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr {
        let token = self.advance();
        match token.token_type {
            TokenType::Number => {
                let x = token.lexeme.parse::<f64>().unwrap();
                Expr::Literal(Literal::Number(x))
            }
            TokenType::StringLiteral => {
                let s = token.lexeme[1..token.lexeme.len() - 1].to_string();
                Expr::Literal(Literal::String(s))
            }
            TokenType::True => Expr::Literal(Literal::True),
            TokenType::False => Expr::Literal(Literal::False),
            TokenType::Nil => Expr::Literal(Literal::Nil),
            TokenType::LeftParen => {
                let expr = self.expression();
                let token = self.advance();
                if token.token_type != TokenType::RightParen {
                    panic!("Expecting ')', got {:?}", token.token_type);
                }
                Expr::Grouping(Grouping(Box::new(expr)))
            }
            _ => panic!("Unexpected token {:?}", token.token_type),
        }
    }
}

pub fn parse(tokens: &[Token]) -> Expr {
    let mut parser = Parser::new(tokens);
    parser.expression()
}
