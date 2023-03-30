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
        superclass: Option<Expr>,
    },
}

impl ToString for Stmt {
    fn to_string(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Var { name, .. } => format!("(var {})", name.name),
            Self::Block { statements } => {
                return format!(
                    "(block {:?})",
                    statements
                        .iter()
                        .map(|stmt| stmt.to_string())
                        .collect::<String>()
                );
            }
            Stmt::IfStmt { .. } => todo!(),
            Stmt::WhileStmt { .. } => todo!(),
            Stmt::Function { .. } => todo!(),
            Stmt::ReturnStmt { .. } => todo!(),
            Stmt::Class { .. } => todo!(),
        }
    }
}
