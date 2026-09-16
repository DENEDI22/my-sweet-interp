#[derive(Debug)]
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

    Compare {
        left: Box<Expr>,
        operator: Comparator,
        right: Box<Expr>,
    },

    Predicate(),
}

#[derive(Debug)]
pub(crate) enum Comparator {
    Bigger,
    BiggerOrEq,
    Eq,
    Less,
    LessOrEq,
    NotEq,
}

#[derive(Debug)]
pub(crate) enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub(crate) enum Statement {
    Empty,
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
