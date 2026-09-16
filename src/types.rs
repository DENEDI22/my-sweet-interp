use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum Type {
    Int,
    Char,
    Bool,
}
#[derive(PartialEq, Debug)]
pub(crate) enum Token {
    Var,
    BoolTypeLiteral,
    IntTypeLiteral,
    CharTypeLiteral,
    Ident(String),
    IntValue(i32),
    PlusOperator,
    MinusOperator,
    StarOperator,
    SlashOperator,
    Assign,
    Semicolon,
    LeftParen,
    RightParen,
    SingleQuote,
    Null,
    True,
    False,
}

#[derive(Debug)]
pub(crate) enum Expr {
    Null,
    Int(i32),
    Bool(bool),
    Char(char),
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },

    Var(String),
}

#[derive(Debug, Clone)]
pub(crate) enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}
#[derive(Debug)]
pub(crate) enum RuntimeError {
    UndefinedVariable(String),
    DivisionByZero,
    InvalidOperation {
        left: Value,
        right: Value,
        op: BinaryOperator,
    },
    TypeMismatch {
        expected: Type,
        got: Value,
    },
    UnknownFunction(String),
}

#[derive(Debug)]
pub(crate) enum Statement {
    VarDecl {
        name: String,
        value: Expr,
        var_type: Type,
    },
    VarAssign {
        name: String,
        value: Expr,
    },
    FuncCall {
        name: String,
        arg: Expr,
    },
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Value {
    Int(i32),
    Char(char),
    Bool(bool),
    Null,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::UndefinedVariable(x) => write!(f, "Undefined Variable {x}"),

            RuntimeError::DivisionByZero => write!(f, "Division by zero!"),
            RuntimeError::TypeMismatch { expected, got } => {
                write!(f, "Type mismatch. Expected {expected:?} but got {got}")
            }
            RuntimeError::UnknownFunction(x) => {
                write!(f, "Function {x} is not found. Check your spelling")
            }
            RuntimeError::InvalidOperation { left, right, op } => {
                write!(f, "Cannot apply {op:?} to {left:?} and {right:?}")
            }
        }
    }
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
