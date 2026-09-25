use std::rc::Rc;

use crate::{
    resolver::Resolver,
    types::{
        BinaryOperator, Expr, FlowState, Object,
        Pattern::{HalfBinary, Literal, Wildcard},
        RuntimeError::{self},
        Statement, Type, Value,
    },
};

pub struct Runtime {
    vars: Vec<Value>,
    heap: Vec<Object>,
    types: Vec<Type>,
    resolver: Resolver,
}

impl Runtime {
    pub fn run(&mut self, statements: &[Statement]) -> Result<FlowState, RuntimeError> {
        for stmt in statements {
            match self.run_statement(stmt)? {
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

    fn run_loop(&mut self, statements: &[Statement]) -> Result<FlowState, RuntimeError> {
        loop {
            match self.run_block(statements)? {
                FlowState::Break => break,
                _ => {}
            }
        }
        Ok(FlowState::None)
    }

    fn run_block(&mut self, statements: &[Statement]) -> Result<FlowState, RuntimeError> {
        for stmt in statements {
            match self.run_statement(stmt)? {
                FlowState::None => {}
                flow => return Ok(flow),
            }
        }
        Ok(FlowState::None)
    }

    fn value_matches(&self, v: &Value, t: &Type) -> bool {
        match (v, t) {
            (Value::Null, _) => true,
            (Value::Int(_), Type::Int) => true,
            (Value::Char(_), Type::Char) => true,
            (Value::Bool(_), Type::Bool) => true,
            (Value::String(_), Type::String) => true,
            (Value::List(h), Type::List(inner)) => match &self.heap[*h] {
                Object::List(items) => items.iter().all(|item| self.value_matches(item, inner)),
            },
            _ => false,
        }
    }

    fn run_statement(&mut self, stmt: &Statement) -> Result<FlowState, RuntimeError> {
        match stmt {
            Statement::VarIdDecl {
                id,
                value,
                var_type,
            } => {
                let v = self.eval(value)?;
                if !self.value_matches(&v, var_type) {
                    return Err(RuntimeError::TypeMismatch {
                        expected: var_type.clone(),
                        got: v,
                    });
                }
                self.vars[*id] = v;
                self.types[*id] = var_type.clone();
            }
            Statement::FuncCall { name, arg } => match name.as_str() {
                "print" => println!("{}", self.eval(arg)?),
                _ => return Err(RuntimeError::UnknownFunction(name.clone())),
            },
            Statement::VarAssignById { id, value } => {
                let v = self.eval(value)?;
                if self.value_matches(&v, &self.types[*id]) {
                    self.vars[*id] = v;
                } else {
                    return Err(RuntimeError::Exception(
                        "Type mismatch by assigning".to_string(),
                    ));
                }
            }
            Statement::Match { subject, arms } => {
                let subj = self.eval(subject)?;
                for arm in arms {
                    let matched = match &arm.pattern {
                        Wildcard => true,
                        Literal(expr) => self.eval(expr)? == subj,
                        HalfBinary(op, expr) => {
                            let rhs = self.eval(expr)?;
                            self.eval_binary(&subj, &rhs, op)? == Value::Bool(true)
                        }
                    };
                    if matched {
                        return self.run_block(&arm.body);
                    }
                }
            }
            Statement::Loop { body } => {
                self.run_loop(body)?;
            }
            Statement::Break => return Ok(FlowState::Break),
            Statement::Continue => return Ok(FlowState::Continue),
            Statement::VarDecl {
                name: _,
                value: _,
                var_type: _,
            } => unreachable!(),
            Statement::VarAssign { name: _, value: _ } => unreachable!(),
            Statement::IndexAssign {
                target,
                index,
                value,
            } => {
                let t = self.eval(target)?;
                let i = self.eval(index)?;
                let v = self.eval(value)?;
                match (t, i) {
                    (Value::List(h), Value::Int(i)) => {
                        let Object::List(values) = &mut self.heap[h];
                        if i < 0 || i as usize >= values.len() {
                            return Err(RuntimeError::Exception(format!(
                                "Index {} out of bounds for list of length {}",
                                i,
                                values.len()
                            )));
                        }
                        values[i as usize] = v;
                    }
                    (Value::List(_), _) => {
                        return Err(RuntimeError::Exception("Index is not a number".to_string()));
                    }
                    _ => {
                        return Err(RuntimeError::Exception(
                            "Trying to assign a value by index to a non-indexable type."
                                .to_string(),
                        ));
                    }
                }
            }
            Statement::Expr(expr) => {
                self.eval(expr)?;
            }
        }
        Ok(FlowState::None)
    }
    fn eval(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
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
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                self.eval_binary(&l, &r, &operator)
            }
            Expr::VarId(i) => {
                let get = self.vars.get(*i);
                match get {
                    Some(v) => Ok(v.clone()),
                    None => Err(RuntimeError::UndefinedVariable(
                        self.resolver.names[*i].clone(),
                    )),
                }
            }
            Expr::String(s) => Ok(Value::String(Rc::from(s.clone()))),
            Expr::List(exprs) => {
                let mut new_list: Vec<Value> = Vec::new();
                for e in exprs {
                    new_list.push(self.eval(e)?);
                }
                self.heap.push(Object::List(new_list));
                Ok(Value::List(self.heap.len() - 1))
            }
            Expr::Index { target, index } => {
                let t = self.eval(target)?;
                let i = self.eval(index)?;
                match (t, i) {
                    (Value::List(h), Value::Int(i)) => {
                        let Object::List(values) = &self.heap[h];
                        if i < 0 || i as usize >= values.len() {
                            return Err(RuntimeError::Exception(format!(
                                "Index {} out of bounds for list of length {}",
                                i,
                                values.len()
                            )));
                        }
                        Ok(values[i as usize].clone())
                    }
                    (Value::List(_), _) => {
                        Err(RuntimeError::Exception("Index is not a number".to_string()))
                    }
                    _ => Err(RuntimeError::Exception(
                        "Trying to read a value by index from a non-indexable type.".to_string(),
                    )),
                }
            }
            Expr::MethodCall {
                receiver,
                name,
                args,
            } => {
                let r = self.eval(receiver)?;
                match (&r, name.as_str()) {
                    (Value::List(h), "len") => {
                        if !args.is_empty() {
                            return Err(RuntimeError::Exception(
                                "len() takes no arguments".to_string(),
                            ));
                        }
                        let Object::List(values) = &self.heap[*h];
                        Ok(Value::Int(values.len() as i32))
                    }
                    (Value::List(h), "push") => {
                        if !args.is_empty() {
                            if args.len() > 1 {
                                return Err(RuntimeError::Exception(format!(
                                    "More than 1 arg in 'push' method. Exactly one arg is required"
                                )));
                            }
                            let new_value = self.eval(&args[0])?;
                            let Object::List(values) = &mut self.heap[*h];
                            values.push(new_value);
                            Ok(Value::Null)
                        } else {
                            Err(RuntimeError::Exception(format!(
                                "Empty args of 'push' method. Value is required"
                            )))
                        }
                    }
                    (Value::Null, _) => Err(RuntimeError::Exception(format!(
                        "Method call .{}() on null",
                        name
                    ))),
                    _ => Err(RuntimeError::Exception(format!(
                        "Unknown method .{}() for this type",
                        name
                    ))),
                }
            }
        }
    }

    fn eval_binary(
        &self,
        left: &Value,
        right: &Value,
        op: &BinaryOperator,
    ) -> Result<Value, RuntimeError> {
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
            (l, BinaryOperator::Equal, r) => Ok(Value::Bool(l == r)),
            (l, BinaryOperator::NotEqual, r) => Ok(Value::Bool(l != r)),
            (Value::Int(a), BinaryOperator::LessThan, Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Int(a), BinaryOperator::MoreThan, Value::Int(b)) => Ok(Value::Bool(a > b)),
            //concat
            (Value::String(a), BinaryOperator::Add, Value::String(b)) => {
                let mut s = String::with_capacity(a.len() + b.len());
                s.push_str(a);
                s.push_str(b);
                Ok(Value::String(Rc::from(s)))
            }
            (Value::String(a), BinaryOperator::Add, Value::Char(b)) => {
                let mut s = String::with_capacity(a.len() + 1);
                s.push_str(a);
                s.push(*b);
                Ok(Value::String(Rc::from(s)))
            }
            (Value::Char(a), BinaryOperator::Add, Value::String(b)) => {
                let mut s = String::with_capacity(b.len() + 1);
                s.push(*a);
                s.push_str(b);
                Ok(Value::String(Rc::from(s)))
            }
            (Value::Char(a), BinaryOperator::Add, Value::Char(b)) => {
                let mut s = String::with_capacity(2);
                s.push(*a);
                s.push(*b);
                Ok(Value::String(Rc::from(s)))
            }
            (Value::String(a), BinaryOperator::Add, Value::Int(b)) => {
                let int_converted = b.to_string();
                let mut s = String::with_capacity(a.len() + int_converted.len());
                s.push_str(a);
                s.push_str(&int_converted);
                Ok(Value::String(Rc::from(s)))
            }
            (Value::Int(a), BinaryOperator::Add, Value::String(b)) => {
                let int_converted = a.to_string();
                let mut s = String::with_capacity(b.len() + int_converted.len());
                s.push_str(&int_converted);
                s.push_str(b);
                Ok(Value::String(Rc::from(s)))
            }
            (Value::String(a), BinaryOperator::Add, Value::Null) => Ok(Value::String(a.clone())),
            (Value::Null, BinaryOperator::Add, Value::String(a)) => Ok(Value::String(a.clone())),
            (Value::Char(a), BinaryOperator::Add, Value::Null) => Ok(Value::Char(*a)),
            (Value::Null, BinaryOperator::Add, Value::Char(a)) => Ok(Value::Char(*a)),
            (left, op, right) => Err(RuntimeError::InvalidOperation {
                left: left.clone(),
                right: right.clone(),
                op: op.clone(),
            }),
        }
    }

    pub(crate) fn new(resolver: Resolver) -> Self {
        let vars = vec![Value::Null; resolver.names.len()];
        let types = vec![Type::None; resolver.names.len()];
        Self {
            vars,
            heap: Vec::new(),
            types,
            resolver,
        }
    }
}
