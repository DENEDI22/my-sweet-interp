use std::fmt;

#[derive(Debug, Clone)]
pub(crate) enum Type {
    Int,
    Char,
    Bool,
}
#[derive(PartialEq, Debug, Clone)]
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
    //match
    Match,
    MoreThan,
    LessThan,
    Equal,
    NotEqual,
    Wildcard,
    CurlyBraceOpen,
    CurlyBraceClose,
    //loop
    Loop,
    Break,
}

#[derive(Debug, Clone)]
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
    VarId(usize),
}

#[derive(Debug, Clone)]
pub(crate) enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    LessThan,
    MoreThan,
    NotEqual,
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
    Exception(String),
}

#[derive(Debug)]
pub enum ResolutionError {
    UndefinedVariable(String),
    VariableIsAlreadyDefined(String),
}

#[derive(Debug)]
pub(crate) enum FlowState {
    None,
    Break,
    Finished,
}

#[derive(Debug, Clone)]
pub(crate) enum Statement {
    VarDecl {
        name: String,
        value: Expr,
        var_type: Type,
    },
    VarIdDecl {
        id: usize,
        value: Expr,
        var_type: Type,
    },
    VarAssign {
        name: String,
        value: Expr,
    },
    VarAssignById {
        id: usize,
        value: Expr,
    },
    FuncCall {
        name: String,
        arg: Expr,
    },
    Match {
        subject: Expr,
        arms: Vec<MatchArm>,
    },
    Loop {
        body: Vec<Statement>,
    },
    Break,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Expr),
    Wildcard,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
            RuntimeError::Exception(x) => write!(f, "{x}"),
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
