mod errors;
mod lexer;
mod parser;
mod interpreter;
mod file_stream;

use std::time::{Instant};
use colored::Colorize;
use std::{env, process::exit};
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn run(path: &String) {
    let mut lexer = Lexer::from_file(path);

    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    if let Ok(statements) = statements {
        let interpreter = Interpreter::new(statements);
        interpreter.interpret();
    }
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        // TODO: Dialog mode
        eprintln!("Usage: `novis <file name>`");
        exit(1);
    }

    if argv.contains(&String::from("--bench")) {
        let now = Instant::now();

        run(&argv[1]);

        println!(
            "Executed in: {} {}",
            now.elapsed().as_nanos().to_string().yellow(),
            "ns".yellow()
        );
    } else {
        run(&argv[1]);
    }
}
