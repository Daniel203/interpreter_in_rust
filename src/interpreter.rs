use crate::{environment::Environment, stmt::Stmt};

pub struct Interpreter {
    environment: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            environment: Environment::new(),
        };
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(&mut self.environment)?;
                }
                Stmt::Print { expression } => {
                    let value = expression.evaluate(&mut self.environment)?;
                    println!("{value:?}");
                }
                Stmt::Var { name, initializer } => {
                    let value = initializer.evaluate(&mut self.environment)?;
                    self.environment.define(name.value, value);
                }
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(Box::from(self.environment.clone()));

                    let old_environment = self.environment.clone();
                    self.environment = new_environment;
                    self.interpret(statements)?;
                    self.environment = old_environment;

                    return Ok(());
                }
            };
        }

        return Ok(());
    }
}
