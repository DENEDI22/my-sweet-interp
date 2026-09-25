use core::panic;

use crate::types::{
    BinaryOperator, Expr, MatchArm, Pattern, Statement,
    Token::{self},
    Type,
};

pub fn parse(tokens: &[Token]) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut current = 0;

    while current < tokens.len() {
        statements.push(parse_statement(tokens, &mut current));
    }

    statements
}

fn parse_type_literal(tokens: &[Token], current: &mut usize) -> Type {
    let expected_type = match &tokens[*current] {
        Token::IntTypeLiteral => Type::Int,
        Token::CharTypeLiteral => Type::Char,
        Token::BoolTypeLiteral => Type::Bool,
        Token::StringTypeLiteral => Type::String,
        Token::ListTypeLiteral => {
            *current += 1;
            assert_eq!(tokens[*current], Token::SquaredParenOpen);
            *current += 1;
            let expected_type = parse_type_literal(tokens, current);
            assert_eq!(tokens[*current], Token::SquaredParenClose);
            Type::List(Box::new(expected_type))
        }
        _ => panic!("Unknown or undefined type"),
    };
    *current += 1;
    expected_type
}
fn parse_statement(tokens: &[Token], current: &mut usize) -> Statement {
    match &tokens[*current] {
        Token::Var => {
            let name: String;
            *current += 1;
            //Define type
            let var_type = parse_type_literal(tokens, current);
            //Find name
            match &tokens[*current] {
                Token::Ident(x) => name = x.to_string(),
                _ => panic!("Variable is not identified"),
            }
            *current += 1;
            //Expect assign token
            assert_eq!(tokens[*current], Token::Assign, "expected '='");
            *current += 1;
            //Define Value
            let value = parse_expression(&tokens, current);

            let statem = Statement::VarDecl {
                name,
                value,
                var_type,
            };
            assert_eq!(tokens[*current], Token::Semicolon, "expected ';'");
            *current += 1;
            statem
        }
        Token::Ident(name) => {
            let name = name.clone();
            match tokens.get(*current + 1) {
                Some(Token::LeftParen) => {
                    *current += 2;
                    let arg = parse_expression(tokens, current);
                    assert_eq!(tokens[*current], Token::RightParen, "expected ')'");
                    *current += 1;
                    assert_eq!(tokens[*current], Token::Semicolon, "expected ';'");
                    *current += 1;
                    Statement::FuncCall { name, arg }
                }
                Some(Token::Assign) => {
                    *current += 2;
                    let value = parse_expression(tokens, current);
                    assert_eq!(tokens[*current], Token::Semicolon, "expected ';'");
                    *current += 1;
                    Statement::VarAssign { name, value }
                }
                Some(Token::SquaredParenOpen) | Some(Token::Dot) => {
                    let target = parse_expression(tokens, current);
                    match &tokens[*current] {
                        Token::Assign => {
                            *current += 1;
                            let value = parse_expression(tokens, current);
                            assert_eq!(tokens[*current], Token::Semicolon, "expected ';'");
                            *current += 1;
                            match target {
                                Expr::Index { target, index } => Statement::IndexAssign {
                                    target,
                                    index,
                                    value,
                                },
                                other => panic!("cannot assign to {:?}", other),
                            }
                        }
                        Token::Semicolon => {
                            *current += 1;
                            Statement::Expr(target)
                        }
                        other => panic!("expected '=' or ';' after expression, got {:?}", other),
                    }
                }
                other => panic!("unexpected token after identifier: {:?}", other),
            }
        }
        Token::Match => {
            *current += 1;
            let subj = parse_expression(tokens, current);
            let mut has_wildcard = false;
            assert_eq!(tokens[*current], Token::CurlyBraceOpen, "expected '{{'");
            *current += 1;
            let mut arms: Vec<MatchArm> = Vec::new();
            while !matches!(tokens[*current], Token::CurlyBraceClose) {
                let arm_pattern: Pattern = match tokens[*current] {
                    Token::Wildcard => {
                        *current += 1;
                        if has_wildcard {
                            panic!("More than one wildcard found!")
                        }
                        has_wildcard = true;
                        Pattern::Wildcard
                    }
                    Token::Equal | Token::NotEqual | Token::LessThan | Token::MoreThan => {
                        if has_wildcard {
                            panic!("Wildcard has to be the last statement in the match expression")
                        }
                        let op = match tokens[*current] {
                            Token::Equal => Some(BinaryOperator::Equal),
                            Token::NotEqual => Some(BinaryOperator::NotEqual),
                            Token::LessThan => Some(BinaryOperator::LessThan),
                            Token::MoreThan => Some(BinaryOperator::MoreThan),
                            _ => panic!("unexpected opeartor in hals-binary expression"),
                        }
                        .unwrap();
                        *current += 1;
                        Pattern::HalfBinary(op, parse_additive(tokens, current))
                    }
                    _ => {
                        if has_wildcard {
                            panic!("Wildcard has to be the last statement in the match expression")
                        }
                        Pattern::Literal(parse_expression(tokens, current))
                    }
                };
                assert_eq!(tokens[*current], Token::CurlyBraceOpen, "expected '{{'");
                let body = parse_block(tokens, current);
                arms.push(MatchArm {
                    pattern: arm_pattern,
                    body: body,
                });
            }
            *current += 1;
            Statement::Match {
                subject: subj,
                arms,
            }
        }
        Token::Loop => {
            *current += 1;
            assert_eq!(tokens[*current], Token::CurlyBraceOpen, "expected '{{'");
            let stmt = Statement::Loop {
                body: parse_block(tokens, current),
            };
            stmt
        }
        Token::Break => {
            *current += 1;
            Statement::Break
        }
        Token::Continue => {
            *current += 1;
            Statement::Continue
        }
        token => {
            panic!("Unexpected token: {token:?} at position {current:?}");
        }
    }
}

