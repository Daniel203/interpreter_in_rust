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

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.curr;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "", None, self.line));

        return Ok(self.tokens.clone());
    }

    fn is_at_end(self: &Self) -> bool {
        return self.curr >= self.src.len();
    }

    fn scan_token(self: &mut Self) -> Result<(), String> {
        let ch = self.advance();

        if ch.is_some() {
            match ch.unwrap() {
                _ => {}
            }
        } else {
            return Err("Cannot scan token".to_string());
        }

        return Ok(());
    }

    fn advance(self: &mut Self) -> Option<char> {
        let ch = self.src.chars().nth(self.curr);
        self.curr += 1;
        return ch;
    }

    fn add_token(self: &mut Self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.src.get(self.start..self.curr).unwrap_or_default();
        self.tokens.push(Token::new(token_type, &text, literal, self.line))

    }
}
