use std::collections::HashMap;

use crate::{expr::Expr, interpreter::Interpreter, stmt::Stmt, token::Token};

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        return Self {
            interpreter: interpreter,
            scopes: Vec::new(),
        };
    }

    pub fn resolve(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block { statements: _ } => self.resolve_block(stmt)?,
            Stmt::Var {
                name: _,
                initializer: _,
            } => self.resolve_var(stmt)?,
            Stmt::Function {
                name: _,
                params: _,
                body: _,
            } => self.resolve_function(stmt)?,
            Stmt::Expression { expression } => self.resolve_expr(expression)?,
            Stmt::IfStmt {
                condition: _,
                then_branch: _,
                else_branch: _,
            } => self.resolve_if_stmt(stmt)?,
            Stmt::Print { expression } => self.resolve_expr(expression)?,
            Stmt::ReturnStmt {
                keyword: _,
                value: None,
            } => (),
            Stmt::ReturnStmt {
                keyword: _,
                value: Some(value),
            } => self.resolve_expr(value)?,
            Stmt::WhileStmt { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve(body)?;
            }
        };

        return Ok(());
    }

    fn resolve_many(&mut self, stmts: &[Box<Stmt>]) -> Result<(), String> {
        for stmt in stmts {
            self.resolve(stmt.as_ref())?;
        }

        return Ok(());
    }

    fn resolve_block(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_many(statements)?;
                self.end_scope();
            }
            _ => panic!("Wrong type"),
        }

        return Ok(());
    }

    fn resolve_var(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::Var { name, initializer } = stmt {
            self.declare(name);
            self.resolve_expr(initializer)?;
            self.define(name);
        } else {
            panic!("Wrong type in resolve var");
        }

        return Ok(());
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Variable { name: _ } => return self.resolve_expr_var(expr),
            Expr::Assign { name: _, value: _ } => return self.resolve_expr_assign(expr),
            Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(callee)?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }
            }
            Expr::Grouping { expression } => self.resolve_expr(expression)?,
            Expr::Literal { value: _ } => (),
            Expr::Logical {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
            }
            Expr::Unary { operator: _, right } => self.resolve_expr(right)?,
            Expr::AnonFunction {
                paren: _,
                arguments,
                body,
            } => self.resolve_function_helper(arguments, body)?,
        };

        return Ok(());
    }

    fn resolve_expr_var(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::Variable { name } = expr {
            let last_value = self
                .scopes
                .last()
                .expect("Cannot read last element of scopes in resolver")
                .get(&name.value);

            if !self.scopes.is_empty() && last_value == Some(&false) {
                return Err("Can't read local variable on its own initializer".to_string());
            }

            return self.resolve_local(name);
        } else {
            panic!("Wrong type in resolve_expr_var");
        }
    }

    fn resolve_local(&mut self, name: &Token) -> Result<(), String> {
        let size = self.scopes.len() - 1;

        for i in (0..size).rev() {
            let scope = self
                .scopes
                .get(i)
                .unwrap_or_else(|| panic!("Cannot read from scopes"));

            if scope.contains_key(&name.value) {
                //self.interpreter.resolve(expr, size - 1 - i)?;
                return Ok(());
            }
        }

        return Ok(());
    }

    fn resolve_function(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::Function { name, params, body } = stmt {
            self.declare(name);
            self.define(name);

            self.resolve_function_helper(params, body)?;
        } else {
            panic!("Wrong type in resolve function");
        }

        return Ok(());
    }

    fn resolve_function_helper(
        &mut self,
        params: &[Token],
        body: &[Box<Stmt>],
    ) -> Result<(), String> {
        self.begin_scope();
        for param in params {
            self.declare(param);
            self.define(param);
        }
        self.resolve_many(body)?;
        self.end_scope();

        return Ok(());
    }

    fn resolve_if_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::IfStmt {
            condition,
            then_branch,
            else_branch,
        } = stmt
        {
            self.resolve_expr(condition)?;
            self.resolve(then_branch)?;

            if let Some(else_branch) = else_branch {
                self.resolve(else_branch)?;
            }
        } else {
            panic!("Wrong type in resolve if stmt");
        }
        return Ok(());
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop().expect("Stack underflow");
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap_or_else(|| panic!("Cannot read last element of scopes in resolver"))
            .insert(name.value.clone(), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap_or_else(|| panic!("Cannot read last element of scopes in resolver"))
            .insert(name.value.clone(), true);
    }

    fn resolve_expr_assign(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::Assign { name, value } = expr {
            self.resolve_expr(value.as_ref())?;
            self.resolve_local(name)?;
        } else {
            panic!("Wrong type in resolve assign");
        }

        return Ok(());
    }
}
