use programming_language::lexer::tokenize;
use programming_language::token::Token;
use programming_language::token_type::TokenType;

#[test]
fn tokenize_equals() {
    let input = "==".to_string();
    let expected = vec![
        Token::new(TokenType::EqualEqual, "==".to_string(), 1),
        Token::new(TokenType::EOF, "".to_string(), 1),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}

#[test]
fn tokenize_not_equal() {
    let input = "!=".to_string();
    let expected = vec![
        Token::new(TokenType::BangEqual, "!=".to_string(), 1),
        Token::new(TokenType::EOF, "".to_string(), 1),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}

#[test]
fn tokenize_greater() {
    let input = ">".to_string();
    let expected = vec![
        Token::new(TokenType::Greater, ">".to_string(), 1),
        Token::new(TokenType::EOF, "".to_string(), 1),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}

#[test]
fn tokenize_multi_line() {
    let input = ">\n>=".to_string();
    let expected = vec![
        Token::new(TokenType::Greater, ">".to_string(), 1),
        Token::new(TokenType::GreaterEqual, ">=".to_string(), 2),
        Token::new(TokenType::EOF, "".to_string(), 2),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}

#[test]
fn tokenize_unhandled_char() {
    let input = "\t".to_string();
    assert_eq!(Err(65), tokenize(input));
}

#[test]
fn tokenize_string() {
    let input = "\"test-string\"".to_string();
    let expected = vec![
        Token::new(TokenType::String, "test-string".to_string(), 1),
        Token::new(TokenType::EOF, "".to_string(), 1),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}

#[test]
fn tokenize_string_multi_line() {
    let input = "\"test-string\nnew-line\"".to_string();
    let expected = vec![
        Token::new(TokenType::String, "test-stringnew-line".to_string(), 2),
        Token::new(TokenType::EOF, "".to_string(), 2),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}

#[test]
fn tokenize_comment() {
    let input = ">= // test comment in this line".to_string();
    let expected = vec![
        Token::new(TokenType::GreaterEqual, ">=".to_string(), 1),
        Token::new(TokenType::EOF, "".to_string(), 1),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}

#[test]
fn tokenize_integer_number() {
    let input = "1234".to_string();
    let expected = vec![
        Token::new(TokenType::Number, "1234".to_string(), 1),
        Token::new(TokenType::EOF, "".to_string(), 1),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}

#[test]
fn tokenize_float_number() {
    let input = "1234.56".to_string();
    let expected = vec![
        Token::new(TokenType::Number, "1234.56".to_string(), 1),
        Token::new(TokenType::EOF, "".to_string(), 1),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}

#[test]
fn tokenize_identifiers() {
    let input = "let for and other".to_string();
    let expected = vec![
        Token::new(TokenType::Let, "let".to_string(), 1),
        Token::new(TokenType::For, "for".to_string(), 1),
        Token::new(TokenType::And, "and".to_string(), 1),
        Token::new(TokenType::Identifier, "other".to_string(), 1),
        Token::new(TokenType::EOF, "".to_string(), 1),
    ];
    assert_eq!(Ok(expected), tokenize(input));
}
