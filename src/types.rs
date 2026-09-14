pub(crate) enum Type {
    Int,
}
#[derive(PartialEq, Debug)]
pub(crate) enum Token {
    Var,
    IntTypeLiteral,
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
}

#[derive(Debug)]
pub(crate) enum Expr {
    Int(i32),

    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },

    Var(String),
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
    VarDecl { name: String, value: Expr },
    VarAssign { name: String, value: Expr },
    FuncCall { name: String, arg: Expr },
}
