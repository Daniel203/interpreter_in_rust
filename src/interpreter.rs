use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    environment::Environment,
    expr::{Expr, Literal},
    stmt::Stmt,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub specials: Rc<RefCell<HashMap<String, Literal>>>,
    pub environment: Rc<RefCell<Environment>>,
    pub locals: Rc<RefCell<HashMap<usize, usize>>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            specials: Rc::new(RefCell::new(HashMap::new())),
            environment: Rc::new(RefCell::new(Environment::new())),
            locals: Rc::new(RefCell::new(HashMap::new())),
        };
    }

    fn for_closure(parent: Rc<RefCell<Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);

        return Self {
            specials: Rc::new(RefCell::new(HashMap::new())),
            environment,
            locals: Rc::new(RefCell::new(HashMap::new())),
        };
    }

    pub fn for_anon(parent: Rc<RefCell<Environment>>) -> Self {
        let mut env = Environment::new();
        env.enclosing = Some(parent);

        return Self {
            specials: Rc::new(RefCell::new(HashMap::new())),
            environment: Rc::new(RefCell::new(env)),
            locals: Rc::new(RefCell::new(HashMap::new())),
        };
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    let distance = self.get_distance(expression);
                    expression.evaluate(self.environment.clone(), distance)?;
                }
                Stmt::Print { expression } => {
                    let distance = self.get_distance(expression);
                    let value = expression.evaluate(self.environment.clone(), distance)?;
                    println!("{}", value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let distance = self.get_distance(initializer);
                    let value = initializer.evaluate(self.environment.clone(), distance)?;

                    self.environment
                        .borrow_mut()
                        .define(name.value.to_string(), value);
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
                    let distance = self.get_distance(condition);
                    let truth_value = condition.evaluate(self.environment.clone(), distance)?;

                    if truth_value.is_truthy() == Literal::True {
                        self.interpret(vec![then_branch.as_ref()])?;
                    } else if let Some(else_stmt) = else_branch {
                        self.interpret(vec![else_stmt.as_ref()])?;
                    }
                }
                Stmt::WhileStmt { condition, body } => {
                    let distance = self.get_distance(condition);
                    let mut flag = condition.evaluate(self.environment.clone(), distance)?;

                    while flag.is_truthy() == Literal::True {
                        self.interpret(vec![body.as_ref()])?;
                        flag = condition.evaluate(self.environment.clone(), distance)?;
                    }
                }
                Stmt::Function { name, params, body } => {
                    let arity = params.len();

                    let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
                    let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
                    let name_clone = name.value.clone();

                    let parent_env = self.environment.clone();
                    let fun_impl = move |args: &[Literal]| {
                        let mut clos_int = Interpreter::for_closure(parent_env.clone());

                        for (i, arg) in args.iter().enumerate() {
                            clos_int.environment.borrow_mut().define(
                                params
                                    .get(i)
                                    .expect("Cannot read function param")
                                    .value
                                    .clone(),
                                (*arg).clone(),
                            );
                        }

                        for i in 0..body.len() {
                            clos_int
                                .interpret(vec![body
                                    .get(i)
                                    .unwrap_or_else(|| panic!("Element in position {i} not found"))
                                    .as_ref()])
                                .unwrap_or_else(|_| {
                                    panic!("Evaluating failed inside {name_clone}")
                                });

                            if let Some(value) = clos_int.specials.borrow_mut().get("return") {
                                return value.clone();
                            }
                        }

                        return Literal::Nil;
                    };

                    let callable = Literal::Callable {
                        name: name.value.clone(),
                        arity,
                        fun: Rc::new(fun_impl),
                    };

                    self.environment
                        .borrow_mut()
                        .define(name.value.clone(), callable);
                }
                Stmt::ReturnStmt { keyword: _, value } => {
                    let eval_value = if let Some(value) = value {
                        let distance = self.get_distance(value);
                        value.evaluate(self.environment.clone(), distance)?
                    } else {
                        Literal::Nil
                    };

                    self.specials
                        .borrow_mut()
                        .insert("return".to_string(), eval_value);
                }
            };
        }

        return Ok(());
    }

    pub fn resolve(&mut self, expr: &Expr, steps: usize) -> Result<(), String> {
        let addr = std::ptr::addr_of!(expr) as usize;
        self.locals.borrow_mut().insert(addr, steps);

        return Ok(());
    }

    fn get_distance(&self, expr: &Expr) -> Option<usize> {
        let addr = std::ptr::addr_of!(expr) as usize;
        return self.locals.borrow().get(&addr).copied();
    }
}
