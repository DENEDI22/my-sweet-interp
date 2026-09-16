use std::{collections::HashMap, fmt};

use crate::{BinaryOperator, Expr, Statement};

pub fn run(statements: &[Statement]) {
    let mut vars: HashMap<String, Value> = HashMap::new();
    for stmt in statements {
        match stmt {
            Statement::VarDecl {
                name,
                value,
                var_type,
            } => {
                let v = eval(value, &vars);
                vars.insert(name.clone(), v);
            }
            Statement::FuncCall { name, arg } => match name.as_str() {
                "print" => println!("{}", eval(arg, &vars)),
                _ => panic!("Unknown function: {name}"),
            },
            Statement::Empty => {}
            Statement::VarAssign { name, value } => {
                let v = eval(value, &vars);
                vars.insert(name.clone(), v);
            }
        }
    }
}

fn eval(expr: &Expr, vars: &HashMap<String, Value>) -> Value {
    match expr {
        Expr::Int(n) => Value::Int(*n),
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Char(c) => Value::Char(*c),
        Expr::Null => Value::Null,
        Expr::Var(name) => *vars
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable: {name:?}")),
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let l = eval(left, vars);
            let r = eval(right, vars);
            eval_binary(&l, &r, &operator)
        }
        Expr::Compare {
            left,
            operator,
            right,
        } => todo!(),
        Expr::Predicate() => todo!(),
    }
}

fn eval_binary(left: &Value, right: &Value, op: &BinaryOperator) -> Value {
    match (left, op, right) {
        (Value::Int(a), BinaryOperator::Add, Value::Int(b)) => Value::Int(a + b),
        (Value::Int(a), BinaryOperator::Add, Value::Null) => Value::Int(*a),
        (Value::Null, BinaryOperator::Add, Value::Int(b)) => Value::Int(*b),
        (Value::Int(a), BinaryOperator::Subtract, Value::Int(b)) => Value::Int(a - b),
        (Value::Int(a), BinaryOperator::Multiply, Value::Int(b)) => Value::Int(a * b),
        (Value::Int(a), BinaryOperator::Divide, Value::Int(b)) => Value::Int(a / b),
        (left, op, right) => panic!("Cannot apply {op:?} to {left:?} and {right:?}"),
    }
}
#[derive(Debug, Clone, Copy)]
enum Value {
    Int(i32),
    Char(char),
    Bool(bool),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Char(c) => write!(f, "{c}"),
            Value::Null => write!(f, "null"),
        }
    }
}
