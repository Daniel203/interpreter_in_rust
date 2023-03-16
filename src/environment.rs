use std::{collections::HashMap, rc::Rc};

use crate::expr::Literal;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    pub enclosing: Option<Rc<Environment>>,
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

    pub fn get(&self, name: String) -> Option<&Literal> {
        let value = self.values.get(&name);

        return match (value, &self.enclosing) {
            (Some(val), _) => Some(val),
            (None, Some(env)) => env.get(name),
            (None, None) => None,
        };
    }

    pub fn assign(&mut self, name: &str, value: Literal) -> bool {
        let old_value = self.values.get(name);

        match (old_value, &mut self.enclosing) {
            (Some(_), _) => {
                self.values.insert(name.to_string(), value);
                return true;
            }
            (None, Some(env)) => {
                return Rc::get_mut(&mut env.clone())
                    .expect("Could not get mutable ref to env")
                    .assign(name, value);
            }
            (None, None) => return false,
        };
    }
}
