fn main() {
    let source = "var int x = 10 + 20;";

    let tokens = tokenize(source);

    println!("{tokens:?}");
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
                let mut expr_tokens = Vec::new();
                while tokens[current] != Token::Semicolon {
                    expr_tokens.push(&tokens[current]);
                    current += 1;
                }
                let statem = Statement::VarDecl {
                    name,
                    value: parse_expression(&expr_tokens),
                };
            }

            Token::Ident(_) => {}

            token => {
                panic!("Unexpected token: {:?}", token);
            }
        }
    }

    statements
}

fn parse_expression(tokens: &[Token]) -> Expr {}

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
    Add(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
enum Statement {
    Empty,
    VarDecl { name: String, value: Expr },
    FuncCall { name: String },
}
