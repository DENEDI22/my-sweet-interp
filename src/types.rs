use std::{
    fmt::{self, write},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub(crate) enum Type {
    None,
    Int,
    Char,
    Bool,
    String,
    List(Box<Type>),
}
#[derive(PartialEq, Debug, Clone)]
pub(crate) enum Token {
    Var,
    BoolTypeLiteral,
    IntTypeLiteral,
    CharTypeLiteral,
    StringTypeLiteral,
    Ident(String),
    IntValue(i32),
    StringValue(Rc<str>),
    CharValue(char),
    ListValue(Vec<Token>),
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
    Continue,
    //strings
    DoubleQuotes,
    //arrays
    ListTypeLiteral,
    SquaredParenOpen,
    SquaredParenClose,
    Dot,
    Comma,
}

#[derive(Debug, Clone)]
pub(crate) enum Expr {
    Null,
    Int(i32),
    Bool(bool),
    Char(char),
    String(Rc<str>),
    List(Vec<Expr>),
    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Var(String),
    VarId(usize),
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    MethodCall {
        receiver: Box<Expr>,
        name: String,
        args: Vec<Expr>,
    },
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
    Continue,
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
    Continue,
    IndexAssign {
        target: Box<Expr>,
        index: Box<Expr>,
        value: Expr,
    },
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Expr),
    HalfBinary(BinaryOperator, Expr),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Value {
    Int(i32),
    Char(char),
    Bool(bool),
    String(Rc<str>),
    List(usize),
    Null,
}

#[derive(Debug)]
pub(crate) enum Object {
    List(Vec<Value>),
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
            Value::String(s) => write!(f, "{s}"),
            Value::Null => write!(f, "null"),
            Value::List(i) => write!(f, "{i}"),
        }
    }
}
