use crate::{
    expr::{Expr, Literal},
    stmt::Stmt,
    token::Token,
    token_type::TokenType,
};
use std::str;

#[derive(Debug)]
enum FunctionKind {
    Function,
    Method,
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
    next_id: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        return Self {
            tokens,
            curr: 0,
            next_id: 0,
        };
    }

    fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        return id;
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        let mut errs = Vec::new();

        while !self.is_at_end() {
            let stmt = self.declaration();

            match stmt {
                Ok(s) => stmts.push(s),
                Err(err) => {
                    errs.push(err);
                    self.synchronize()?;
                }
            }
        }

        if errs.is_empty() {
            return Ok(stmts);
        } else {
            return Err(errs.join("\n"));
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(TokenType::Var)? {
            return self.var_declaration();
        } else if self.match_token(TokenType::Class)? {
            return self.class_declaration();
        } else if self.match_token(TokenType::Fun)? {
            return self.function(FunctionKind::Function);
        } else {
            return self.statement();
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let initializer = if self.match_token(TokenType::Equal)? {
            self.expression()?
        } else {
            Expr::Literal {
                id: self.get_id(),
                value: Literal::Nil,
            }
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        return Ok(Stmt::Var { name, initializer });
    }

    fn class_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, "Expected name after 'class' keyword")?;

        let superclass = if self.match_token(TokenType::Colon)? {
            self.consume(TokenType::Identifier, "Expected superclass name after ':'")?;
            Some(Expr::Variable {
                id: self.get_id(),
                name: self.previous()?,
            })
        } else {
            None
        };

        self.consume(TokenType::LeftBrace, "Expected '{{' before class body")?;

        let mut methods = Vec::new();
        while !self.check(TokenType::RightBrace) {
            let method = self.function(FunctionKind::Method)?;
            methods.push(Box::new(method));
        }

        self.consume(TokenType::RightBrace, "Expected '}}' after class body")?;

        return Ok(Stmt::Class {
            name,
            methods,
            superclass,
        });
    }

    fn function(&mut self, kind: FunctionKind) -> Result<Stmt, String> {
        let name = self.consume(TokenType::Identifier, &format!("Expected {kind:?} name"))?;

        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {kind:?} name"),
        )?;

        let mut params = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                let location = self.peek().unwrap().line;
                if params.len() >= 255 {
                    return Err(format!(
                        "Line {location}: Can't have more than 255 parameters"
                    ));
                }

                let param = self.consume(TokenType::Identifier, "Expected parameter name")?;
                params.push(param);

                if !self.match_token(TokenType::Comma)? {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expcected '{{' before {kind:?} name"),
        )?;

        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Block statement parsed something that was not a block"),
        };

        return Ok(Stmt::Function { name, params, body });
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(TokenType::Print)? {
            return self.print_statement();
        } else if self.match_token(TokenType::LeftBrace)? {
            return self.block_statement();
        } else if self.match_token(TokenType::If)? {
            return self.if_statement();
        } else if self.match_token(TokenType::While)? {
            return self.while_statement();
        } else if self.match_token(TokenType::For)? {
            return self.for_statement();
        } else if self.match_token(TokenType::Return)? {
            return self.return_statement();
        } else {
            return self.expression_statement();
        }
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'print'")?;

        let expr = self.expression()?;

        let mut arguments = vec![];
        if self.match_token(TokenType::Comma)? && !self.check(TokenType::RightParen) {
            loop {
                let variable = self.expression()?;
                arguments.push(variable);

                if !self.match_token(TokenType::Comma)? {
                    break;
                }
            }
        }

        self.consume(TokenType::RightParen, "Expected ')' after value")?;
        self.consume(TokenType::Semicolon, "Expected ';' after value")?;

        // Control the number of parameters in the print statement
        // A parameter is in this format: print("{}", a)
        let expected_parameters = expr.to_string().matches("{}").count();
        let found_parameters = arguments.len();

        if expected_parameters != found_parameters {
            return Err(format!(
                "Invalid parameters length: {} expected, {} found",
                expected_parameters, found_parameters
            ));
        }

        return Ok(Stmt::Print {
            expression: expr,
            arguments,
        });
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;

        return Ok(Stmt::Expression { expression: expr });
    }

    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(Box::new(self.declaration()?));
        }

        self.consume(TokenType::RightBrace, "Expected '}' after a block")?;
        return Ok(Stmt::Block { statements });
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected ')' after 'if'")?;
        let condition = self.expression()?;

        self.consume(TokenType::RightParen, "Expected ')' after 'if-condition'")?;
        let then_branch = Box::from(self.statement()?);

        let else_branch = if self.match_token(TokenType::Else)? {
            let stmt = self.statement()?;
            Some(Box::from(stmt))
        } else {
            None
        };

        return Ok(Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        });
    }

    fn while_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected ')' after 'while'")?;
        let condition = self.expression()?;

        self.consume(
            TokenType::RightParen,
            "Expected ')' after 'while-condition'",
        )?;
        let body = Box::from(self.statement()?);

        return Ok(Stmt::WhileStmt { condition, body });
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected ')' after 'while'")?;

        let initializer = if self.match_token(TokenType::Semicolon)? {
            None
        } else if self.match_token(TokenType::Var)? {
            let var_decl = self.var_declaration()?;
            Some(var_decl)
        } else {
            let expr = self.expression_statement()?;
            Some(expr)
        };

        let condition = if !self.check(TokenType::Semicolon) {
            let expr = self.expression()?;
            Some(expr)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition.")?;

        let increment = if !self.check(TokenType::RightParen) {
            let expr = self.expression()?;
            Some(expr)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expected ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(incr) = increment {
            body = Stmt::Block {
                statements: vec![
                    Box::new(body),
                    Box::new(Stmt::Expression { expression: incr }),
                ],
            }
        }

        let cond = match condition {
            Some(c) => c,
            None => Expr::Literal {
                id: self.get_id(),
                value: Literal::True,
            },
        };

        body = Stmt::WhileStmt {
            condition: cond,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = Stmt::Block {
                statements: vec![Box::new(init), Box::new(body)],
            }
        }

        return Ok(body);
    }

    pub fn return_statement(&mut self) -> Result<Stmt, String> {
        let keyword = self.previous()?;

        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expected ';' after return value.")?;

        return Ok(Stmt::ReturnStmt { keyword, value });
    }

    pub fn expression(&mut self) -> Result<Expr, String> {
        return self.assignment();
    }

    pub fn function_expression(&mut self) -> Result<Expr, String> {
        let paren = self.consume(
            TokenType::LeftParen,
            "Expected '(' after anonymous function",
        )?;

        let mut arguments = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                let location = self.peek().unwrap().line;
                if arguments.len() >= 255 {
                    return Err(format!(
                        "Line {location}: Can't have more than 255 parameters"
                    ));
                }

                let param = self.consume(TokenType::Identifier, "Expected parameter name")?;
                arguments.push(param);

                if !self.match_token(TokenType::Comma)? {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParen,
            "Expected ')' after anonymous function",
        )?;
        self.consume(
            TokenType::LeftBrace,
            "Expected '{{' after anonymous function",
        )?;

        let body = match self.block_statement()? {
            Stmt::Block { statements } => statements,
            _ => panic!("Block statement parsed something that was not a block"),
        };

        return Ok(Expr::AnonFunction {
            id: self.get_id(),
            paren,
            arguments,
            body,
        });
    }

    pub fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.or()?;

        if self.match_token(TokenType::Equal)? {
            let equals = self.previous()?;
            let value = self.expression()?;

            match expr {
                Expr::Variable { id: _, name } => {
                    return Ok(Expr::Assign {
                        id: self.get_id(),
                        name,
                        value: Box::from(value),
                    });
                }
                Expr::Get {
                    id: _,
                    object,
                    name,
                } => {
                    return Ok(Expr::Set {
                        id: self.get_id(),
                        object,
                        name,
                        value: Box::new(value),
                    });
                }
                _ => {
                    return Err(format!(
                        "Invalid assignment target: '{}'.",
                        equals.to_string()
                    ));
                }
            };
        }

        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr = self.and()?;

        while self.match_token(TokenType::Or)? {
            let operator = self.previous()?;
            let right = self.and()?;
            expr = Expr::Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.match_token(TokenType::And)? {
            let operator = self.previous()?;
            let right = self.equality()?;
            expr = Expr::Logical {
                id: self.get_id(),
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        return Ok(expr);
    }

    fn previous(&self) -> Result<Token, String> {
        let mut prev = None;

        if self.curr >= 1 {
            prev = self.tokens.get(self.curr - 1);
        }

        match prev {
            Some(token) => return Ok(token.clone()),
            None => return Err("Cannot find any previous value.".to_string()),
        }
    }

    fn peek(&mut self) -> Option<Token> {
        return self.tokens.get(self.curr).cloned();
    }

    fn is_at_end(&mut self) -> bool {
        if let Some(token) = self.peek() {
            return token.token_type == TokenType::EOF;
        }

        return true;
    }

    fn check(&mut self, token_type: TokenType) -> bool {
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
            if self.match_token(token_type)? {
                return Ok(true);
            }
        }

        return Ok(false);
    }

    fn match_token(&mut self, token_type: TokenType) -> Result<bool, String> {
        if self.check(token_type) {
            self.advance()?;
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison();

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual])? {
            expr = Ok(Expr::Binary {
                id: self.get_id(),
                left: Box::from(expr?.clone()),
                operator: self.previous()?.clone(),
                right: Box::from(self.comparison()?.clone()),
            });
        }

        return expr;
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
                id: self.get_id(),
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
                id: self.get_id(),
                left: Box::from(expr?.clone()),
                operator: self.previous()?.clone(),
                right: Box::from(self.factor()?),
            });
        }

        return expr;
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary();

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star])? {
            expr = Ok(Expr::Binary {
                id: self.get_id(),
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
                id: self.get_id(),
                operator: self.previous()?,
                right: Box::from(self.unary()?),
            });
        }

        return self.call();
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(TokenType::LeftParen)? {
                expr = self.finish_call(expr)?;
            } else if self.match_token(TokenType::Dot)? {
                let name =
                    self.consume(TokenType::Identifier, "Expected proprety name after '.'")?;
                expr = Expr::Get {
                    id: self.get_id(),
                    object: Box::new(expr),
                    name,
                };
            } else {
                break;
            }
        }

        return Ok(expr);
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);

                if arguments.len() >= 255 {
                    let location = self.peek().unwrap().line;
                    return Err(format!(
                        "Line {location}: Can't have more than 255 arguments"
                    ));
                }

                if !self.match_token(TokenType::Comma)? {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments.")?;

        return Ok(Expr::Call {
            id: self.get_id(),
            callee: Box::from(callee),
            paren,
            arguments,
        });
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if let Some(token) = self.peek() {
            let result = match token.token_type {
                TokenType::LeftParen => {
                    self.advance()?;
                    let expr = self.expression()?;
                    self.consume(TokenType::RightParen, "Expected ')'")?;
                    Expr::Grouping {
                        id: self.get_id(),
                        expression: Box::from(expr),
                    }
                }
                TokenType::False
                | TokenType::True
                | TokenType::Nil
                | TokenType::Number
                | TokenType::String => {
                    self.advance()?;
                    let token = self.previous()?;
                    Expr::Literal {
                        id: self.get_id(),
                        value: Literal::from_token(token),
                    }
                }
                TokenType::Identifier => {
                    self.advance()?;
                    Expr::Variable {
                        id: self.get_id(),
                        name: self.previous()?,
                    }
                }
                TokenType::Fun => {
                    self.advance()?;
                    self.function_expression()?
                }
                TokenType::This => {
                    self.advance()?;

                    Expr::This {
                        id: self.get_id(),
                        keyword: token,
                    }
                }
                TokenType::Super => {
                    self.advance()?;
                    self.consume(TokenType::Dot, "Expected '.' after 'super'")?;
                    let method =
                        self.consume(TokenType::Identifier, "Expected superclass method name")?;

                    Expr::Super {
                        id: self.get_id(),
                        keyword: token,
                        method,
                    }
                }

                _ => return Err("Expected expression.".to_string()),
            };

            return Ok(result);
        } else {
            return Err("Expected expression.".to_string());
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String> {
        if let Some(token) = self.peek() {
            if token.token_type == token_type {
                self.advance()?;
                let token = self.previous()?;
                return Ok(token);
            } else {
                return Err(format!("Line {}: {}", token.line, msg));
            }
        } else {
            return Err(msg.to_string());
        }
    }

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
                | TokenType::Var
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
