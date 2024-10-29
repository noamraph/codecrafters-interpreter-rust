use std::fmt;

use crate::parser::{Expr, Literal, UnaryOperator};

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
                UnaryOperator::Negative => {
                    let Value::Number(x) = val else {
                        panic!("Expecting a number");
                    };
                    Value::Number(-x)
                }
                UnaryOperator::Not => Value::Bool(!to_bool(&val)),
            }
        }
        Expr::Grouping(grouping) => evaluate(&grouping.0),
        _ => todo!(),
    }
}
