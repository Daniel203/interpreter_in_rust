use core::fmt::Debug;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token;
use crate::token::Token;
use crate::token_type::TokenType;

type CallableFunction = Rc<dyn Fn(&[Literal]) -> Literal>;

#[derive(Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Callable {
        name: String,
        arity: usize,
        fun: CallableFunction,
    },
    Class {
        name: String,
    },
    Instance {
        class: Box<Literal>,
        fields: Rc<RefCell<Vec<(String, Literal)>>>,
    },
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}

macro_rules! class_name {
    ($class:expr) => {{
        if let Literal::Class { name } = &**$class {
            name
        } else {
            panic!("Unreachable")
        }
    }};
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Number(x) => x.to_string(),
            Literal::String(x) => x.to_string(),
            Literal::True => "true".to_string(),
            Literal::False => "false".to_string(),
            Literal::Nil => "nil".to_string(),
            Literal::Callable {
                name,
                arity: _,
                fun: _,
            } => format!("<fn {name}>"),
            Literal::Class { name } => format!("Class '{name}'"),
            Literal::Instance { class, fields: _ } => {
                format!("Instance of '{}'", class_name!(class))
            }
        }
    }
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (Literal::Number(x), Literal::Number(y)) => x == y,
            (
                Literal::Callable {
                    name,
                    arity,
                    fun: _,
                },
                Literal::Callable {
                    name: name2,
                    arity: arity2,
                    fun: _,
                },
            ) => name == name2 && arity == arity2,
            (Literal::String(x), Literal::String(y)) => x == y,
            (Literal::True, Literal::True) => true,
            (Literal::False, Literal::False) => true,
            (Literal::Nil, Literal::Nil) => true,
            _ => false,
        };
    }
}

fn unwrap_as_f64(literal: Option<token::Literal>) -> f64 {
    if let Some(token::Literal::Number(x)) = literal {
        return x;
    } else {
        panic!("Could not unwrap as f64")
    }
}

fn unwrap_as_string(literal: Option<token::Literal>) -> String {
    if let Some(token::Literal::String(s)) = literal {
        return s;
    } else {
        panic!("Could not unwrap as string")
    }
}

impl Literal {
    pub fn from_token_literal(literal: token::Literal) -> Self {
        return match literal {
            token::Literal::Number(val) => Self::Number(val),
            token::Literal::String(val) => Self::String(val),
            token::Literal::Identifier(val) => {
                if val == "true" {
                    return Self::True;
                } else if val == "false" {
                    return Self::False;
                } else if val == "Nil" {
                    return Self::Nil;
                } else {
                    panic!("Could not create literal from value {val:?}.");
                }
            }
        };
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f64(token.literal)),
            TokenType::String => Self::String(unwrap_as_string(token.literal)),
            TokenType::False => Self::False,
            TokenType::True => Self::True,
            TokenType::Nil => Self::Nil,
            _ => panic!("Could not create LiteralValue from {token:?}"),
        }
    }

    pub fn from_bool(b: bool) -> Self {
        if b {
            return Literal::True;
        } else {
            return Literal::False;
        }
    }

    pub fn is_falsey(&self) -> Literal {
        return match self {
            Literal::Number(x) => {
                if *x == 0 as f64 {
                    Literal::True
                } else {
                    Literal::False
                }
            }
            Literal::String(x) => {
                if x.is_empty() {
                    Literal::True
                } else {
                    Literal::False
                }
            }
            Literal::True => Literal::False,
            Literal::False => Literal::True,
            Literal::Nil => Literal::False,
            Literal::Callable {
                name: _,
                arity: _,
                fun: _,
            } => {
                panic!("Cannot use callable as falsey value")
            }
            Literal::Class { name: _ } => {
                panic!("Cannot use class as falsey value")
            }
            Literal::Instance {
                class: _,
                fields: _,
            } => panic!("Cannot use instance as falsey value"),
        };
    }

    pub fn is_truthy(&self) -> Literal {
        return match self {
            Literal::Number(x) => {
                if *x == 0 as f64 {
                    Literal::False
                } else {
                    Literal::True
                }
            }
            Literal::String(x) => {
                if x.is_empty() {
                    Literal::False
                } else {
                    Literal::True
                }
            }
            Literal::True => Literal::True,
            Literal::False => Literal::False,
            Literal::Nil => Literal::True,
            Literal::Callable {
                name: _,
                arity: _,
                fun: _,
            } => {
                panic!("Cannot use callable as truthy value.")
            }
            Literal::Class { name: _ } => {
                panic!("Cannot use class as truthy value")
            }
            Literal::Instance {
                class: _,
                fields: _,
            } => panic!("Cannot use instance as truthy value"),
        };
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    AnonFunction {
        id: usize,
        paren: Token,
        arguments: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    Assign {
        id: usize,
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        id: usize,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        id: usize,
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        id: usize,
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        id: usize,
        expression: Box<Expr>,
    },
    Literal {
        id: usize,
        value: Literal,
    },
    Logical {
        id: usize,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        id: usize,
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    Unary {
        id: usize,
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        id: usize,
        name: Token,
    },
}

impl Hash for Expr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        return std::ptr::hash(self, state);
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        let ptr = std::ptr::addr_of!(self);
        let ptr2 = std::ptr::addr_of!(other);
        ptr == ptr2
    }
}

