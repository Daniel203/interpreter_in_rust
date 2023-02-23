use crate::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub value: String,
    pub line: u32,
}

impl Token {
    pub fn new(token: TokenType, value: String, line: u32) -> Self {
        return Self { token, value, line };
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        return String::from("{type} {value}");
    }
}
