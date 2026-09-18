use std::rc::Rc;

use crate::{
    resolver::Resolver,
    types::{
        BinaryOperator, Expr, FlowState,
        Pattern::{Literal, Wildcard},
        RuntimeError::{self, Exception},
        Statement, Type, Value,
    },
};

pub fn run(statements: &[Statement], resolver: &Resolver) -> Result<FlowState, RuntimeError> {
    let mut vars: Vec<Value> = Vec::new();
    resolver.names.iter().for_each(|_| {
        vars.push(Value::Null);
    });
    for stmt in statements {
        match run_statement(stmt, &mut vars, resolver)? {
            FlowState::Break => {
                return Err(RuntimeError::Exception(
                    "Break found out of a loop scope".to_string(),
                ));
            }
            _ => {}
        };
    }
    return Ok(FlowState::Finished);
}

fn run_loop(
    statements: &[Statement],
    vars: &mut Vec<Value>,
    resolver: &Resolver,
) -> Result<FlowState, RuntimeError> {
    loop {
        match run_block(statements, vars, resolver)? {
            FlowState::Break => break,
            _ => {}
        }
    }
    Ok(FlowState::None)
}

fn run_block(
    statements: &[Statement],
    vars: &mut Vec<Value>,
    resolver: &Resolver,
) -> Result<FlowState, RuntimeError> {
    for stmt in statements {
        match run_statement(stmt, vars, resolver)? {
            FlowState::None => {}
            flow => return Ok(flow),
        }
    }
    Ok(FlowState::None)
}

fn run_statement(
    stmt: &Statement,
    vars: &mut Vec<Value>,
    resolver: &Resolver,
) -> Result<FlowState, RuntimeError> {
    match stmt {
        Statement::VarIdDecl {
            id,
            value,
            var_type,
        } => {
            let v = eval(value, &vars, resolver)?;
            let ok = matches!(
                (&v, var_type),
                (Value::Int(_), Type::Int)
                    | (Value::Char(_), Type::Char)
                    | (Value::String(_), Type::String)
                    | (Value::Bool(_), Type::Bool)
                    | (Value::Null, _)
            );
            if !ok {
                return Err(RuntimeError::TypeMismatch {
                    expected: var_type.clone(),
                    got: v,
                });
            }
            vars[*id] = v;
        }
        Statement::FuncCall { name, arg } => match name.as_str() {
            "print" => println!("{}", eval(arg, &vars, resolver)?),
            _ => return Err(RuntimeError::UnknownFunction(name.clone())),
        },
        Statement::VarAssignById { id, value } => {
            let v = eval(value, &vars, resolver)?;
            vars[*id] = v;
        }
        Statement::Match { subject, arms } => {
            let subj = eval(subject, vars, resolver)?;
            let mut arm_pattern_values: Vec<Value> = Vec::new();
            let mut has_wildcard_statement = false;
            for arm in arms {
                match &arm.pattern {
                    Literal(expr) => {
                        let value = eval(expr, vars, resolver)?;
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
                        let value = eval(expr, vars, resolver)?;
                        if value == subj {
                            return run_block(&arm.body, vars, resolver);
                        }
                    }
                    Wildcard => {
                        return run_block(&arm.body, vars, resolver);
                    }
                }
            }
        }
        Statement::Loop { body } => {
            run_loop(body, vars, resolver)?;
        }
        Statement::Break => return Ok(FlowState::Break),
        Statement::VarDecl {
            name: _,
            value: _,
            var_type: _,
        } => unreachable!(),
        Statement::VarAssign { name: _, value: _ } => unreachable!(),
    }
    Ok(FlowState::None)
}
fn eval(expr: &Expr, vars: &Vec<Value>, resolver: &Resolver) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Int(n) => Ok(Value::Int(*n)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Char(c) => Ok(Value::Char(*c)),
        Expr::Null => Ok(Value::Null),
        Expr::Var(_) => unreachable!(),
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let l = eval(left, vars, resolver)?;
            let r = eval(right, vars, resolver)?;
            eval_binary(&l, &r, &operator)
        }
        Expr::VarId(i) => {
            let get = vars.get(*i);
            match get {
                Some(v) => Ok(v.clone()),
                None => Err(RuntimeError::UndefinedVariable(resolver.names[*i].clone())),
            }
        }
        Expr::String(s) => Ok(Value::String(Rc::from(s.clone()))),
    }
}

fn are_same_type(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Bool(_), Value::Bool(_)) => true,
        (Value::Int(_), Value::Int(_)) => true,
        (Value::Char(_), Value::Char(_)) => true,
        (Value::String(_), Value::String(_)) => true,
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
        (Value::String(a), BinaryOperator::Equal, Value::String(b)) => Ok(Value::Bool(a == b)),
        (Value::String(a), BinaryOperator::NotEqual, Value::String(b)) => Ok(Value::Bool(a != b)),
        //concat
        (Value::String(a), BinaryOperator::Add, Value::String(b)) => {
            let mut s = String::with_capacity(a.len() + b.len());
            s.push_str(a);
            s.push_str(b);
            Ok(Value::String(Rc::from(s)))
        }
        (Value::String(a), BinaryOperator::Add, Value::Null) => Ok(Value::String(a.clone())),
        (Value::Null, BinaryOperator::Add, Value::String(a)) => Ok(Value::String(a.clone())),
        (left, op, right) => Err(RuntimeError::InvalidOperation {
            left: left.clone(),
            right: right.clone(),
            op: op.clone(),
        }),
    }
}
