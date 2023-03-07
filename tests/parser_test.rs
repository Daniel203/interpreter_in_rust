use programming_language::{
    lexer::Lexer,
    parser::Parser,
    token::{Literal, Token},
    token_type::TokenType,
};

#[test]
fn parser_addition() {
    let one = Token::new(TokenType::Number, "1", Some(Literal::Number(1_f64)), 0);
    let plus = Token::new(TokenType::Plus, "+", None, 0);
    let two = Token::new(TokenType::Number, "2", Some(Literal::Number(2_f64)), 0);
    let semicolon = Token::new(TokenType::Semicolon, ";", None, 0);

    let tokens = vec![one, plus, two, semicolon];
    let mut parser = Parser::new(tokens);

    let result = parser.expression();
    let expected = "(+ 1 2)";

    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().to_string(), expected);
}

#[test]
fn parser_comparison() {
    let source = "1 + 2 == 5 + 7";
    let mut lexer = Lexer::new(source);

    let tokens = lexer.scan_tokens();
    assert_eq!(tokens.is_ok(), true);

    let mut parser = Parser::new(tokens.ok().unwrap());

    let result = parser.expression();
    let expected = "(== (+ 1 2) (+ 5 7))";

    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().to_string(), expected);
}

#[test]
fn parser_evaluation_numbers_equal() {
    // false
    let source = "1 == 2";
    let mut lexer = Lexer::new(source);

    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens.ok().unwrap());
    let result = parser.expression().unwrap().evaluate();

    let expected = "false";

    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().to_string(), expected);

    // true
    let source = "2 == 2";
    let mut lexer = Lexer::new(source);

    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens.ok().unwrap());
    let result = parser.expression().unwrap().evaluate();

    let expected = "true";

    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().to_string(), expected);
}

#[test]
fn parser_evaluation_strings_contatenation() {
    // string + string
    let source = r#""Hello " + "world""#;
    let mut lexer = Lexer::new(source);

    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens.ok().unwrap());
    let result = parser.expression().unwrap().evaluate();

    let expected = "Hello world";

    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().to_string(), expected);

    // string + number
    let source = r#""number: " + 10"#;
    let mut lexer = Lexer::new(source);

    let tokens = lexer.scan_tokens();
    let mut parser = Parser::new(tokens.ok().unwrap());
    let result = parser.expression().unwrap().evaluate();

    let expected = "number: 10";

    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap().to_string(), expected);
}