use std::collections::HashMap;

use crate::{token::Token, token_type::TokenType};

pub fn tokenize(source_code: String) -> Result<Vec<Token>, i32> {
    let mut src = source_code;
    let mut line: u32 = 1;

    let mut tokens: Vec<Token> = Vec::new();

    while !src.is_empty() {
        let ch = src.remove(0);
        let next_ch = src.chars().next();

        match ch {
            '(' => tokens.push(Token::new(TokenType::LeftParen, ch.to_string(), line)),
            ')' => tokens.push(Token::new(TokenType::RightParen, ch.to_string(), line)),
            '{' => tokens.push(Token::new(TokenType::LeftBrace, ch.to_string(), line)),
            '}' => tokens.push(Token::new(TokenType::RightBrace, ch.to_string(), line)),
            ',' => tokens.push(Token::new(TokenType::Comma, ch.to_string(), line)),
            '.' => tokens.push(Token::new(TokenType::Dot, ch.to_string(), line)),
            '-' => tokens.push(Token::new(TokenType::Minus, ch.to_string(), line)),
            '+' => tokens.push(Token::new(TokenType::Plus, ch.to_string(), line)),
            ';' => tokens.push(Token::new(TokenType::Semicolon, ch.to_string(), line)),
            '*' => tokens.push(Token::new(TokenType::Star, ch.to_string(), line)),
            '!' if next_ch == Some('=') => {
                tokens.push(Token::new(TokenType::BangEqual, "!=".to_string(), line));
                src.remove(0);
            }
            '!' => {
                tokens.push(Token::new(TokenType::Bang, ch.to_string(), line));
            }
            '=' if next_ch == Some('=') => {
                tokens.push(Token::new(TokenType::EqualEqual, "==".to_string(), line));
                src.remove(0);
            }
            '=' => {
                tokens.push(Token::new(TokenType::Equal, ch.to_string(), line));
            }
            '<' if next_ch == Some('=') => {
                tokens.push(Token::new(TokenType::LessEqual, "<=".to_string(), line));
                src.remove(0);
            }
            '<' => {
                tokens.push(Token::new(TokenType::Less, ch.to_string(), line));
            }
            '>' if next_ch == Some('=') => {
                tokens.push(Token::new(TokenType::GreaterEqual, ">=".to_string(), line));
                src.remove(0);
            }
            '>' => {
                tokens.push(Token::new(TokenType::Greater, ch.to_string(), line));
            }
            '/' if next_ch == Some('/') => {
                while !src.is_empty() && next_ch != Some('\n') {
                    src.remove(0);
                }
            }
            '/' => {
                tokens.push(Token::new(TokenType::Slash, ch.to_string(), line));
            }
            '\n' => line += 1,
            ' ' => {}
            '"' => match tokenize_string(&mut src, &mut line) {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            },
            '0'..='9' => match tokenize_number(&mut src, &mut line, ch) {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            },
            'a'..='z' | 'A'..='Z' => match tokenize_identifier(&mut src, &mut line, ch) {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            },

            _ => {
                error(line, "Unexpected character.");
                return Err(65);
            }
        }
    }

    tokens.push(Token::new(TokenType::EOF, "".to_string(), line));
    return Ok(tokens);
}

fn tokenize_string(src: &mut String, line: &mut u32) -> Result<Token, i32> {
    let mut string = "".to_string();

    while !src.is_empty() && !src.starts_with('"') {
        let ch = src.remove(0);
        if ch == '\n' {
            *line += 1;
        } else {
            string.push(ch);
        }
    }

    if src.is_empty() {
        error(*line, "Unterminated string.");
        return Err(65);
    }

    src.remove(0);
    return Ok(Token::new(TokenType::String, string, *line));
}

fn tokenize_number(src: &mut String, line: &mut u32, first_digit: char) -> Result<Token, i32> {
    let mut num = first_digit.to_string();

    while !src.is_empty() && src.chars().next().unwrap().is_ascii_digit() {
        num.push(src.remove(0));
    }

    if !src.is_empty() && src.starts_with('.') {
        num.push(src.remove(0));

        while !src.is_empty() && src.chars().next().unwrap().is_ascii_digit() {
            num.push(src.remove(0));
        }
    }

    return Ok(Token::new(TokenType::Number, num, *line));
}

fn tokenize_identifier(src: &mut String, line: &mut u32, first_char: char) -> Result<Token, i32> {
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("let", TokenType::Let),
        ("while", TokenType::While),
    ]);

    let mut identifier = first_char.to_string();

    while !src.is_empty() && src.chars().next().unwrap().is_alphanumeric() {
        identifier.push(src.remove(0));
    }

    if let Some(token_type) = keywords.get(&*identifier) {
        return Ok(Token::new(*token_type, identifier, *line));
    } else {
        return Ok(Token::new(TokenType::Identifier, identifier, *line));
    }
}

fn error(line: u32, message: &str) {
    eprintln!("[{line}] {message}");
}
