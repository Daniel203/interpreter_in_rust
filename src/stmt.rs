use crate::{expr::Expr, token::Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Expression {
        expression: Expr,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Expr,
    },
    Block {
        statements: Vec<Box<Stmt>>,
    },
    IfStmt {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    WhileStmt {
        condition: Expr,
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    ReturnStmt {
        keyword: Token,
        value: Option<Expr>,
    },
    Class {
        name: Token,
        methods: Vec<Box<Stmt>>,
    }
}

impl ToString for Stmt {
    fn to_string(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Var {
                name,
                initializer: _,
            } => format!("(var {})", name.name),
            Self::Block { statements } => {
                return format!(
                    "(block {:?})",
                    statements
                        .iter()
                        .map(|stmt| stmt.to_string())
                        .collect::<String>()
                );
            }
            Stmt::IfStmt {
                condition: _,
                then_branch: _,
                else_branch: _,
            } => todo!(),
            Stmt::WhileStmt {
                condition: _,
                body: _,
            } => todo!(),
            Stmt::Function {
                name: _,
                params: _,
                body: _,
            } => todo!(),
            Stmt::ReturnStmt {
                keyword: _,
                value: _,
            } => todo!(),
            Stmt::Class { name:_, methods:_ } => todo!(),
        }
    }
}
