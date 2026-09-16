use crate::types::Token;

fn tokenize_word(word: &str) -> Token {
    match word {
        "var" => Token::Var,
        "char" => Token::CharTypeLiteral,
        "bool" => Token::BoolTypeLiteral,
        "int" => Token::IntTypeLiteral,
        "true" => Token::True,
        "false" => Token::False,
        "null" => Token::Null,
        "match" => Token::Match,
        _ => Token::Ident(word.to_string()),
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
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

            '<' => tokens.push(Token::LessThan),
            '>' => tokens.push(Token::MoreThan),
            '!' => match chars.peek().unwrap() {
                '=' => {
                    chars.next().unwrap();
                    tokens.push(Token::NotEqual);
                }
                _ => {}
            },
            '=' => match chars.peek().unwrap() {
                '=' => {
                    chars.next().unwrap();
                    tokens.push(Token::Equal)
                }
                _ => tokens.push(Token::Assign),
            },
            '_' => tokens.push(Token::Wildcard),
            ';' => tokens.push(Token::Semicolon),
            '(' => tokens.push(Token::LeftParen),
            ')' => tokens.push(Token::RightParen),
            '{' => tokens.push(Token::CurlyBraceOpen),
            '}' => tokens.push(Token::CurlyBraceClose),

            c if c.is_whitespace() => {}

            _ => panic!("Unknown character"),
        }
    }
    tokens
}
