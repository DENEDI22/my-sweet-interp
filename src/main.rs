use crate::{
    runtime::run,
    types::{BinaryOperator, Expr, Statement, Token, Type},
};
use std::fs;
mod runtime;
mod types;
fn main() {
    let source = fs::read_to_string("test.mbl").expect("Could not find the test.test file.");
    let tokens = tokenize(&source);
    print!("Tokens: \n {tokens:?} \n ");
    let statements = parse(&tokens);
    print!("Statements: \n {statements:?} \n\n");
    _ = run(&statements);
}

fn parse(tokens: &[Token]) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut current = 0;

    while current < tokens.len() {
        match &tokens[current] {
            Token::Var => {
                let mut name: String;
                let mut var_type: Type;
                current += 1;
                //Define type
                match &tokens[current] {
                    Token::IntTypeLiteral => var_type = Type::Int,
                    Token::CharTypeLiteral => var_type = Type::Char,
                    Token::BoolTypeLiteral => var_type = Type::Bool,
                    _ => panic!("Unknown or undefined type"),
                }
                current += 1;
                //Find name
                match &tokens[current] {
                    Token::Ident(x) => name = x.to_string(),
                    _ => panic!("Variable is not identified"),
                }
                current += 1;
                //Expect assign token
                assert_eq!(tokens[current], Token::Assign, "expected '='");
                current += 1;
                //Define Value
                let value = parse_expression(&tokens, &mut current);
                match var_type {
                    Type::Char => current += 1,
                    _ => {}
                }
                let statem = Statement::VarDecl {
                    name,
                    value,
                    var_type,
                };
                assert_eq!(tokens[current], Token::Semicolon, "expected ';'");
                current += 1;
                statements.push(statem);
            }

            Token::Ident(name) => {
                let name = name.clone();
                match tokens.get(current + 1) {
                    Some(Token::LeftParen) => {
                        current += 2;
                        let arg = parse_expression(tokens, &mut current);
                        assert_eq!(tokens[current], Token::RightParen, "expected ')'");
                        current += 1;
                        assert_eq!(tokens[current], Token::Semicolon, "expected ';'");
                        current += 1;
                        statements.push(Statement::FuncCall { name, arg });
                    }
                    Some(Token::Assign) => {
                        current += 2;
                        let value = parse_expression(tokens, &mut current);
                        assert_eq!(tokens[current], Token::Semicolon, "expected ';'");
                        current += 1;
                        statements.push(Statement::VarAssign { name, value });
                    }
                    _other => panic!(),
                }
            }
            token => {
                panic!("Unexpected token: {:?}", token);
            }
        }
    }

    statements
}

fn parse_expression(tokens: &[Token], pos: &mut usize) -> Expr {
    let mut left = parse_term(tokens, pos);
    while let Some(tok) = tokens.get(*pos) {
        let op = match tok {
            Token::PlusOperator => BinaryOperator::Add,
            Token::MinusOperator => BinaryOperator::Subtract,
            _ => break,
        };
        *pos += 1;
        let right = parse_term(tokens, pos);
        left = Expr::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        };
    }
    left
}

fn parse_term(tokens: &[Token], pos: &mut usize) -> Expr {
    let mut left = parse_primary(tokens, pos);
    while let Some(tok) = tokens.get(*pos) {
        let op = match tok {
            Token::StarOperator => BinaryOperator::Multiply,
            Token::SlashOperator => BinaryOperator::Divide,
            _ => break,
        };
        *pos += 1;
        let right = parse_primary(tokens, pos);
        left = Expr::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        };
    }
    left
}

fn parse_primary(tokens: &[Token], pos: &mut usize) -> Expr {
    match &tokens[*pos] {
        Token::IntValue(n) => {
            *pos += 1;
            Expr::Int(*n)
        }
        Token::True => {
            *pos += 1;
            Expr::Bool(true)
        }
        Token::False => {
            *pos += 1;
            Expr::Bool(false)
        }
        Token::SingleQuote => {
            *pos += 1;
            let expected_char: char;
            match &tokens[*pos] {
                Token::Ident(c) => {
                    if c.len() > 1 {
                        panic!("Only one character is expected");
                    }
                    expected_char = c.chars().next().unwrap();
                }

                Token::IntValue(c) => {
                    if c.to_string().len() > 1 {
                        panic!("Only one character is expected");
                    }
                    expected_char = c.to_string().chars().next().unwrap();
                }
                _ => panic!("Unexpected token for char type value"),
            }
            *pos += 1;
            Expr::Char(expected_char)
        }

        Token::LeftParen => {
            *pos += 1;
            let e = parse_expression(tokens, pos);
            assert_eq!(tokens[*pos], Token::RightParen, "expected ')'");
            *pos += 1;
            e
        }
        Token::Ident(name) => {
            *pos += 1;
            Expr::Var(name.clone())
        }

        t => panic!("Unexpected token in expression: {t:?}"),
    }
}

fn tokenize_word(word: &str) -> Token {
    match word {
        "var" => Token::Var,
        "char" => Token::CharTypeLiteral,
        "bool" => Token::BoolTypeLiteral,
        "int" => Token::IntTypeLiteral,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Ident(word.to_string()),
    }
}

fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut number = String::new();
                number.push(c);

                while let Some(next) = chars.peek() {
                    if next.is_ascii_digit() {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                tokens.push(Token::IntValue(number.parse().unwrap()));
            }

            c if c.is_alphabetic() => {
                let mut word = String::new();
                word.push(c);

                while let Some(next) = chars.peek() {
                    if next.is_alphabetic() {
                        word.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(tokenize_word(&word));
            }

            '\'' => tokens.push(Token::SingleQuote),
            '+' => tokens.push(Token::PlusOperator),
            '-' => tokens.push(Token::MinusOperator),
            '*' => tokens.push(Token::StarOperator),
            '/' => tokens.push(Token::SlashOperator),

            '=' => tokens.push(Token::Assign),
            ';' => tokens.push(Token::Semicolon),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),

            c if c.is_whitespace() => {}

            _ => panic!("Unknown character"),
        }
    }
    tokens
}
