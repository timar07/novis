mod errors;
mod lexer;
mod parser;
mod interpreter;
mod code_stream;

use std::{env, process::exit};
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        // TODO: Dialog mode
        eprintln!("Usage: `novis <file name>`");
        exit(1);
    }

    let mut lexer = Lexer::from_file(&argv[1]);

    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    if let Ok(statements) = statements {
        let interpreter = Interpreter::new(statements);
        interpreter.interpret();
    }
}
