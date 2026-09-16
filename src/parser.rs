use crate::types::{BinaryOperator, Expr, Statement, Token, Type};

pub fn parse(tokens: &[Token]) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut current = 0;

    while current < tokens.len() {
        match &tokens[current] {
            Token::Var => {
                let name: String;
                let var_type: Type;
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
        Token::Null => {
            *pos += 1;
            Expr::Null
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
