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

    pub fn expression(&mut self) -> Option<Expr> {
        return self.equality();
    }

    fn equality(&mut self) -> Option<Expr> {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            expr = Some(Expr::Binary {
                left: Box::from(expr.unwrap().clone()),
                operator: self.previous().unwrap().clone(),
                right: Box::from(self.comparison().unwrap().clone()),
            });
        }

        return expr;
    }

    fn previous(&self) -> Option<&Token> {
        return self.tokens.get(self.curr - 1);
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

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.curr += 1;
        }

        return self.previous();
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn comparison(&mut self) -> Option<Expr> {
        let mut expr = self.term();

        while self.match_tokens(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            expr = Some(Expr::Binary {
                left: Box::from(expr.unwrap().clone()),
                operator: self.previous().unwrap().clone(),
                right: Box::from(self.term().unwrap()),
            });
        }

        return expr;
    }

    fn term(&mut self) -> Option<Expr> {
        let mut expr = self.factor();

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            expr = Some(Expr::Binary {
                left: Box::from(expr.unwrap().clone()),
                operator: self.previous().unwrap().clone(),
                right: Box::from(self.unary().unwrap()),
            });
        }

        return expr;
    }

    fn factor(&mut self) -> Option<Expr> {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            expr = Some(Expr::Binary {
                left: Box::from(expr.unwrap().clone()),
                operator: self.previous().unwrap().clone(),
                right: Box::from(self.unary().unwrap()),
            });
        }

        return expr;
    }

    fn unary(&mut self) -> Option<Expr> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            return Some(Expr::Unary {
                operator: self.previous().unwrap().clone(),
                right: Box::from(self.unary().unwrap()),
            });
        }

        return self.primary();
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_tokens(vec![TokenType::False]) {
            return Some(Expr::Literal {
                value: Literal::False,
            });
        }

        if self.match_tokens(vec![TokenType::True]) {
            return Some(Expr::Literal {
                value: Literal::True,
            });
        }

        if self.match_tokens(vec![TokenType::Nil]) {
            return Some(Expr::Literal {
                value: Literal::Nil,
            });
        }

        if self.match_tokens(vec![TokenType::Number, TokenType::String]) {
            return Some(Expr::Literal {
                value: expr::Literal::from_token_literal(
                    self.previous().unwrap().clone().literal.unwrap(),
                ),
            });
        }

        return None;
    }
}
