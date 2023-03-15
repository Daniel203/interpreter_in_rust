use crate::{expr::Expr, token::Token};

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression { expression: Expr },
    Print { expression: Expr },
    Var { name: Token, initializer: Expr },
    Block { statements: Vec<Stmt> },
}

impl ToString for Stmt {
    fn to_string(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Var {
                name,
                initializer: _,
            } => format!("(var {})", name.value),
            Self::Block { statements } => {
                return format!(
                    "(block {:?})",
                    statements
                        .iter()
                        .map(|stmt| stmt.to_string())
                        .collect::<String>()
                );
            }
        }
    }
}
