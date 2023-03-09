use programming_language::environment::Environment;
use programming_language::expr::Literal;

#[test]
fn define_and_get_variable() {
    let mut env = Environment::new();
    env.define("x".to_string(), Literal::Number(42.0));
    let value = env.get("x".to_string());
    assert_eq!(value, Some(&Literal::Number(42.0)));
}

#[test]
fn undefined_variable() {
    let env = Environment::new();
    let value = env.get("x".to_string());
    assert_eq!(value, None);
}

#[test]
fn assign_existing_variable() {
    let mut env = Environment::new();
    env.define("x".to_string(), Literal::Number(42.0));
    let value = env.get("x".to_string());
    assert_eq!(value, Some(&Literal::Number(42.0)));

    env.define("x".to_string(), Literal::Number(43.0));
    let value = env.get("x".to_string());
    assert_eq!(value, Some(&Literal::Number(43.0)));
}

#[test]
fn assign_non_existing_variable() {
    let mut env = Environment::new();
    env.define("x".to_string(), Literal::Number(42.0));
    let value = env.get("x".to_string());
    assert_eq!(value, Some(&Literal::Number(42.0)));
}
