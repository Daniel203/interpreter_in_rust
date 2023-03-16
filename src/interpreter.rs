use std::rc::Rc;

use crate::{environment::Environment, expr::Literal, stmt::Stmt};

pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            environment: Rc::new(Environment::new()),
        };
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable referenc to environment"),
                    )?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable ref to env"),
                    )?;
                    println!("{value:?}");
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable ref to env"),
                    )?;

                    Rc::get_mut(&mut self.environment)
                        .expect("Could not get mutable ref to env")
                        .define(name.value, value);
                }
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();

                    self.environment = Rc::new(new_environment);
                    self.interpret(statements)?;
                    self.environment = old_environment;
                }
                Stmt::IfStmt {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    let truth_value = condition.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable ref to env"),
                    )?;

                    if truth_value.is_truthy() == Literal::True {
                        self.interpret(vec![*then_branch])?;
                    } else if let Some(else_stmt) = else_branch {
                        self.interpret(vec![*else_stmt])?;
                    }
                }
            };
        }

        return Ok(());
    }
}
