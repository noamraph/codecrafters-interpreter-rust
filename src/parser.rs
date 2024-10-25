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

pub struct Grouping {
    expr: Box<Expr>,
}

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

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for Grouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub fn parse(tokens: &[Token]) -> Expr {
    if tokens.len() != 2 {
        todo!()
    }
    assert!(tokens[1].token_type == TokenType::Eof);
    match &tokens[0].token_type {
        TokenType::True => Expr::Literal(Literal::True),
        TokenType::False => Expr::Literal(Literal::False),
        TokenType::Nil => Expr::Literal(Literal::Nil),
        TokenType::Number(x) => Expr::Literal(Literal::Number(*x)),
        TokenType::StringLiteral(s) => Expr::Literal(Literal::String(s.clone())),
        _ => todo!(),
    }
}
