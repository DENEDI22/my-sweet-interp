use std::collections::HashMap;

use crate::{BinaryOperator, Expr, Statement};

pub fn run(statements: &[Statement]) {
    let mut vars: HashMap<String, i32> = HashMap::new();
    for stmt in statements {
        match stmt {
            Statement::VarDecl { name, value } => {
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

fn eval(expr: &Expr, vars: &HashMap<String, i32>) -> i32 {
    match expr {
        Expr::Int(n) => *n,
        Expr::Var(name) => *vars
            .get(name)
            .unwrap_or_else(|| panic!("Undefined variable: {name}")),
        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let l = eval(left, vars);
            let r = eval(right, vars);
            match operator {
                BinaryOperator::Add => l + r,
                BinaryOperator::Subtract => l - r,
                BinaryOperator::Multiply => l * r,
                BinaryOperator::Divide => l / r,
            }
        }
    }
}
