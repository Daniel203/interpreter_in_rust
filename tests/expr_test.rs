use programming_language::environment::Environment;
use programming_language::expr::{Expr, Literal};
use programming_language::token::Token;
use programming_language::token_type::TokenType;

#[test]
fn test_expr_ast() {
    let minus_token = Token::new(TokenType::Minus, "-", None, 0);
    let first_num = Expr::Literal {
        value: Literal::Number(123f64),
    };

    let minus_num = Box::from(Expr::Unary {
        operator: minus_token,
        right: Box::from(first_num),
    });

    let second_num = Box::from(Expr::Grouping {
        expression: Box::from(Expr::Literal {
            value: Literal::Number(34.5),
        }),
    });

    let multiplication = Token::new(TokenType::Star, "*", None, 0);

    let ast = Expr::Binary {
        left: minus_num,
        operator: multiplication,
        right: second_num,
    };

    let result = ast.to_string();

    let expected = "(* (- 123) (group 34.5))";
    assert_eq!(result, expected);
}

#[test]
fn test_evaluate() {
    let mut env = Environment::new();
    env.define("x".to_string(), Literal::Number(10.0));
    env.define("y".to_string(), Literal::Number(5.0));

    let expr = Expr::Binary {
        left: Box::new(Expr::Variable {
            name: Token::new(TokenType::Identifier, "x", None, 1),
        }),
        operator: Token::new(TokenType::Plus, "+", None, 1),
        right: Box::new(Expr::Literal {
            value: Literal::Number(2.0),
        }),
    };

    let result = expr.evaluate(&mut env).unwrap();
    assert_eq!(result, Literal::Number(12.0));
}
