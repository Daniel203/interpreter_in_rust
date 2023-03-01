use std::collections::HashMap;

use crate::{
    token::{Literal, Token},
    token_type::TokenType,
};

pub struct Lexer {
    src: String,
    tokens: Vec<Token>,
    start: usize,
    curr: usize,
    line: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        return Self {
            src: src.to_string(),
            tokens: vec![],
            start: 0,
            curr: 0,
            line: 1,
        };
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", None, self.line));

        return Ok(self.tokens.clone());
    }

    fn is_at_end(&self) -> bool {
        return self.curr >= self.src.len();
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let ch = self.advance();

        if ch.is_none() {
            return Err("No character found.".to_string());
        }

        match ch.unwrap() {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightParen, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' if self.char_match('=') => self.add_token(TokenType::BangEqual, None),
            '!' => self.add_token(TokenType::Bang, None),
            '=' if self.char_match('=') => self.add_token(TokenType::EqualEqual, None),
            '=' => self.add_token(TokenType::Equal, None),
            '<' if self.char_match('=') => self.add_token(TokenType::LessEqual, None),
            '<' => self.add_token(TokenType::Less, None),
            '>' if self.char_match('=') => self.add_token(TokenType::GreaterEqual, None),
            '>' => self.add_token(TokenType::Greater, None),
            '/' if self.char_match('/') => {
                while !self.is_at_end() && self.peek() != Some('\n') {
                    self.advance();
                }
            }
            '/' => self.add_token(TokenType::Slash, None),
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => {}
            '"' => self.string()?,
            '0'..='9' => self.number()?,
            'a'..='z' | 'A'..='Z' | '_' => self.identifier()?,

            _ => {
                return Err(format!(
                    "Line {}: Unrecognized char {}",
                    self.line,
                    ch.unwrap()
                ))
            }
        }

        return Ok(());
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek();
        self.curr += 1;
        return ch;
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.src.get(self.start..self.curr).unwrap_or_default();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line))
    }

    fn char_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != Some(expected) {
            return false;
        }

        self.curr += 1;
        return true;
    }

    fn peek(&self) -> Option<char> {
        return self.src.chars().nth(self.curr);
    }

    fn peek_next(&self) -> Option<char> {
        return self.src.chars().nth(self.curr + 1);
    }

    fn is_alphanumeric(&self, ch: char) -> bool {
        return ch.is_ascii_digit()
            || ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ch == '_';
    }

    fn string(&mut self) -> Result<(), String> {
        while !self.is_at_end() && self.peek() != Some('"') {
            if self.peek() == Some('\n') {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(format!("Line {}: Unterminated string.", self.line));
        }

        // parse the closing "
        self.advance();

        let value = self
            .src
            .get(self.start + 1..self.curr - 1)
            .unwrap_or_default();
        let literal = Literal::String(value.to_string());

        self.add_token(TokenType::String, Some(literal));

        return Ok(());
    }

    fn number(&mut self) -> Result<(), String> {
        while self.peek().unwrap_or_default().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == Some('.') && self.peek_next().unwrap_or_default().is_ascii_digit() {
            self.advance();

            while self.peek().unwrap_or_default().is_ascii_digit() {
                self.advance();
            }
        }

        let value = self.src.get(self.start..self.curr).unwrap_or_default();
        let literal = Literal::Number(value.parse().expect("Invalid number format."));
        self.add_token(TokenType::Number, Some(literal));

        return Ok(());
    }

    fn identifier(&mut self) -> Result<(), String> {
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

        while self.is_alphanumeric(self.peek().unwrap_or_default()) {
            self.advance();
        }

        let value = self.src.get(self.start..self.curr).unwrap_or_default();
        let token_type = keywords.get(value).unwrap_or(&TokenType::Identifier);
        self.add_token(*token_type, None);
        return Ok(());
    }
}