fn parse_block(tokens: &[Token], current: &mut usize) -> Vec<Statement> {
    assert_eq!(tokens[*current], Token::CurlyBraceOpen);
    *current += 1;
    let mut body = Vec::new();
    while !matches!(tokens[*current], Token::CurlyBraceClose) {
        body.push(parse_statement(tokens, current));
    }
    *current += 1;
    body
}

fn parse_expression(tokens: &[Token], pos: &mut usize) -> Expr {
    parse_comparison(tokens, pos)
}

fn parse_comparison(tokens: &[Token], pos: &mut usize) -> Expr {
    let mut left = parse_additive(tokens, pos);
    loop {
        let op = match tokens[*pos] {
            Token::MoreThan => BinaryOperator::MoreThan,
            Token::LessThan => BinaryOperator::LessThan,
            Token::Equal => BinaryOperator::Equal,
            Token::NotEqual => BinaryOperator::NotEqual,
            _ => break,
        };
        *pos += 1;
        let right = parse_additive(tokens, pos);
        left = Expr::Binary {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        };
    }
    left
}
fn parse_additive(tokens: &[Token], pos: &mut usize) -> Expr {
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
    let mut left = parse_postfix(tokens, pos);
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
        Token::Null => {
            *pos += 1;
            Expr::Null
        }
        Token::SingleQuote => {
            *pos += 1;
            let expected_char = match &tokens[*pos] {
                Token::CharValue(c) => c,
                _ => panic!("Unexpected token for char type value"),
            };
            *pos += 1;
            assert_eq!(
                tokens[*pos],
                Token::SingleQuote,
                "expected ''' (single quote)"
            );
            *pos += 1;
            Expr::Char(*expected_char)
        }

        Token::DoubleQuotes => {
            *pos += 1;
            let expected_string = match &tokens[*pos] {
                Token::StringValue(s) => s,
                _ => panic!("Expected string value"),
            };
            *pos += 1;
            assert_eq!(
                tokens[*pos],
                Token::DoubleQuotes,
                "Closing double quotes not found."
            );
            *pos += 1;
            Expr::String(expected_string.clone())
        }

        Token::LeftParen => {
            *pos += 1;
            let e = parse_expression(tokens, pos);
            assert_eq!(tokens[*pos], Token::RightParen, "expected ')'");
            *pos += 1;
            e
        }

        Token::SquaredParenOpen => {
            *pos += 1;
            let mut vals: Vec<Expr> = Vec::new();
            if !matches!(tokens[*pos], Token::SquaredParenClose) {
                loop {
                    vals.push(parse_expression(tokens, pos));
                    match tokens[*pos] {
                        Token::Comma => *pos += 1,
                        Token::SquaredParenClose => break,
                        _ => panic!("Unexpected token in array literal description"),
                    }
                }
            }
            *pos += 1;
            Expr::List(vals)
        }

        Token::Ident(name) => {
            *pos += 1;
            match &tokens[*pos + 1] {
                Token::SquaredParenOpen => {
                    *pos += 2;
                    let ind = parse_expression(tokens, pos);
                    Expr::Index {
                        target: Box::new(Expr::Var(name.clone())),
                        index: Box::new(ind),
                    }
                }
                _ => Expr::Var(name.clone()),
            }
        }

        t => panic!("Unexpected token in expression: {t:?}"),
    }
}

fn parse_postfix(tokens: &[Token], current: &mut usize) -> Expr {
    let mut expr = parse_primary(tokens, current);

    loop {
        match &tokens[*current] {
            Token::SquaredParenOpen => {
                *current += 1;
                let index = parse_expression(tokens, current);
                assert_eq!(tokens[*current], Token::SquaredParenClose, "Expected ]");
                *current += 1;
                expr = Expr::Index {
                    target: Box::new(expr),
                    index: Box::new(index),
                };
            }
            Token::Dot => {
                *current += 1;
                let name = match &tokens[*current] {
                    Token::Ident(n) => n.to_string(),
                    other => panic!("Expected method name after '.', got {:?}", other),
                };
                *current += 1;
                assert_eq!(
                    tokens[*current],
                    Token::LeftParen,
                    "Expected ( after method name"
                );
                *current += 1;
                let args = parse_args(tokens, current);
                expr = Expr::MethodCall {
                    receiver: Box::new(expr),
                    name,
                    args,
                };
            }
            _ => break,
        }
    }
    expr
}

fn parse_args(tokens: &[Token], current: &mut usize) -> Vec<Expr> {
    let mut args = Vec::new();
    if tokens[*current] == Token::RightParen {
        *current += 1;
        return args;
    }
    loop {
        args.push(parse_expression(tokens, current));
        match &tokens[*current] {
            Token::Comma => *current += 1,
            Token::RightParen => {
                *current += 1;
                break;
            }
            other => panic!("Expected , or ) in argument list, got {:?}", other),
        }
    }
    args
}
