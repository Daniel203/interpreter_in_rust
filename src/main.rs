use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use programming_language::{
    interpreter::Interpreter, lexer::Lexer, parser::Parser, resolver::Resolver,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => match run_string(&args[2]) {
            Ok(_) => exit(0),
            Err(err) => error(&err, 64),
        },
        2 => {
            match run_file(&args[1]) {
                Ok(_) => exit(0),
                Err(err) => error(&err, 64),
            };
        }
        1 => match run_prompt() {
            Ok(_) => exit(0),
            Err(err) => error(&err, 64),
        },
        _ => {
            error("Usage: 'program_name' [script]", 64);
        }
    };
}

pub fn error(message: &str, code: i32) {
    println!("Error: {message}");
    exit(code);
}

fn run_file(path: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();

    return match fs::read_to_string(path) {
        Ok(data) => run(&data, &mut interpreter),
        Err(err) => Err(err.to_string()),
    };
}

fn run(src: &str, interpreter: &mut Interpreter) -> Result<(), String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;

    let resolver = Resolver::new();
    let locals = resolver.resolve(&stmts.iter().collect())?;

    interpreter.resolve(locals);
    interpreter.interpret(stmts.iter().collect())?;

    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("Error while flushing.");
        let mut buf = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buf).expect("Couldn't read line.");

        if buf.len() <= 2 {
            return Ok(());
        } else {
            match run(&buf, &mut interpreter) {
                Ok(_) => (),
                Err(msg) => println!("{msg}"),
            };
        }

        println!();
    }
}

pub fn run_string(contents: &str) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    return run(contents, &mut interpreter);
}
