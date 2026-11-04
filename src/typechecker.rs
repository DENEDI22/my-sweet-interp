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
                for expr in exprs {
                    self.check(&expr, &first_expr_type)?;
                }
                Ok(Type::List(Box::new(first_expr_type)))
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => match operator {
                BinaryOperator::Equal
                | BinaryOperator::NotEqual
                | BinaryOperator::LessThan
                | BinaryOperator::MoreThan => Ok(Type::Bool),
                _ => self.synth(left),
            },
            Expr::Var(_) => todo!(),
            Expr::VarId(_) => todo!(),
            Expr::Index { target, index } => todo!(),
            Expr::MethodCall {
                receiver,
                name,
                args,
            } => todo!(),
        }
    } // тип из выражения
    fn check(&self, e: &Expr, expected: &Type) -> Result<(), TypeError> {
        todo!();
    } // проверка против ожидания
    fn check_stmt(&mut self, s: &Statement) -> Result<(), TypeError> {
        todo!();
    }
}