impl Eq for Expr {}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Assign { id: _, name, value } => {
                return format!("({:?} = {})", name, (*value).to_string())
            }
            Expr::Binary {
                id: _,
                left,
                operator,
                right,
            } => {
                return format!(
                    "({} {} {})",
                    &operator.name,
                    (*left).to_string(),
                    (*right).to_string()
                );
            }
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                return format!("({}, {:?})", (*callee).to_string(), arguments);
            }
            Expr::Grouping { id: _, expression } => {
                return format!("(group {})", (*expression).to_string());
            }
            Expr::Literal { id: _, value } => {
                return value.to_string();
            }
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                return format!("({} {})", &operator.name, (*right).to_string());
            }
            Expr::Variable { id: _, name } => format!("(var {})", name.name),
            Expr::Logical {
                id: _,
                left,
                operator,
                right,
            } => {
                return format!(
                    "({} {} {})",
                    operator.to_string(),
                    left.to_string(),
                    right.to_string()
                )
            }
            Expr::AnonFunction {
                id: _,
                paren: _,
                arguments,
                body: _,
            } => format!("anon|{}", arguments.len()),
            Expr::Get {
                id: _,
                object,
                name,
            } => format!("(get {} {})", object.to_string(), name.name),
            Expr::Set {
                id: _,
                object,
                name,
                value,
            } => format!(
                "set {} {} {})",
                object.to_string(),
                name.to_string(),
                value.to_string()
            ),
        }
    }
}

impl Expr {
    pub fn get_id(&self) -> usize {
        return match self {
            Expr::AnonFunction {
                id,
                paren: _,
                arguments: _,
                body: _,
            } => *id,
            Expr::Assign {
                id,
                name: _,
                value: _,
            } => *id,
            Expr::Binary {
                id,
                left: _,
                operator: _,
                right: _,
            } => *id,
            Expr::Call {
                id,
                callee: _,
                paren: _,
                arguments: _,
            } => *id,
            Expr::Get {
                id,
                object: _,
                name: _,
            } => *id,
            Expr::Grouping { id, expression: _ } => *id,
            Expr::Literal { id, value: _ } => *id,
            Expr::Logical {
                id,
                left: _,
                operator: _,
                right: _,
            } => *id,
            Expr::Unary {
                id,
                operator: _,
                right: _,
            } => *id,
            Expr::Variable { id, name: _ } => *id,
            Expr::Set {
                id,
                object: _,
                name: _,
                value: _,
            } => *id,
        };
    }

