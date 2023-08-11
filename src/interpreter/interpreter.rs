use std::process;
use crate::{
    parser::ast::statement::Statement,
    errors::DescribableError
};
use super::{
    statement::Executable,
    env::Env,
    runtime_exception::{
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
            if let Err(error) = stmt.run(global_env.as_mut()) {
                match error {
                    InterpreterException::Fatal(fatal) => {
                        fatal.print();
                        process::exit(1)
                    },
                    InterpreterException::Return(value) => {
                        RuntimeError {
                            span: value.span,
                            tag: ReturnOutOfFunction
                        }.print();
                        process::exit(1)
                    }
                };
            };
        };
    }
}
