use std::process;
use crate::{parser::ast::statement::Statement, errors::DescribableError};
use super::{
    statement::statement,
    env::Env,
    runtime_error::{
        InterpreterException,
        RuntimeError,
        RuntimeErrorTag::*
    }
};

pub struct Interpreter {
    statements: Vec<Statement>
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Interpreter {
        Interpreter { statements: statements }
    }

    pub fn interpret(&self) -> () {
        let mut global_env = Box::new(Env::new());

        for stmt in &self.statements {
            if let Err(error) = statement(global_env.as_mut(), stmt) {
                match error {
                    InterpreterException::Fatal(fatal) => {
                        fatal.print();
                        process::exit(1)
                    },
                    InterpreterException::Return(_) => {
                        RuntimeError {
                            tag: ReturnOutOfFunction
                        }
                    }
                }.print();
            };
        };
    }
}