    pub fn evaluate(&self, environment: Environment) -> Result<Literal, String> {
        return match self {
            Expr::AnonFunction {
                id: _,
                paren,
                arguments,
                body,
            } => {
                let arity = arguments.len();
                let env = environment;
                let arguments: Vec<Token> = arguments.iter().map(|t| (*t).clone()).collect();
                let body: Vec<Box<Stmt>> = body.iter().map(|s| (*s).clone()).collect();
                let parent = paren.clone();

                let fun_impl = move |args: &[Literal]| {
                    let mut anon_int = Interpreter::for_anon(env.clone());

                    for (i, arg) in args.iter().enumerate() {
                        anon_int.environment.define(
                            arguments
                                .get(i)
                                .expect("Cannot read function param")
                                .name
                                .clone(),
                            (*arg).clone(),
                        );
                    }

                    for i in 0..body.len() {
                        anon_int
                            .interpret(vec![body
                                .get(i)
                                .unwrap_or_else(|| panic!("Element in position {i} not found"))])
                            .unwrap_or_else(|_| {
                                panic!(
                                    "Evaluating failed inside anonymous function at line {}",
                                    parent.line
                                )
                            });

                        if let Some(value) = anon_int.specials.get("return") {
                            return value.clone();
                        }
                    }

                    return Literal::Nil;
                };

                return Ok(Literal::Callable {
                    name: "anon_function".to_string(),
                    arity,
                    fun: Rc::new(fun_impl),
                });
            }
            Expr::Get {
                id: _,
                object,
                name,
            } => {
                let obj_value = object.evaluate(environment)?;
                if let Literal::Instance { class: _, fields } = obj_value {
                    for (field_name, value) in fields.borrow().iter() {
                        if *field_name == name.name {
                            return Ok(value.clone());
                        }
                    }

                    return Err(format!("No field named '{}' on this instance", name.name));
                } else {
                    return Err(format!(
                        "Cannot access property on type '{}'",
                        obj_value.to_string()
                    ));
                }
            }
            Expr::Set {
                id: _,
                object,
                name,
                value,
            } => {
                let obj_value = object.evaluate(environment.clone())?;

                if let Literal::Instance { class: _, fields } = obj_value {
                    let value = value.evaluate(environment)?;

                    let mut idx = 0;
                    let mut found = false;

                    for i in 0..fields.borrow().len() {
                        let field_name = &fields.borrow()[i].0;
                        if field_name == &name.name {
                            idx = i;
                            found = true;
                            break;
                        }
                    }

                    if found {
                        fields.borrow_mut()[idx].1 = value;
                    } else {
                        fields.borrow_mut().push((name.name.clone(), value));
                    }

                    return Ok(Literal::Nil);
                } else {
                    return Err(format!(
                        "Cannot access property on type '{}'",
                        obj_value.to_string()
                    ));
                }
            }
            Expr::Grouping { id: _, expression } => expression.evaluate(environment),
            Expr::Literal { id: _, value } => Ok(value.clone()),
            Expr::Variable { id: _, name } => match environment.get(&name.name, self.get_id()) {
                Some(value) => Ok(value),
                None => Err(format!("Undefined variable '{}'.", name.name)),
            },
            Expr::Assign { id: _, name, value } => {
                let new_value = (*value).evaluate(environment.clone())?;
                let assign_success =
                    environment.assign(&name.name, new_value.clone(), self.get_id());

                if assign_success {
                    return Ok(new_value);
                } else {
                    return Err(format!("Variable {} was not declared", name.name));
                }
            }
            Expr::Unary {
                id: _,
                operator,
                right,
            } => {
                let right = (*right).evaluate(environment)?;

                return match (right.clone(), operator.token_type) {
                    (Literal::Number(x), TokenType::Minus) => Ok(Literal::Number(-x)),
                    (_, TokenType::Minus) => Err(format!("Minus not implemented for {right:?}")),
                    (any, TokenType::Bang) => Ok(any.is_falsey()),
                    (_, token_type) => Err(format!("{token_type:?} is not a valid unary operator")),
                };
            }
            Expr::Call {
                id: _,
                callee,
                paren: _,
                arguments,
            } => {
                let callable = (*callee).evaluate(environment.clone())?;
                match callable {
                    Literal::Callable { name, arity, fun } => {
                        if arguments.len() != arity {
                            return Err(format!(
                                "Callable {} expected {} arguments but got {}",
                                name,
                                arity,
                                arguments.len()
                            ));
                        }

                        let mut args_val = vec![];
                        for arg in arguments {
                            args_val.push(arg.evaluate(environment.clone())?);
                        }

                        return Ok(fun(&args_val));
                    }
                    Literal::Class { name: _ } => {
                        if !arguments.is_empty() {
                            return Err(
                                "Can only call the constructor with zero arguments".to_string()
                            );
                        }

                        return Ok(Literal::Instance {
                            class: Box::new(callable.clone()),
                            fields: Rc::new(RefCell::new(vec![])),
                        });
                    }
                    other => return Err(format!("{} is not callable", other.to_string())),
                };
            }
            Expr::Logical {
                id: _,
                left,
                operator,
                right,
            } => match operator.token_type {
                TokenType::Or => {
                    let left_value = left.evaluate(environment.clone())?;
                    let left_true = left_value.is_truthy();

                    if left_true == Literal::True {
                        return Ok(left_value);
                    } else {
                        return right.evaluate(environment);
                    }
                }
                TokenType::And => {
                    let left_true = left.evaluate(environment.clone())?.is_truthy();

                    if left_true == Literal::False {
                        return Ok(Literal::False);
                    } else {
                        return right.evaluate(environment);
                    }
                }
                token_type => Err(format!(
                    "Invalid token in logical expression: {token_type:?}"
                )),
            },
            Expr::Binary {
                id: _,
                left,
                operator,
                right,
            } => {
                let left = (*left).evaluate(environment.clone())?;
                let right = (*right).evaluate(environment)?;

                match (left, operator.token_type, right) {
                    (Literal::Number(l), TokenType::Minus, Literal::Number(r)) => {
                        return Ok(Literal::Number(l - r));
                    }
                    (Literal::Number(l), TokenType::Slash, Literal::Number(r)) => {
                        return Ok(Literal::Number(l / r));
                    }
                    (Literal::Number(l), TokenType::Star, Literal::Number(r)) => {
                        return Ok(Literal::Number(l * r));
                    }
                    (Literal::Number(l), TokenType::Plus, Literal::Number(r)) => {
                        return Ok(Literal::Number(l + r));
                    }
                    (Literal::String(l), TokenType::Plus, Literal::String(r)) => {
                        return Ok(Literal::String(format!("{l}{r}")));
                    }
                    (Literal::Number(l), TokenType::Plus, Literal::String(r)) => {
                        return Ok(Literal::String(format!("{l}{r}")));
                    }
                    (Literal::String(l), TokenType::Plus, Literal::Number(r)) => {
                        return Ok(Literal::String(format!("{l}{r}")));
                    }

                    (Literal::Number(l), TokenType::Greater, Literal::Number(r)) => {
                        return Ok(Literal::from_bool(l > r));
                    }
                    (Literal::Number(l), TokenType::GreaterEqual, Literal::Number(r)) => {
                        return Ok(Literal::from_bool(l >= r));
                    }
                    (Literal::Number(l), TokenType::Less, Literal::Number(r)) => {
                        return Ok(Literal::from_bool(l < r));
                    }
                    (Literal::Number(l), TokenType::LessEqual, Literal::Number(r)) => {
                        return Ok(Literal::from_bool(l <= r));
                    }
                    (Literal::String(l), TokenType::Greater, Literal::String(r)) => {
                        return Ok(Literal::from_bool(l > r));
                    }
                    (Literal::String(l), TokenType::GreaterEqual, Literal::String(r)) => {
                        return Ok(Literal::from_bool(l >= r));
                    }
                    (Literal::String(l), TokenType::Less, Literal::String(r)) => {
                        return Ok(Literal::from_bool(l < r));
                    }
                    (Literal::String(l), TokenType::LessEqual, Literal::String(r)) => {
                        return Ok(Literal::from_bool(l <= r));
                    }

                    (l, TokenType::EqualEqual, r) => {
                        return Ok(Literal::from_bool(l == r));
                    }
                    (l, TokenType::BangEqual, r) => {
                        return Ok(Literal::from_bool(l != r));
                    }

                    (Literal::String(_), op, Literal::Number(_)) => {
                        return Err(format!("{op:?} is not defined for string and number"));
                    }
                    (Literal::Number(_), op, Literal::String(_)) => {
                        return Err(format!("{op:?} is not defined for number and string"));
                    }

                    (l, token_type, r) => Err(format!(
                        "{token_type:?} is not implemented for operands {l:?} {r:?}",
                    )),
                }
            }
        };
    }
}
