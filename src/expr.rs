use core::fmt::Debug;
use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::token;
use crate::token::Token;
use crate::token_type::TokenType;

type CallableFunction = Rc<dyn Fn(Rc<RefCell<Environment>>, &[Literal]) -> Literal>;

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
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_string());
    }
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
                arity,
                fun: _,
            } => format!("{name}|{arity}"),
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
                panic!("Cannot use callable as falsey value.")
            }
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
        };
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

impl ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Assign { name, value } => format!("({:?} = {})", name, (*value).to_string()),
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                return format!(
                    "({} {} {})",
                    &operator.value,
                    (*left).to_string(),
                    (*right).to_string()
                );
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                return format!("({}, {:?})", (*callee).to_string(), arguments);
            }
            Expr::Grouping { expression } => {
                return format!("(group {})", (*expression).to_string());
            }
            Expr::Literal { value } => {
                return value.to_string();
            }
            Expr::Unary { operator, right } => {
                return format!("({} {})", &operator.value, (*right).to_string());
            }
            Expr::Variable { name } => format!("(var {})", name.value),
            Expr::Logical {
                left,
                operator,
                right,
            } => format!(
                "({} {} {})",
                operator.to_string(),
                left.to_string(),
                right.to_string()
            ),
        }
    }
}

impl Expr {
    pub fn evaluate(&self, environment: Rc<RefCell<Environment>>) -> Result<Literal, String> {
        return match self {
            Expr::Grouping { expression } => expression.evaluate(environment),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Variable { name } => match environment.borrow().get(&name.value) {
                Some(value) => Ok(value),
                None => Err(format!("Undefined variable '{}'.", name.value)),
            },
            Expr::Assign { name, value } => {
                let new_value = (*value).evaluate(environment.clone())?;
                let assign_succes = environment
                    .borrow_mut()
                    .assign(&name.value, new_value.clone());

                if assign_succes {
                    return Ok(new_value);
                } else {
                    return Err(format!("Variable {} was not declared", name.value));
                }
            }
            Expr::Unary { operator, right } => {
                let right = (*right).evaluate(environment)?;

                return match (right.clone(), operator.token_type) {
                    (Literal::Number(x), TokenType::Minus) => Ok(Literal::Number(-x)),
                    (_, TokenType::Minus) => Err(format!("Minus not implemented for {right:?}")),
                    (any, TokenType::Bang) => Ok(any.is_falsey()),
                    (_, token_type) => Err(format!("{token_type:?} is not a valid unary operator")),
                };
            }
            Expr::Call {
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

                        return Ok(fun(environment, &args_val));
                    }
                    other => return Err(format!("{} is not callable", other.to_string())),
                };
            }
            Expr::Logical {
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
