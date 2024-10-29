use std::fmt;

use crate::parser::{BinaryOperator, Expr, Literal, UnaryOperator};

#[derive(PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Number(x) => write!(f, "{}", x),
            Value::String(s) => write!(f, "{}", s),
        }
    }
}

pub struct RuntimeError {
    pub line: usize,
    pub msg: String,
}

impl RuntimeError {
    fn new(line: usize, msg: String) -> Self {
        Self { line, msg }
    }
}

fn to_bool(val: &Value) -> bool {
    match val {
        Value::Nil => false,
        Value::Bool(b) => *b,
        _ => true,
    }
}

fn expect_number(val: &Value, line: usize) -> Result<f64, RuntimeError> {
    match val {
        Value::Number(x) => Ok(*x),
        _ => Err(RuntimeError::new(line, "Expecting a number".into())),
    }
}

pub fn evaluate(expr: &Expr) -> Result<Value, RuntimeError> {
    Ok(match expr {
        Expr::Literal(_, literal) => match literal {
            Literal::Number(x) => Value::Number(*x),
            Literal::String(s) => Value::String(s.clone()),
            Literal::True => Value::Bool(true),
            Literal::False => Value::Bool(false),
            Literal::Nil => Value::Nil,
        },
        Expr::Unary(line, unary) => {
            let val = evaluate(&unary.expr)?;
            match unary.op {
                UnaryOperator::Negative => Value::Number(-expect_number(&val, *line)?),
                UnaryOperator::Not => Value::Bool(!to_bool(&val)),
            }
        }
        Expr::Grouping(_, grouping) => evaluate(&grouping.0)?,
        Expr::Binary(line, binary) => {
            let left = evaluate(&binary.left)?;
            let right = evaluate(&binary.right)?;
            match binary.op {
                BinaryOperator::Add => match left {
                    Value::Number(left) => Value::Number(left + expect_number(&right, *line)?),
                    Value::String(left) => {
                        let Value::String(right) = right else {
                            panic!("Expecting a string");
                        };
                        Value::String(format!("{}{}", left, right))
                    }
                    _ => panic!("Expecting a number or a string"),
                },
                BinaryOperator::Sub => {
                    Value::Number(expect_number(&left, *line)? - expect_number(&right, *line)?)
                }
                BinaryOperator::Mul => {
                    Value::Number(expect_number(&left, *line)? * expect_number(&right, *line)?)
                }
                BinaryOperator::Div => {
                    Value::Number(expect_number(&left, *line)? / expect_number(&right, *line)?)
                }
                BinaryOperator::Equal => Value::Bool(left == right),
                BinaryOperator::NotEqual => Value::Bool(left != right),
                BinaryOperator::Less => {
                    Value::Bool(expect_number(&left, *line)? < expect_number(&right, *line)?)
                }
                BinaryOperator::LessEqual => {
                    Value::Bool(expect_number(&left, *line)? <= expect_number(&right, *line)?)
                }
                BinaryOperator::Greater => {
                    Value::Bool(expect_number(&left, *line)? > expect_number(&right, *line)?)
                }
                BinaryOperator::GreaterEqual => {
                    Value::Bool(expect_number(&left, *line)? >= expect_number(&right, *line)?)
                }
            }
        }
    })
}
