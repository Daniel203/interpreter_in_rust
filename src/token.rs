use crate::token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub name: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(token: TokenType, value: &str, literal: Option<Literal>, line: usize) -> Self {
        return Self {
            token_type: token,
            name: value.to_string(),
            literal,
            line,
        };
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        return format!("{:?} '{}'", self.token_type, self.name);
    }
}
