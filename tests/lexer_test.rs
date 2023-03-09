use programming_language::lexer::Lexer;
use programming_language::token::{Literal, Token};
use programming_language::token_type::TokenType;

#[test]
fn test_single_character() {
    let result = Lexer::new("()").scan_tokens();
    let expected = Ok(vec![
        Token::new(TokenType::LeftParen, "(", None, 1),
        Token::new(TokenType::RightParen, ")", None, 1),
        Token::new(TokenType::EOF, "", None, 1),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_double_character() {
    let result = Lexer::new("!=()").scan_tokens();
    let expected = Ok(vec![
        Token::new(TokenType::BangEqual, "!=", None, 1),
        Token::new(TokenType::LeftParen, "(", None, 1),
        Token::new(TokenType::RightParen, ")", None, 1),
        Token::new(TokenType::EOF, "", None, 1),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_comment() {
    let result = Lexer::new(
        r#"
            //very beautiful comment
            ()
        "#,
    )
    .scan_tokens();
    let expected = Ok(vec![
        Token::new(TokenType::LeftParen, "(", None, 3),
        Token::new(TokenType::RightParen, ")", None, 3),
        Token::new(TokenType::EOF, "", None, 4),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_multi_line() {
    let result = Lexer::new(
        r#" 
        !=()

    "#,
    )
    .scan_tokens();
    let expected = Ok(vec![
        Token::new(TokenType::BangEqual, "!=", None, 2),
        Token::new(TokenType::LeftParen, "(", None, 2),
        Token::new(TokenType::RightParen, ")", None, 2),
        Token::new(TokenType::EOF, "", None, 4),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_string() {
    let result = Lexer::new(r#""try string""#).scan_tokens();
    let expected = Ok(vec![
        Token::new(
            TokenType::String,
            "\"try string\"",
            Some(Literal::String("try string".to_string())),
            1,
        ),
        Token::new(TokenType::EOF, "", None, 1),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_number() {
    let result = Lexer::new("123.45").scan_tokens();
    let expected = Ok(vec![
        Token::new(
            TokenType::Number,
            "123.45",
            Some(Literal::Number(123.45f64)),
            1,
        ),
        Token::new(TokenType::EOF, "", None, 1),
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_unexpected_char() {
    let result = Lexer::new("\'").scan_tokens();
    let expected = Err("Line 1: Unrecognized char '".to_string());

    assert_eq!(result, expected)
}

#[test]
fn test_identifier() {
    let result = Lexer::new("var return while other").scan_tokens();
    let expected = Ok(vec![
        Token::new(TokenType::Var, "var", None, 1),
        Token::new(TokenType::Return, "return", None, 1),
        Token::new(TokenType::While, "while", None, 1),
        Token::new(TokenType::Identifier, "other", None, 1),
        Token::new(TokenType::EOF, "", None, 1),
    ]);

    assert_eq!(result, expected)
}
