use std::{
    env, fs,
    io::{self, Write},
    process::exit,
};

use programming_language::{lexer::Lexer, parser::Parser};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
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
    eprintln!("{message}");
    exit(code);
}

fn run_file(path: &str) -> Result<(), String> {
    return match fs::read_to_string(path) {
        Ok(data) => run(&data),
        Err(err) => Err(err.to_string()),
    };
}

fn run(src: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.scan_tokens()?;

    let mut parser = Parser::new(tokens);
    let expr = parser.parse()?;
    let result = expr.evaluate()?;

    println!("{}", result.to_string());

    return Ok(());
}

fn run_prompt() -> Result<(), String> {
    loop {
        print!("> ");
        io::stdout().flush().expect("Error while flushing.");
        let mut buf = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buf).expect("Couldn't read line.");

        if buf.len() <= 2 {
            return Ok(());
        } else {
            run(&buf)?;
        }
    }
}
