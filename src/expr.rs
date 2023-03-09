use crate::environment::Environment;
use crate::token;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Number(x) => x.to_string(),
            Literal::String(x) => x.to_string(),
            Literal::True => "true".to_string(),
            Literal::False => "false".to_string(),
            Literal::Nil => "nil".to_string(),
        }
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
                    panic!("Could not create literal from value {:?}.", val);
                }
            }
        };
    }

    pub fn from_token(token: Token) -> Self {
        if let Some(literal) = token.literal {
            return Literal::from_token_literal(literal);
        } else {
            panic!("Token {} doesn't have literal.", token.to_string());
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
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Literal,
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
            Expr::Assign { name, value } => format!("({:?} = {})", name, (*value).to_string()),
        }
    }
}

impl Expr {
    pub fn evaluate(&self, environment: &mut Environment) -> Result<Literal, String> {
        return match self {
            Expr::Grouping { expression } => expression.evaluate(environment),
            Expr::Literal { value } => Ok(value.clone()),
            Expr::Variable { name } => match environment.get(name.value.clone()) {
                Some(value) => Ok(value.clone()),
                None => Err(format!("Undefined variable '{}'.", name.value)),
            },
            Expr::Assign { name, value } => {
                let new_value = (*value).evaluate(environment)?;
                let assign_succes = environment.assign(&name.value, new_value.clone());

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
                    (_, TokenType::Minus) => Err(format!("Minus not implemented for {:?}", right)),
                    (any, TokenType::Bang) => Ok(any.is_falsey()),
                    (_, token_type) => {
                        Err(format!("{:?} is not a valid unary operator", token_type))
                    }
                };
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = (*left).evaluate(environment)?;
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
                        return Ok(Literal::String(format!("{}{}", l, r)));
                    }
                    (Literal::Number(l), TokenType::Plus, Literal::String(r)) => {
                        return Ok(Literal::String(format!("{}{}", l, r)));
                    }
                    (Literal::String(l), TokenType::Plus, Literal::Number(r)) => {
                        return Ok(Literal::String(format!("{}{}", l, r)));
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
                        return Err(format!("{:?} is not defined for string and number", op));
                    }
                    (Literal::Number(_), op, Literal::String(_)) => {
                        return Err(format!("{:?} is not defined for number and string", op));
                    }

                    (l, token_type, r) => Err(format!(
                        "{:?} is not implemented for operands {:?} {:?}",
                        token_type, l, r
                    )),
                }
            }
        };
    }
}
