use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, expr::Literal, stmt::Stmt};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        };
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(self.environment.clone())?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    println!("{value:?}");
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(self.environment.clone())?;

                    self.environment
                        .borrow_mut()
                        .define(name.value.clone(), value);
                }
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();

                    self.environment = Rc::new(RefCell::new(new_environment));
                    self.interpret((*statements).iter().map(|b| b.as_ref()).collect())?;
                    self.environment = old_environment;
                }
                Stmt::IfStmt {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    let truth_value = condition.evaluate(self.environment.clone())?;

                    if truth_value.is_truthy() == Literal::True {
                        self.interpret(vec![then_branch.as_ref()])?;
                    } else if let Some(else_stmt) = else_branch {
                        self.interpret(vec![else_stmt.as_ref()])?;
                    }
                }
                Stmt::WhileStmt { condition, body } => {
                    let mut flag = condition.evaluate(self.environment.clone())?;

                    while flag.is_truthy() == Literal::True {
                        self.interpret(vec![body.as_ref()])?;

                        flag = condition.evaluate(self.environment.clone())?;
                    }
                }
            };
        }

        return Ok(());
    }
}
