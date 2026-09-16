use std::collections::HashMap;

use crate::types::{
    self, BinaryOperator, Expr,
    Pattern::{Literal, Wildcard},
    RuntimeError::{self, Exception},
    Statement, Type, Value,
};

pub fn run(statements: &[Statement]) -> Result<(), RuntimeError> {
    let mut vars: HashMap<String, Value> = HashMap::new();
    let mut global: Vec<String> = Vec::new();
    for stmt in statements {
        run_statement(stmt, &mut vars, &mut global)?;
    }
    return Ok(());
}

fn run_loop(
    statements: &[Statement],
    vars: &mut HashMap<String, Value>,
) -> Result<(), RuntimeError> {
    match run_block(statements, vars) {
        Ok(_) => run_loop(statements, vars),
        Err(e) => match e {
            RuntimeError::UnexpectedBreak => Ok(()),
            _ => return Err(e),
        },
    }
}

fn run_block(
    statements: &[Statement],
    vars: &mut HashMap<String, Value>,
) -> Result<(), RuntimeError> {
    let mut current_scope: Vec<String> = Vec::new();

    for stmt in statements {
        run_statement(stmt, vars, &mut current_scope)?;
    }

    for name in current_scope {
        vars.remove(&name).unwrap();
    }
    Ok(())
}

fn run_statement(
    stmt: &Statement,
    vars: &mut HashMap<String, Value>,
    current_scope: &mut Vec<String>,
) -> Result<(), RuntimeError> {
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
            current_scope.push(name.clone());
        }
        Statement::FuncCall { name, arg } => match name.as_str() {
            "print" => println!("{}", eval(arg, &vars)?),
            _ => return Err(RuntimeError::UnknownFunction(name.clone())),
        },
        Statement::VarAssign { name, value } => {
            let v = eval(value, &vars)?;
            vars.insert(name.clone(), v);
        }
        Statement::Match { subject, arms } => {
            let subj = eval(subject, vars)?;
            let mut arm_pattern_values: Vec<Value> = Vec::new();
            let mut has_wildcard_statement = false;
            for arm in arms {
                match &arm.pattern {
                    Literal(expr) => {
                        let value = eval(expr, vars)?;
                        if are_same_type(&subj, &value) & !arm_pattern_values.contains(&value) {
                            arm_pattern_values.push(value);
                        } else {
                            return Err(RuntimeError::Exception(
                                "Error while processing match statement".to_string(),
                            ));
                        }
                    }
                    Wildcard => {
                        if !has_wildcard_statement {
                            has_wildcard_statement = true;
                        } else {
                            return Err(Exception(
                                "More than one wildcard statement in match".to_string(),
                            ));
                        }
                    }
                }
            }
            for arm in arms {
                match &arm.pattern {
                    Literal(expr) => {
                        let value = eval(expr, vars)?;
                        if value == subj {
                            run_block(&arm.body, vars)?;
                            break;
                        }
                    }
                    Wildcard => {
                        run_block(&arm.body, vars)?;
                        break;
                    }
                }
            }
        }
        Statement::Loop { body } => run_loop(body, vars)?,
        Statement::Break => return Err(RuntimeError::UnexpectedBreak),
    }
    Ok(())
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

fn are_same_type(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Bool(_), Value::Bool(_)) => true,
        (Value::Int(_), Value::Int(_)) => true,
        (Value::Char(_), Value::Char(_)) => true,
        (_, Value::Null) => true,
        (Value::Null, _) => true,
        _ => false,
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
        //logical
        (Value::Int(a), BinaryOperator::Equal, Value::Int(b)) => Ok(Value::Bool(a == b)),
        (Value::Int(a), BinaryOperator::LessThan, Value::Int(b)) => Ok(Value::Bool(a < b)),
        (Value::Int(a), BinaryOperator::MoreThan, Value::Int(b)) => Ok(Value::Bool(a > b)),
        (Value::Int(a), BinaryOperator::NotEqual, Value::Int(b)) => Ok(Value::Bool(a != b)),
        (Value::Bool(a), BinaryOperator::NotEqual, Value::Bool(b)) => Ok(Value::Bool(a != b)),
        (Value::Bool(a), BinaryOperator::Equal, Value::Bool(b)) => Ok(Value::Bool(a == b)),
        (left, op, right) => Err(RuntimeError::InvalidOperation {
            left: left.clone(),
            right: right.clone(),
            op: op.clone(),
        }),
    }
}
