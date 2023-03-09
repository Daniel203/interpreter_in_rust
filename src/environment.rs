use std::collections::HashMap;

use crate::expr::Literal;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Literal>,
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
        };
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<&Literal> {
        return self.values.get(&name);
    }
}
