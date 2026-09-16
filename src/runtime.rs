use std::collections::HashMap;

use crate::types::{BinaryOperator, Expr, RuntimeError, Statement, Type, Value};

pub fn run(statements: &[Statement]) -> Result<(), RuntimeError> {
    let mut vars: HashMap<String, Value> = HashMap::new();
    for stmt in statements {
        match stmt {
            Statement::VarDecl {
                name,
                value,
                var_type,
            } => {
                let v = eval(value, &vars)?;
                let ok = matches!(
                    (&v, var_type),
                    (Value::Int(_), Type::Int)
                        | (Value::Char(_), Type::Char)
                        | (Value::Bool(_), Type::Bool)
                        | (Value::Null, _)
                );
                if !ok {
                    return Err(RuntimeError::TypeMismatch {
                        expected: var_type.clone(),
                        got: v,
                    });
                }
                vars.insert(name.clone(), v);
            }
            Statement::FuncCall { name, arg } => match name.as_str() {
                "print" => println!("{}", eval(arg, &vars)?),
                _ => return Err(RuntimeError::UnknownFunction(name.clone())),
            },
            Statement::VarAssign { name, value } => {
                let v = eval(value, &vars)?;
                vars.insert(name.clone(), v);
            }
            Statement::Match { subject, arms } => todo!(),
        }
    }
    return Ok(());
}

fn eval(expr: &Expr, vars: &HashMap<String, Value>) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Int(n) => Ok(Value::Int(*n)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Char(c) => Ok(Value::Char(*c)),
        Expr::Null => Ok(Value::Null),
        Expr::Var(name) => {
            let get = vars.get(name);
            match get {
                Some(v) => Ok(*v),
                None => Err(RuntimeError::UndefinedVariable(name.clone())),
            }
        }
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let l = eval(left, vars)?;
            let r = eval(right, vars)?;
            eval_binary(&l, &r, &operator)
        }
    }
}

fn eval_binary(left: &Value, right: &Value, op: &BinaryOperator) -> Result<Value, RuntimeError> {
    match (left, op, right) {
        (Value::Int(a), BinaryOperator::Add, Value::Int(b)) => Ok(Value::Int(a + b)),
        (Value::Int(a), BinaryOperator::Add, Value::Null) => Ok(Value::Int(*a)),
        (Value::Null, BinaryOperator::Add, Value::Int(b)) => Ok(Value::Int(*b)),
        (Value::Int(a), BinaryOperator::Subtract, Value::Int(b)) => Ok(Value::Int(a - b)),
        (Value::Int(a), BinaryOperator::Multiply, Value::Int(b)) => Ok(Value::Int(a * b)),
        (Value::Int(a), BinaryOperator::Divide, Value::Int(b)) => {
            if *b == 0 {
                return Err(RuntimeError::DivisionByZero);
            }
            Ok(Value::Int(a / b))
        }
        (left, op, right) => Err(RuntimeError::InvalidOperation {
            left: left.clone(),
            right: right.clone(),
            op: op.clone(),
        }),
    }
}
