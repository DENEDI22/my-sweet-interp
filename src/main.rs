fn main() {
    let source = "var int x = 10 + 20 + 30; print(x);";
    let tokens = tokenize(source);
    let statements = parse(&tokens);
    println!("{statements:?}");
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
                match &tokens[current] {
                    Token::IntTypeLiteral => var_type = Type::Int,
                    _ => panic!("Unknown or undefined type"),
                }
                current += 1;
                match &tokens[current] {
                    Token::Ident(x) => name = x.to_string(),
                    _ => panic!("Variable is not identified"),
                }
                current += 1;
                assert_eq!(tokens[current], Token::Assign, "expected '='");
                current += 1;
                let value = parse_expression(&tokens, &mut current);
                let statem = Statement::VarDecl { name, value };
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
        "int" => Token::IntTypeLiteral,
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

enum Type {
    Int,
}
#[derive(PartialEq, Debug)]
enum Token {
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
enum Expr {
    Int(i32),

    Binary {
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },

    Var(String),
}

#[derive(Debug)]
enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Statement {
    Empty,
    VarDecl { name: String, value: Expr },
    VarAssign { name: String, value: Expr },
    FuncCall { name: String, arg: Expr },
}
