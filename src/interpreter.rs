use std::{collections::HashMap, fmt};

use crate::parser::{
    BinaryOperator, Expr, Literal, LogicalOperator, Program, Stmt, UnaryOperator, Variable,
};

#[derive(PartialEq, Clone)]
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

pub struct Environment {
    /// We hold a stack of scopes. The most local is the last
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    fn get(&self, name: &String) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) {
                return Some(val.clone());
            }
        }
        None
    }

    fn set(&mut self, name: &String, val: &Value) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.clone(), val.clone());
                return true;
            }
        }
        false
    }

    fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.scopes.pop();
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            scopes: Vec::<_>::from([HashMap::<_, _>::new()]),
        }
    }
}

pub fn evaluate(expr: &Expr, ctx: &mut Environment) -> Result<Value, RuntimeError> {
    Ok(match expr {
        Expr::Literal(_, literal) => match literal {
            Literal::Number(x) => Value::Number(*x),
            Literal::String(s) => Value::String(s.clone()),
            Literal::True => Value::Bool(true),
            Literal::False => Value::Bool(false),
            Literal::Nil => Value::Nil,
        },
        Expr::Variable(line, Variable(name)) => match ctx.get(name) {
            Some(v) => v.clone(),
            None => {
                return Err(RuntimeError::new(
                    *line,
                    format!("Undefined variable '{}'.", name),
                ))
            }
        },
        Expr::Unary(line, unary) => {
            let val = evaluate(&unary.expr, ctx)?;
            match unary.op {
                UnaryOperator::Negative => Value::Number(-expect_number(&val, *line)?),
                UnaryOperator::Not => Value::Bool(!to_bool(&val)),
            }
        }
        Expr::Grouping(_, grouping) => evaluate(&grouping.0, ctx)?,
        Expr::Binary(line, binary) => {
            let left = evaluate(&binary.left, ctx)?;
            let right = evaluate(&binary.right, ctx)?;
            match binary.op {
                BinaryOperator::Add => match left {
                    Value::Number(left) => Value::Number(left + expect_number(&right, *line)?),
                    Value::String(left) => {
                        let Value::String(right) = right else {
                            return Err(RuntimeError::new(*line, "Expecting a string".into()));
                        };
                        Value::String(format!("{}{}", left, right))
                    }
                    _ => {
                        return Err(RuntimeError::new(
                            *line,
                            "Expecting a number or a string".into(),
                        ))
                    }
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
        Expr::Logical(_line, logical) => {
            let left = evaluate(&logical.left, ctx)?;
            let left_as_bool = to_bool(&left);
            let eval_right = match logical.op {
                LogicalOperator::And => left_as_bool,
                LogicalOperator::Or => !left_as_bool,
            };
            if eval_right {
                evaluate(&logical.right, ctx)?
            } else {
                left
            }
        }
        Expr::Assign(line, assign) => {
            let val = evaluate(&assign.rhs, ctx)?;
            let is_ok = ctx.set(&assign.name, &val);
            if !is_ok {
                return Err(RuntimeError::new(
                    *line,
                    format!("Variable '{}' not declared before assignment", assign.name),
                ));
            }
            val
        }
    })
}

pub fn interpret_stmt(stmt: &Stmt, ctx: &mut Environment) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::Print(e) => {
            let val = evaluate(e, ctx)?;
            println!("{}", val);
        }
        Stmt::Expr(e) => {
            // This is just for possible side effects
            evaluate(e, ctx)?;
        }
        Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        } => {
            let val = evaluate(condition, ctx)?;
            if to_bool(&val) {
                interpret_stmt(then_branch, ctx)?;
            } else if let Some(else_branch) = else_branch {
                interpret_stmt(else_branch, ctx)?;
            }
        }
        Stmt::Var { name, initializer } => {
            let val = if let Some(e) = initializer {
                evaluate(e, ctx)?
            } else {
                Value::Nil
            };
            let n_scopes = ctx.scopes.len();
            ctx.scopes[n_scopes - 1].insert(name.into(), val);
        }
        Stmt::Block(stmts) => {
            ctx.push();
            for stmt in stmts {
                interpret_stmt(stmt, ctx)?;
            }
            ctx.pop();
        }
    }
    Ok(())
}

pub fn interpret_program(program: &Program) -> Result<(), RuntimeError> {
    let mut ctx = Environment::default();
    for stmt in &program.stmts {
        interpret_stmt(stmt, &mut ctx)?;
    }
    Ok(())
}
