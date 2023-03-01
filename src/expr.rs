use crate::token::Token;

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

pub enum Expr {
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
                return format!("{}", value.to_string());
            }
            Expr::Unary { operator, right } => {
                return format!("({} {})", &operator.value, (*right).to_string());
            }
        }
    }
}
