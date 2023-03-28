use std::{collections::HashMap, rc::Rc};

use crate::{environment::Environment, expr::Literal, stmt::Stmt, token::Token};

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub specials: HashMap<String, Literal>,
    pub environment: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        return Self::new();
    }
}

impl Interpreter {
    pub fn new() -> Self {
        return Self {
            specials: HashMap::new(),
            environment: Environment::new(HashMap::new()),
        };
    }

    pub fn resolve(&mut self, locals: HashMap<usize, usize>) {
        self.environment.resolve(locals);
    }

    fn for_closure(parent: Environment) -> Self {
        let environment = parent.enclose();

        return Self {
            specials: HashMap::new(),
            environment,
        };
    }

    pub fn for_anon(parent: Environment) -> Self {
        let environment = parent.enclose();

        return Self {
            specials: HashMap::new(),
            environment,
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

                    self.environment.define(name.name.clone(), value);
                }
                Stmt::Block { statements } => {
                    let new_environment = self.environment.enclose();
                    let old_environment = self.environment.clone();

                    self.environment = new_environment;
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
                Stmt::Function {
                    name,
                    params: _,
                    body: _,
                } => {
                    let callable = self.make_function(stmt);
                    self.environment.define(name.name.clone(), callable);
                }
                Stmt::ReturnStmt { keyword: _, value } => {
                    let eval_value = if let Some(value) = value {
                        value.evaluate(self.environment.clone())?
                    } else {
                        Literal::Nil
                    };

                    self.specials.insert("return".to_string(), eval_value);
                }
                Stmt::Class { name, methods } => {
                    self.environment.define(name.name.clone(), Literal::Nil);

                    let mut methods_map = HashMap::new();
                    for method in methods {
                        if let Stmt::Function {
                            name,
                            params: _,
                            body: _,
                        } = method.as_ref()
                        {
                            let function = self.make_function(method.as_ref());
                            methods_map.insert(name.name.clone(), function);
                        } else {
                            panic!(
                                "Something that was not a function was in the methods of a class"
                            );
                        }
                    }

                    let class = Literal::Class {
                        name: name.name.clone(),
                        methods: methods_map,
                    };
                    if !self.environment.assign_global(&name.name, class) {
                        return Err(format!("Class definition failed for {}", name.name));
                    }
                }
            };
        }

        return Ok(());
    }

    fn make_function(&self, fn_stmt: &Stmt) -> Literal {
        if let Stmt::Function { name, params, body } = fn_stmt {
            let arity = params.len();

            let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
            let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
            let name_clone = name.name.clone();

            let parent_env = self.environment.clone();
            let fun_impl = move |args: &[Literal]| {
                let mut clos_int = Interpreter::for_closure(parent_env.clone());

                for (i, arg) in args.iter().enumerate() {
                    clos_int.environment.define(
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
                        .unwrap_or_else(|_| panic!("Evaluating failed inside {name_clone}"));

                    if let Some(value) = clos_int.specials.get("return") {
                        return value.clone();
                    }
                }

                return Literal::Nil;
            };

            return Literal::Callable {
                name: name.name.clone(),
                arity,
                fun: Rc::new(fun_impl),
            };
        } else {
            panic!("Tried to make a function from a non-function statement");
        }
    }
}
