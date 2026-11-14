use crate::types::{BinaryOperator, Expr, Statement, Type, TypeError};

pub struct TypeChecker {
    var_types: Vec<Type>,
}

impl TypeChecker {
    fn synth(&self, e: &Expr) -> Result<Type, TypeError> {
        match e {
            Expr::Null => Ok(Type::None),
            Expr::Int(_) => Ok(Type::Int),
            Expr::Bool(_) => Ok(Type::Bool),
            Expr::Char(_) => Ok(Type::Char),
            Expr::String(_) => Ok(Type::String),
            Expr::List(exprs) => {
                //empty list value [] is just a list of nulls with len = 0. This way any list can be
                //flushed with 'my_list = [];'
                if exprs.is_empty() {
                    return Ok(Type::List(Box::new(Type::None)));
                }
                let mut first_expr_type = self.synth(&exprs.iter().next().unwrap())?;
                //for [null, 1, null] case
                if matches!(first_expr_type, Type::None) {
                    for expr in exprs {
                        match self.synth(&expr)? {
                            Type::None => continue,
                            t => {
                                first_expr_type = t;
                                break;
                            }
                        }
                    }
                }
                if !matches!(first_expr_type, Type::None) {
                    for expr in exprs {
                        self.check(&expr, &first_expr_type)?;
                    }
                }
                Ok(Type::List(Box::new(first_expr_type)))
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l = self.synth(left)?;
                let r = self.synth(right)?;
                match binary_result_type(&l, operator, &r) {
                    Some(t) => Ok(t),
                    None => Err(TypeError::TypeMissing(
                        "Could not resolve type for a result of a binary expression".to_string(),
                    )),
                }
            }
            Expr::Var(_) => {
                unreachable!();
            }
            Expr::VarId(i) => {
                //govnokod, mb optimise later
                let var = self
                    .var_types
                    .get(*i)
                    .ok_or(TypeError::TypeMissing("flskjföa".to_string()))?;
                Ok(var.clone())
            }
            Expr::Index { target, index } => {
                let t = self.synth(target)?;
                self.check(index, &Type::Int)?;
                match t {
                    Type::List(elem) => Ok(*elem),
                    _ => todo!(),
                }
            }
            Expr::MethodCall {
                receiver,
                name,
                args,
            } => {
                let r = self.synth(receiver)?;
                match (&r, name.as_str()) {
                    (Type::List(_), "len") => {
                        if !args.is_empty() {
                            return Err(TypeError::SignatureError(
                                "len() takes no arguments".to_string(),
                            ));
                        }
                        Ok(Type::Int)
                    }
                    (Type::List(elem), "push") => {
                        if args.len() != 1 {
                            return Err(TypeError::SignatureError(
                                "push() takes exactly one argument".to_string(),
                            ));
                        }
                        self.check(&args[0], elem)?;
                        Ok(Type::None) // push ничего не возвращает; Null сюда и так присваивается во что угодно
                    }
                    (Type::List(_), _) => Err(TypeError::SignatureError(format!(
                        "unknown method .{name}()"
                    ))),
                    _ => Err(TypeError::SignatureError(format!(
                        "type has no methods, got .{name}()"
                    ))),
                }
            }
        }
    } // тип из выражения
    fn check(&self, e: &Expr, expected: &Type) -> Result<(), TypeError> {
        todo!();
    } // проверка против ожидания
    fn check_stmt(&mut self, s: &Statement) -> Result<(), TypeError> {
        todo!();
    }
}

pub fn binary_result_type(l: &Type, op: &BinaryOperator, r: &Type) -> Option<Type> {
    use BinaryOperator::*;
    match (l, op, r) {
        (Type::Int, Add | Subtract | Multiply | Divide, Type::Int) => Some(Type::Int),
        (Type::Int, Add, Type::None) | (Type::None, Add, Type::Int) => Some(Type::Int),

        (_, Equal | NotEqual, _) => Some(Type::Bool),
        (Type::Int, LessThan | MoreThan, Type::Int) => Some(Type::Bool),

        (Type::String, Add, Type::String)
        | (Type::String, Add, Type::Char)
        | (Type::Char, Add, Type::String)
        | (Type::Char, Add, Type::Char)
        | (Type::String, Add, Type::Int)
        | (Type::Int, Add, Type::String)
        | (Type::String, Add, Type::None)
        | (Type::None, Add, Type::String)
        | (Type::Char, Add, Type::None)
        | (Type::None, Add, Type::Char) => Some(Type::String),

        _ => None,
    }
}
