use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::expr::Literal;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            values: HashMap::new(),
            enclosing: None,
        };
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn define_top_level(&mut self, name: String, value: Literal) {
        match &self.enclosing {
            Some(env) => env.borrow_mut().define_top_level(name, value),
            None => self.define(name, value),
        }
    }

    pub fn get(&self, name: &str) -> Option<Literal> {
        let value = self.values.get(name);

        return match (value, &self.enclosing) {
            (Some(val), _) => Some(val.clone()),
            (None, Some(env)) => env.borrow().get(name),
            (None, None) => None,
        };
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> bool {
        let old_value = self.values.get(name);

        match (old_value, &self.enclosing) {
            (Some(_), _) => {
                self.values.insert(name.to_string(), value);
                return true;
            }
            (None, Some(env)) => {
                return env.borrow_mut().assign(name, value);
            }
            (None, None) => return false,
        };
    }
}
