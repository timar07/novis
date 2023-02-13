use std::process;
use crate::{
    parser::ast::statement::Statement
};
use super::{
    statement::Executable,
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
            if let Err(error) = stmt.run(global_env.as_mut()) {
                match error {
                    InterpreterException::Fatal(fatal) => {
                        eprint!("{:?}", fatal);
                        process::exit(1)
                    },
                    InterpreterException::Return(_) => {
                        RuntimeError {
                            span: todo!(),
                            tag: ReturnOutOfFunction
                        }
                    }
                };
            };
        };
    }
}
