use std::{
    cell::RefCell,
    env, fs,
    io::{self, Write},
    process::exit,
    rc::Rc,
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
    eprintln!("Error: {message}");
    exit(code);
}

fn run_file(path: &str) -> Result<(), String> {
    let interpreter = Rc::new(RefCell::new(Interpreter::new()));
    return match fs::read_to_string(path) {
        Ok(data) => run(&data, interpreter),
        Err(err) => Err(err.to_string()),
    };
}

fn run(src: &str, interpreter: Rc<RefCell<Interpreter>>) -> Result<(), String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse()?;

    let mut resolver = Resolver::new(interpreter.clone());
    resolver.resolve_many(&stmts.iter().collect())?;

    interpreter.borrow_mut().interpret(stmts.iter().collect())?;

    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    let interpreter = Rc::new(RefCell::new(Interpreter::new()));

    loop {
        print!("> ");
        io::stdout().flush().expect("Error while flushing.");
        let mut buf = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buf).expect("Couldn't read line.");

        if buf.len() <= 2 {
            return Ok(());
        } else {
            match run(&buf, interpreter.clone()) {
                Ok(_) => (),
                Err(msg) => println!("{msg}"),
            };
        }

        println!();
    }
}

pub fn run_string(contents: &str) -> Result<(), String> {
    let interpreter = Rc::new(RefCell::new(Interpreter::new()));

    return run(contents, interpreter);
}
