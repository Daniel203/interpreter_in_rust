use crate::{
    expr::{self, Expr, Literal},
    token::Token,
    token_type::TokenType,
};

pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self { tokens, curr: 0 };
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        return self.expression();
    }

    pub fn expression(&mut self) -> Result<Expr, String> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual])? {
            expr = Ok(Expr::Binary {
                left: Box::from(expr?.clone()),
                operator: self.previous()?.clone(),
                right: Box::from(self.comparison()?.clone()),
            });
        }

        return expr;
    }

    fn previous(&self) -> Result<Token, String> {
        let prev = self.tokens.get(self.curr - 1);

        match prev {
            Some(token) => return Ok(token.clone()),
            None => return Err("Canno find any previous value.".to_string()),
        }
    }

    fn peek(&self) -> Option<&Token> {
        return self.tokens.get(self.curr);
    }

    fn is_at_end(&self) -> bool {
        if let Some(token) = self.peek() {
            return token.token_type == TokenType::EOF;
        }

        return true;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() || self.peek().is_none() {
            return false;
        }

        return self.peek().unwrap().token_type == token_type;
    }

    fn advance(&mut self) -> Result<Token, String> {
        if !self.is_at_end() {
            self.curr += 1;
        }

        return self.previous();
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> Result<bool, String> {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance()?;
                return Ok(true);
            }
        }

        return Ok(false);
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term();

        while self.match_tokens(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])? {
            expr = Ok(Expr::Binary {
                left: Box::from(expr?.clone()),
                operator: self.previous()?.clone(),
                right: Box::from(self.term()?),
            });
        }

        return expr;
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus])? {
            expr = Ok(Expr::Binary {
                left: Box::from(expr?.clone()),
                operator: self.previous()?.clone(),
                right: Box::from(self.unary()?),
            });
        }

        return expr;
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star])? {
            expr = Ok(Expr::Binary {
                left: Box::from(expr?.clone()),
                operator: self.previous()?,
                right: Box::from(self.unary()?),
            });
        }

        return expr;
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus])? {
            return Ok(Expr::Unary {
                operator: self.previous()?,
                right: Box::from(self.unary()?),
            });
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_tokens(vec![TokenType::False])? {
            return Ok(Expr::Literal {
                value: Literal::False,
            });
        }

        if self.match_tokens(vec![TokenType::True])? {
            return Ok(Expr::Literal {
                value: Literal::True,
            });
        }

        if self.match_tokens(vec![TokenType::Nil])? {
            return Ok(Expr::Literal {
                value: Literal::Nil,
            });
        }

        if self.match_tokens(vec![TokenType::Number, TokenType::String])? {
            return Ok(Expr::Literal {
                value: expr::Literal::from_token_literal(self.previous().unwrap().literal.unwrap()),
            });
        }

        if self.match_tokens(vec![TokenType::LeftParen])? {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping {
                expression: Box::from(expr),
            });
        }

        return Err("Expected expression".to_string());
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), String> {
        let token = self.peek();

        if token.is_some() && token.unwrap().token_type == token_type {
            self.advance()?;
            return Ok(());
        } else {
            return Err(msg.to_string());
        }
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) -> Result<(), String> {
        self.advance()?;

        while !self.is_at_end() {
            if self.previous()?.token_type == TokenType::Semicolon {
                return Ok(());
            }

            let token_type = match self.peek() {
                Some(token) => token.token_type,
                None => return Err("Cannot find a token.".to_string()),
            };

            match token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Let
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return Ok(()),
                _ => (),
            }

            self.advance()?;
        }

        return Ok(());
    }
}
