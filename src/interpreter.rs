use std::fmt;

use crate::parser::{BinaryOperator, Expr, Literal, UnaryOperator};

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

fn to_bool(val: &Value) -> bool {
    match val {
        Value::Nil => false,
        Value::Bool(b) => *b,
        _ => true,
    }
}

fn expect_number(val: &Value) -> f64 {
    let Value::Number(x) = val else {
        panic!("Expecting a number");
    };
    *x
}

pub fn evaluate(expr: &Expr) -> Value {
    match expr {
        Expr::Literal(literal) => match literal {
            Literal::Number(x) => Value::Number(*x),
            Literal::String(s) => Value::String(s.clone()),
            Literal::True => Value::Bool(true),
            Literal::False => Value::Bool(false),
            Literal::Nil => Value::Nil,
        },
        Expr::Unary(unary) => {
            let val = evaluate(&unary.expr);
            match unary.op {
                UnaryOperator::Negative => Value::Number(-expect_number(&val)),
                UnaryOperator::Not => Value::Bool(!to_bool(&val)),
            }
        }
        Expr::Grouping(grouping) => evaluate(&grouping.0),
        Expr::Binary(binary) => {
            let left = evaluate(&binary.left);
            let right = evaluate(&binary.right);
            match binary.op {
                BinaryOperator::Add => match left {
                    Value::Number(left) => Value::Number(left + expect_number(&right)),
                    Value::String(left) => {
                        let Value::String(right) = right else {
                            panic!("Expecting a string");
                        };
                        Value::String(format!("{}{}", left, right))
                    }
                    _ => panic!("Expecting a number or a string"),
                },
                BinaryOperator::Sub => Value::Number(expect_number(&left) - expect_number(&right)),
                BinaryOperator::Mul => Value::Number(expect_number(&left) * expect_number(&right)),
                BinaryOperator::Div => Value::Number(expect_number(&left) / expect_number(&right)),
                _ => todo!(),
            }
        }
    }
}
