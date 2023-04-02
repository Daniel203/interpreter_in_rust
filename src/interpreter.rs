use std::collections::HashMap;

use crate::{
    environment::Environment,
    expr::{CallableImpl, FunctionImpl, Literal},
    stmt::Stmt,
    token::Token,
};

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

    pub fn with_env(env: Environment) -> Self {
        return Self {
            specials: HashMap::new(),
            environment: env,
        };
    }

    pub fn for_anon(parent: Environment) -> Self {
        let env = parent.enclose();

        return Self {
            specials: HashMap::new(),
            environment: env,
        };
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(self.environment.clone())?;
                }
                Stmt::Print {
                    expression,
                    arguments,
                } => {
                    let value = expression.evaluate(self.environment.clone())?;
                    let mut string = value.to_string();

                    let mut args = Vec::new();
                    for arg in arguments.iter().rev() {
                        args.push(arg.evaluate(self.environment.clone())?);
                    }

                    while !args.is_empty() {
                        let arg = args.pop().unwrap();
                        string = string.replacen("{}", &arg.to_string(), 1);
                    }

                    println!("{}", string);
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
                    let fun = Literal::Callable(CallableImpl::Function(callable));
                    self.environment.define(name.name.clone(), fun);
                }
                Stmt::ReturnStmt { keyword: _, value } => {
                    let eval_value = if let Some(value) = value {
                        value.evaluate(self.environment.clone())?
                    } else {
                        Literal::Nil
                    };

                    self.specials.insert("return".to_string(), eval_value);
                }
                Stmt::Class {
                    name,
                    methods,
                    superclass,
                } => {
                    let mut methods_map = HashMap::new();

                    let superclass_value;
                    if let Some(superclass) = superclass {
                        let superclass = superclass.evaluate(self.environment.clone())?;

                        if let Literal::Class { .. } = superclass {
                            superclass_value = Some(Box::new(superclass));
                        } else {
                            return Err(format!(
                                "Superclass must be a class, not '{}'",
                                superclass.to_type(),
                            ));
                        }
                    } else {
                        superclass_value = None;
                    }

                    self.environment.define(name.name.clone(), Literal::Nil);

                    self.environment = self.environment.enclose();
                    if let Some(sc) = superclass_value.clone() {
                        self.environment.define("super".to_string(), *sc);
                    }

                    for method in methods {
                        if let Stmt::Function { name, .. } = method.as_ref() {
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
                        methods: methods_map.clone(),
                        superclass: superclass_value,
                    };

                    if !self.environment.assign_global(&name.name, class) {
                        return Err(format!("Class definition failed for {}", name.name));
                    }

                    self.environment = *self.environment.enclosing.clone().unwrap();
                }
            };
        }

        return Ok(());
    }

    fn make_function(&self, fn_stmt: &Stmt) -> FunctionImpl {
        if let Stmt::Function { name, params, body } = fn_stmt {
            let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
            let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();

            return FunctionImpl {
                name: name.name.clone(),
                arity: params.len(),
                parent_env: self.environment.clone(),
                params,
                body,
            };
        } else {
            panic!("Tried to make a function from a non-function statement");
        }
    }
}
