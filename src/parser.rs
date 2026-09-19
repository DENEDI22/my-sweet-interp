use crate::types::{BinaryOperator, Expr, MatchArm, Pattern, Statement, Token, Type};

pub fn parse(tokens: &[Token]) -> Vec<Statement> {
    let mut statements = Vec::new();
    let mut current = 0;

    while current < tokens.len() {
        statements.push(parse_statement(tokens, &mut current));
    }

    statements
}

fn parse_statement(tokens: &[Token], current: &mut usize) -> Statement {
    match &tokens[*current] {
        Token::Var => {
            let name: String;
            let var_type: Type;
            *current += 1;
            //Define type
            match &tokens[*current] {
                Token::IntTypeLiteral => var_type = Type::Int,
                Token::CharTypeLiteral => var_type = Type::Char,
                Token::BoolTypeLiteral => var_type = Type::Bool,
                Token::StringTypeLiteral => var_type = Type::String,
                _ => panic!("Unknown or undefined type"),
            }
            *current += 1;
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
                _other => panic!(),
            }
        }
        Token::Match => {
            *current += 1;
            let subj = parse_expression(tokens, current);
            assert_eq!(tokens[*current], Token::CurlyBraceOpen, "expected '{{'");
            *current += 1;
            let mut arms: Vec<MatchArm> = Vec::new();
            while !matches!(tokens[*current], Token::CurlyBraceClose) {
                let arm_pattern: Pattern = match tokens[*current] {
                    Token::Wildcard => {
                        *current += 1;
                        Pattern::Wildcard
                    }
                    _ => Pattern::Literal(parse_expression(tokens, current)),
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

        Token::Ident(name) => {
            *pos += 1;
            Expr::Var(name.clone())
        }

        t => panic!("Unexpected token in expression: {t:?}"),
    }
}
