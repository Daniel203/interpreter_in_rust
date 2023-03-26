use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{expr::Expr, interpreter::Interpreter, stmt::Stmt, token::Token};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FunctionType {
    None,
    Function,
}

#[derive(Debug)]
pub struct Resolver {
    interpreter: Rc<RefCell<Interpreter>>,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
}

impl Resolver {
    pub fn new(interpreter: Rc<RefCell<Interpreter>>) -> Self {
        return Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
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
            Stmt::ReturnStmt { keyword: _, value } => {
                if self.current_function == FunctionType::None {
                    return Err("Return statement not allowed outside of a function".to_string());
                }

                if let Some(value) = value {
                    self.resolve_expr(value)?;
                }
            }
            Stmt::WhileStmt { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve(body)?;
            }
        };

        return Ok(());
    }

    pub fn resolve_many(&mut self, stmts: &Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            self.resolve(stmt)?;
        }

        return Ok(());
    }

    fn resolve_block(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_many(&statements.iter().map(|b| b.as_ref()).collect())?;
                self.end_scope();
            }
            _ => panic!("Wrong type"),
        }

        return Ok(());
    }

    fn resolve_var(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::Var { name, initializer } = stmt {
            self.declare(name)?;
            self.resolve_expr(initializer)?;
            self.define(name);
        } else {
            panic!("Wrong type in resolve var");
        }

        return Ok(());
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Variable { id: _, name: _ } => return self.resolve_expr_var(expr, expr.get_id()),
            Expr::Assign {
                id: _,
                name: _,
                value: _,
            } => return self.resolve_expr_assign(expr, expr.get_id()),
            Expr::Binary {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                return self.resolve_expr(right);
            }
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr_var(callee.as_ref(), callee.get_id())?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }

                return Ok(());
            }
            Expr::Grouping { id: _, expression } => return self.resolve_expr(expression),
            Expr::Literal { id: _, value: _ } => return Ok(()),
            Expr::Logical {
                id: _,
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                return self.resolve_expr(right);
            }
            Expr::Unary {
                id: _,
                operator: _,
                right,
            } => return self.resolve_expr(right),
            Expr::AnonFunction {
                id: _,
                paren: _,
                arguments,
                body,
            } => {
                return self
                    .resolve_function_helper(arguments, &body.iter().map(|b| b.as_ref()).collect())
            }
        };
    }

    fn resolve_expr_var(&mut self, expr: &Expr, resolve_id: usize) -> Result<(), String> {
        match expr {
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments: _,
            } => {
                match callee.as_ref() {
                    Expr::Variable { id: _, name } => return self.resolve_local(name, resolve_id),
                    _ => panic!("Wrong type in resolve_expr_var"),
                };
            }
            Expr::Variable { id: _, name } => {
                if !self.scopes.is_empty() {
                    let last_value = self
                        .scopes
                        .last()
                        .expect("Cannot read last element of scopes in resolver")
                        .get(&name.name);

                    if let Some(false) = last_value {
                        return Err("Can't read local variable on its own initializer".to_string());
                    }
                }
                return self.resolve_local(name, resolve_id);
            }
            _ => panic!("Wrong type in resolve_expr_var"),
        };
    }

    fn resolve_local(&mut self, name: &Token, resolve_id: usize) -> Result<(), String> {
        let size = self.scopes.len();
        if size == 0 {
            return Ok(());
        }

        for i in (0..=size - 1).rev() {
            let scope = self
                .scopes
                .get(i)
                .unwrap_or_else(|| panic!("Cannot read from scopes"));

            if scope.contains_key(&name.name) {
                self.interpreter
                    .borrow_mut()
                    .resolve(resolve_id, size - 1 - i)?;
                return Ok(());
            }
        }

        return Ok(());
    }

    fn resolve_function(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::Function { name, params, body } = stmt {
            let enclosing_function = self.current_function;
            self.current_function = FunctionType::Function;
            self.declare(name)?;
            self.define(name);

            self.resolve_function_helper(params, &body.iter().map(|b| b.as_ref()).collect())?;
            self.current_function = enclosing_function;
        } else {
            panic!("Wrong type in resolve function");
        }

        return Ok(());
    }

    fn resolve_function_helper(
        &mut self,
        params: &[Token],
        body: &Vec<&Stmt>,
    ) -> Result<(), String> {
        self.begin_scope();

        for param in params {
            self.declare(param)?;
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

    fn declare(&mut self, name: &Token) -> Result<(), String> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        if let Some(last) = self.scopes.last_mut() {
            if last.contains_key(&name.name) {
                return Err("Variable with this name already declared".to_string());
            }

            last.insert(name.name.clone(), false);
        }

        return Ok(());
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap_or_else(|| panic!("Cannot read last element of scopes in resolver"))
            .insert(name.name.clone(), true);
    }

    fn resolve_expr_assign(&mut self, expr: &Expr, resolve_id: usize) -> Result<(), String> {
        if let Expr::Assign { id: _, name, value } = expr {
            self.resolve_expr(value.as_ref())?;
            self.resolve_local(name, resolve_id)?;
        } else {
            panic!("Wrong type in resolve assign");
        }

        return Ok(());
    }
}
