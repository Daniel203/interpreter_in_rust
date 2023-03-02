use crate::{expr::Expr, token::Token, token_type::TokenType};

struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, curr: 0 };
    }

    fn expression(&self) -> Expr {
        return self.equality();
    }

    fn equality(&self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::from(expr),
                operator: operator.expect("No operator found.").clone(),
                right: Box::from(right),
            }
        }

        return expr;
    }

    fn comparison(&self) -> Expr {
        todo!()
    }

    fn previous(&self) -> Option<&Token> {
        return self.tokens.get(self.curr - 1);
    }

    fn peek(&self) -> Option<&Token> {
        return self.tokens.get(self.curr);
    }

    fn is_at_end(&self) -> Result<bool, String> {
        if let Some(token) = self.peek() {
            return Ok(token.token == TokenType::EOF);
        }

        return Err("Cannot check if parser is at end".to_string());
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return true;
        //return self.peek()
    }

    fn advance(&self) {
        todo!()
    }

    fn match_tokens(&self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }
}
