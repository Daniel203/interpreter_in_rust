use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::expr::CallableImpl;
use crate::expr::{Literal, NativeFunctionImpl};

#[derive(Debug, Clone)]
pub struct Environment {
    values: Rc<RefCell<HashMap<String, Literal>>>,
    locals: Rc<RefCell<HashMap<usize, usize>>>,
    pub enclosing: Option<Box<Environment>>,
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

    let callable_impl = NativeFunctionImpl {
        name: name.clone(),
        arity: 0,
        fun: Rc::new(clock_impl),
    };

    env.insert(
        name,
        Literal::Callable(CallableImpl::NativeFunction(callable_impl)),
    );

    return env;
}

impl Environment {
    pub fn new(locals: HashMap<usize, usize>) -> Self {
        return Self {
            values: Rc::new(RefCell::new(get_globals())),
            locals: Rc::new(RefCell::new(locals)),
            enclosing: None,
        };
    }

    pub fn resolve(&mut self, locals: HashMap<usize, usize>) {
        for (k, v) in locals.iter() {
            self.locals.borrow_mut().insert(*k, *v);
        }
    }

    pub fn enclose(&self) -> Environment {
        return Self {
            values: Rc::new(RefCell::new(HashMap::new())),
            locals: self.locals.clone(),
            enclosing: Some(Box::new(self.clone())),
        };
    }

    pub fn define(&self, name: String, value: Literal) {
        self.values.borrow_mut().insert(name, value);
    }

    pub fn get(&self, name: &str, expr_id: usize) -> Option<Literal> {
        let distance = self.locals.borrow().get(&expr_id).cloned();
        return self.get_internal(name, distance);
    }

    pub fn get_distance(&self, expr_id: usize) -> Option<usize> {
        return self.locals.borrow().get(&expr_id).cloned();
    }

    fn get_internal(&self, name: &str, distance: Option<usize>) -> Option<Literal> {
        if let Some(distance) = distance {
            if distance == 0 {
                return self.values.borrow().get(name).cloned();
            } else {
                match &self.enclosing {
                    Some(env) => env.get_internal(name, Some(distance -1)),
                    None => panic!("Tried to resolve a variable that was defined deeper than the current environment depth"),
                }
            }
        } else {
            return match &self.enclosing {
                Some(env) => env.get_internal(name, distance),
                None => self.values.borrow().get(name).cloned(),
            };
        }
    }

    pub fn assign_global(&self, name: &str, value: Literal) -> bool {
        return self.assign_internal(name, value, None);
    }

    pub fn assign(&self, name: &str, value: Literal, expr_id: usize) -> bool {
        let distance = self.locals.borrow().get(&expr_id).cloned();
        return self.assign_internal(name, value, distance);
    }

    fn assign_internal(&self, name: &str, value: Literal, distance: Option<usize>) -> bool {
        if let Some(distance) = distance {
            if distance == 0 {
                self.values.borrow_mut().insert(name.to_string(), value);
                return true;
            } else {
                match &self.enclosing {
                    Some(env) => env.assign_internal(name, value, Some(distance -1)),
                    None => panic!("Tried to resolve a variable that was defined deeper than the current environment depth"),
                };
                return true;
            }
        } else {
            match &self.enclosing {
                Some(env) => return env.assign_internal(name, value, None),
                None => match self.values.borrow_mut().insert(name.to_string(), value) {
                    Some(_) => return true,
                    None => return false,
                },
            };
        }
    }
}
