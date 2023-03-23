use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::expr::Literal;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Literal>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

fn clock_impl(_args: &[Literal]) -> Literal {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    return Literal::Number(now as f64 / 1000.0);
}

fn get_globals() -> HashMap<String, Literal> {
    let mut env = HashMap::new();
    let name = "clock".to_string();

    env.insert(
        name.clone(),
        Literal::Callable {
            name,
            arity: 0,
            fun: Rc::new(clock_impl),
        },
    );

    return env;
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            values: get_globals(),
            enclosing: None,
        };
    }

    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str, distance: Option<usize>) -> Option<Literal> {
        if let Some(distance) = distance {
            if distance == 0 {
                return self.values.get(name).cloned();
            } else {
                match &self.enclosing {
                    Some(env) => env.borrow().get(name, Some(distance -1)),
                    None => panic!("Tried to resolve a variable that was defined deeper than the current environment depth"),
                }
            }
        } else {
            return match &self.enclosing {
                Some(env) => env.borrow().get(name, None),
                None => self.values.get(name).cloned(),
            };
        }
    }

    pub fn assign(&mut self, name: &str, value: Literal, distance: Option<usize>) -> bool {
        if let Some(distance) = distance {
            if distance == 0 {
                self.values.insert(name.to_string(), value);
                return true;
            } else {
                match &self.enclosing {
                    Some(env) => env.borrow_mut().assign(name, value, Some(distance -1)),
                    None => panic!("Tried to resolve a variable that was defined deeper than the current environment depth"),
                };
                return true;
            }
        } else {
            match &self.enclosing {
                Some(env) => env.borrow_mut().assign(name, value, None),
                None => false,
            };
            return true;
        }
    }
}
