use programming_language::expr::{Expr, Literal};
use programming_language::lexer::Lexer;
use programming_language::parser::Parser;
use programming_language::stmt::Stmt;
use programming_language::token::Token;
use programming_language::token_type::TokenType;

fn assert_parse(input: &str, expected: &[Stmt]) {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.scan_tokens().map_err(|err| format!("{}", err));
    let mut parser = Parser::new(tokens.unwrap());

    match parser.parse() {
        Ok(statements) => assert_eq!(statements, expected),
        Err(msg) => panic!("{}", msg),
    }
}

#[test]
fn test_empty_program() {
    let input = "";
    let expected = vec![];
    assert_parse(input, &expected);
}

#[test]
fn test_var_declaration() {
    let input = "var x;";
    let expected = vec![Stmt::Var {
        name: Token::new(TokenType::Identifier, "x", None, 1),
        initializer: Expr::Literal {
            value: Literal::Nil,
        },
    }];
    assert_parse(input, &expected);
}

#[test]
fn test_var_declaration_with_initializer() {
    let input = "var x = 10;";
    let expected = vec![Stmt::Var {
        name: Token::new(TokenType::Identifier, "x", None, 1),
        initializer: Expr::Literal {
            value: Literal::Number(10.0),
        },
    }];
    assert_parse(input, &expected);
}

#[test]
fn test_print_statement() {
    let input = "print 1 + 2;";
    let expected = vec![Stmt::Print {
        expression: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::Number(1.0),
            }),
            operator: Token::new(TokenType::Plus, "+", None, 1),
            right: Box::new(Expr::Literal {
                value: Literal::Number(2.0),
            }),
        },
    }];
    assert_parse(input, &expected);
}

#[test]
fn test_print_variable() {
    let input = r#"
        var x = "Hello, world!";
        print x;
    "#;
    let expected = vec![
        Stmt::Var {
            name: Token::new(TokenType::Identifier, "x", None, 2),
            initializer: Expr::Literal {
                value: Literal::String("Hello, world!".to_string()),
            },
        },
        Stmt::Print {
            expression: Expr::Variable {
                name: Token::new(TokenType::Identifier, "x", None, 3),
            },
        },
    ];
    assert_parse(input, &expected);
}

#[test]
fn test_print_sum_number_variables() {
    let input = r#"
        var x = 10;
        var y = 6;
        print x + y;
    "#;
    let expected = vec![
        Stmt::Var {
            name: Token::new(TokenType::Identifier, "x", None, 2),
            initializer: Expr::Literal {
                value: Literal::Number(10 as f64),
            },
        },
        Stmt::Var {
            name: Token::new(TokenType::Identifier, "y", None, 3),
            initializer: Expr::Literal {
                value: Literal::Number(6 as f64),
            },
        },
        Stmt::Print {
            expression: Expr::Binary {
                left: Box::from(Expr::Variable {
                    name: Token::new(TokenType::Identifier, "x", None, 4),
                }),
                operator: Token::new(TokenType::Plus, "+", None, 4),
                right: Box::from(Expr::Variable {
                    name: Token::new(TokenType::Identifier, "y", None, 4),
                }),
            },
        },
    ];
    assert_parse(input, &expected);
}

#[test]
fn test_expression_statement() {
    let input = "1 + 2;";
    let expected = vec![Stmt::Expression {
        expression: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::Number(1.0),
            }),
            operator: Token::new(TokenType::Plus, "+", None, 1),
            right: Box::new(Expr::Literal {
                value: Literal::Number(2.0),
            }),
        },
    }];
    assert_parse(input, &expected);
}

#[test]
fn test_operator_precedence() {
    let input = "1 + 2 * 3;";
    let expected = vec![Stmt::Expression {
        expression: Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Literal::Number(1.0),
            }),
            operator: Token::new(TokenType::Plus, "+", None, 1),
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: Literal::Number(2.0),
                }),
                operator: Token::new(TokenType::Star, "*", None, 1),
                right: Box::new(Expr::Literal {
                    value: Literal::Number(3.0),
                }),
            }),
        },
    }];
    assert_parse(input, &expected);
}

#[test]
fn test_grouping() {
    let input = "(1 + 2) * 3;";
    let expected = vec![Stmt::Expression {
        expression: Expr::Binary {
            left: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal {
                        value: Literal::Number(1.0),
                    }),
                    operator: Token::new(TokenType::Plus, "+", None, 1),
                    right: Box::new(Expr::Literal {
                        value: Literal::Number(2.0),
                    }),
                }),
            }),
            operator: Token::new(TokenType::Star, "*", None, 1),
            right: Box::new(Expr::Literal {
                value: Literal::Number(3.0),
            }),
        },
    }];
    assert_parse(input, &expected);
}

#[test]
fn test_unary() {
    let input = "-1;";
    let expected = vec![Stmt::Expression {
        expression: Expr::Unary {
            operator: Token::new(TokenType::Minus, "-", None, 1),
            right: Box::new(Expr::Literal {
                value: Literal::Number(1.0),
            }),
        },
    }];
    assert_parse(input, &expected);
}
