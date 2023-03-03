use programming_language::expr::{Expr, Literal};
use programming_language::token::Token;
use programming_language::token_type::TokenType;

#[test]
fn expr_ast() {
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
