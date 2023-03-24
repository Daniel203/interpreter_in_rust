use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{environment::Environment, expr::Literal, stmt::Stmt, token::Token};

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

    fn for_closure(
        parent: Rc<RefCell<Environment>>,
        locals: Rc<RefCell<HashMap<usize, usize>>>,
    ) -> Self {
        let environment = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);

        return Self {
            specials: Rc::new(RefCell::new(HashMap::new())),
            environment,
            locals,
        };
    }

    pub fn for_anon(
        parent: Rc<RefCell<Environment>>,
        locals: Rc<RefCell<HashMap<usize, usize>>>,
    ) -> Self {
        let mut env = Environment::new();
        env.enclosing = Some(parent);

        return Self {
            specials: Rc::new(RefCell::new(HashMap::new())),
            environment: Rc::new(RefCell::new(env)),
            locals,
        };
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(self.environment.clone(), self.locals.clone())?;
                }
                Stmt::Print { expression } => {
                    let value =
                        expression.evaluate(self.environment.clone(), self.locals.clone())?;
                    println!("{}", value.to_string());
                }
                Stmt::Var { name, initializer } => {
                    let value =
                        initializer.evaluate(self.environment.clone(), self.locals.clone())?;

                    self.environment
                        .borrow_mut()
                        .define(name.name.clone(), value);
                }
                Stmt::Block { statements } => {
                    let mut new_environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());

                    let old_environment = self.environment.clone();

                    self.environment = Rc::new(RefCell::new(new_environment));
                    let block_result =
                        self.interpret((*statements).iter().map(|b| b.as_ref()).collect());
                    self.environment = old_environment;

                    block_result?;
                }
                Stmt::IfStmt {
                    condition,
                    then_branch,
                    else_branch,
                } => {
                    let truth_value =
                        condition.evaluate(self.environment.clone(), self.locals.clone())?;

                    if truth_value.is_truthy() == Literal::True {
                        self.interpret(vec![then_branch.as_ref()])?;
                    } else if let Some(else_stmt) = else_branch {
                        self.interpret(vec![else_stmt.as_ref()])?;
                    }
                }
                Stmt::WhileStmt { condition, body } => {
                    let mut flag =
                        condition.evaluate(self.environment.clone(), self.locals.clone())?;

                    while flag.is_truthy() == Literal::True {
                        self.interpret(vec![body.as_ref()])?;
                        flag = condition.evaluate(self.environment.clone(), self.locals.clone())?;
                    }
                }
                Stmt::Function { name, params, body } => {
                    let arity = params.len();

                    let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
                    let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
                    let name_clone = name.name.clone();

                    let parent_env = self.environment.clone();
                    let parent_locals = self.locals.clone();
                    let fun_impl = move |args: &[Literal]| {
                        let mut clos_int =
                            Interpreter::for_closure(parent_env.clone(), parent_locals.clone());

                        for (i, arg) in args.iter().enumerate() {
                            clos_int.environment.borrow_mut().define(
                                params
                                    .get(i)
                                    .expect("Cannot read function param")
                                    .name
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

                            if let Some(value) = clos_int.specials.borrow().get("return") {
                                return value.clone();
                            }
                        }

                        return Literal::Nil;
                    };

                    let callable = Literal::Callable {
                        name: name.name.clone(),
                        arity,
                        fun: Rc::new(fun_impl),
                    };

                    self.environment
                        .borrow_mut()
                        .define(name.name.clone(), callable);
                }
                Stmt::ReturnStmt { keyword: _, value } => {
                    let eval_value = if let Some(value) = value {
                        value.evaluate(self.environment.clone(), self.locals.clone())?
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

    pub fn resolve(&mut self, id: usize, steps: usize) -> Result<(), String> {
        self.locals.borrow_mut().insert(id, steps);
        return Ok(());
    }
}
