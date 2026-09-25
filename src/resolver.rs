use crate::types::{Expr, MatchArm, Pattern, ResolutionError, Statement};

pub struct Resolver {
    pub names: Vec<String>,
    scopes: Vec<(String, usize)>,
}

impl Resolver {
    fn declare(&mut self, name: &str) -> Result<usize, ResolutionError> {
        match self.lookup(name) {
            Ok(_) => return Err(ResolutionError::VariableIsAlreadyDefined(name.to_string())),
            Err(_) => {
                let id = self.names.len();
                self.names.push(name.to_string());
                self.scopes.push((name.to_string(), id));
                Ok(id)
            }
        }
    }

    fn lookup(&self, name: &str) -> Result<usize, ResolutionError> {
        self.scopes
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, id)| *id)
            .ok_or_else(|| ResolutionError::UndefinedVariable(name.to_string()))
    }

    pub fn resolve_block(
        &mut self,
        stmts: &[Statement],
    ) -> Result<Vec<Statement>, ResolutionError> {
        let mark = self.scopes.len();
        let out = stmts.iter().map(|s| self.resolve_stmt(s)).collect();
        self.scopes.truncate(mark);
        out
    }

    fn resolve_stmt(&mut self, stmt: &Statement) -> Result<Statement, ResolutionError> {
        match stmt {
            Statement::VarDecl {
                name,
                value,
                var_type,
            } => {
                let value = self.resolve_expr(value)?;
                let id = self.declare(name)?;
                Ok(Statement::VarIdDecl {
                    id,
                    value,
                    var_type: var_type.clone(),
                })
            }
            Statement::VarIdDecl {
                id: _,
                value: _,
                var_type: _,
            } => unreachable!(),
            Statement::VarAssign { name, value } => {
                let id = self.lookup(name)?;
                let res_value = self.resolve_expr(value)?;
                Ok(Statement::VarAssignById {
                    id: id,
                    value: res_value,
                })
            }
            Statement::VarAssignById { id: _, value: _ } => unreachable!(),
            Statement::FuncCall { name, arg } => Ok(Statement::FuncCall {
                name: name.clone(),
                arg: self.resolve_expr(arg)?,
            }),
            Statement::Match { subject, arms } => {
                let rsubject = self.resolve_expr(subject)?;
                let mut rarms: Vec<MatchArm> = Vec::new();
                for arm in arms {
                    let rpattern = match &arm.pattern {
                        Pattern::Literal(expr) => Pattern::Literal(self.resolve_expr(&expr)?),
                        Pattern::Wildcard => Pattern::Wildcard,
                        Pattern::HalfBinary(binary_operator, expr) => {
                            Pattern::HalfBinary(binary_operator.clone(), self.resolve_expr(expr)?)
                        }
                    };
                    let rbody = self.resolve_block(&arm.body)?;
                    rarms.push(MatchArm {
                        pattern: rpattern,
                        body: rbody,
                    });
                }
                Ok(Statement::Match {
                    subject: rsubject,
                    arms: rarms,
                })
            }
            Statement::Loop { body } => Ok(Statement::Loop {
                body: self.resolve_block(body)?,
            }),
            Statement::Break => Ok(Statement::Break),
            Statement::Continue => Ok(Statement::Continue),
            Statement::IndexAssign {
                target,
                index,
                value,
            } => Ok(Statement::IndexAssign {
                target: Box::new(self.resolve_expr(target)?),
                index: Box::new(self.resolve_expr(index)?),
                value: self.resolve_expr(value)?,
            }),
            Statement::Expr(expr) => Ok(Statement::Expr(self.resolve_expr(expr)?)),
        }
    }

    fn resolve_expr(&self, expr: &Expr) -> Result<Expr, ResolutionError> {
        Ok(match expr {
            Expr::Var(name) => Expr::VarId(self.lookup(name)?),
            Expr::Binary {
                left,
                operator,
                right,
            } => Expr::Binary {
                left: Box::new(self.resolve_expr(left)?),
                operator: operator.clone(),
                right: Box::new(self.resolve_expr(right)?),
            },
            Expr::Null => Expr::Null,
            Expr::Int(i) => Expr::Int(*i),
            Expr::Bool(b) => Expr::Bool(*b),
            Expr::Char(c) => Expr::Char(*c),
            Expr::String(s) => Expr::String(s.clone()),
            Expr::VarId(_) => unreachable!(),
            Expr::List(exprs) => Expr::List(
                exprs
                    .into_iter()
                    .map(|e| self.resolve_expr(e))
                    .collect::<Result<_, _>>()?,
            ),
            Expr::Index { target, index } => Expr::Index {
                target: Box::new(self.resolve_expr(target)?),
                index: Box::new(self.resolve_expr(index)?),
            },
            Expr::MethodCall {
                receiver,
                name,
                args,
            } => Expr::MethodCall {
                receiver: Box::new(self.resolve_expr(receiver)?),
                name: name.clone(),
                args: args
                    .into_iter()
                    .map(|e| self.resolve_expr(e))
                    .collect::<Result<_, _>>()?,
            },
        })
    }

    pub(crate) fn new() -> Self {
        Self {
            names: Vec::new(),
            scopes: Vec::new(),
        }
    }
}
