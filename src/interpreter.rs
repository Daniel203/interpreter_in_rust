use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, expr::Literal, stmt::Stmt, token::Token};

#[derive(Debug)]
pub struct Interpreter {
    pub specials: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

fn clock_impl(_args: &[Literal]) -> Literal {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    return Literal::Number(now as f64 / 1000.0);
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let mut globals = Environment::new();
        let name = "clock".to_string();

        globals.define(
            name.clone(),
            Literal::Callable {
                name,
                arity: 0,
                fun: Rc::new(clock_impl),
            },
        );

        return Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(globals)),
        };
    }

    fn for_closure(parent: Rc<RefCell<Environment>>) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);

        return Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment,
        };
    }

    pub fn for_anon(parent: Rc<RefCell<Environment>>) -> Self {
        let mut env = Environment::new();
        env.enclosing = Some(parent);

        return Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(env)),
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
                    println!("{}", value.to_string());
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
                                return value;
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
                        value.evaluate(self.environment.clone())?
                    } else {
                        Literal::Nil
                    };

                    self.specials
                        .borrow_mut()
                        .define_top_level("return".to_string(), eval_value);
                }
            };
        }

        return Ok(());
    }
}
