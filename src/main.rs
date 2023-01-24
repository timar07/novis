mod errors;
mod lexer;
mod parser;
mod interpreter;
mod code_stream;

use std::{env};
use code_stream::{
    FileStream
};
use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;
// use crate::errors::DescribableError;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let source = FileStream::new(&argv[1]).as_str();

    let mut lexer = Lexer::new(&source)
        .expect("Couldn't open file");

    let tokens = lexer.lex();
    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    if let Ok(statements) = statements {
        let interpreter = Interpreter::new(statements);
        interpreter.interpret();
    }
}
