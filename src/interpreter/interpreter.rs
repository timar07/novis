use std::process;
use crate::{parser::ast::statement::Statement, errors::DescribableError};
use super::{
    statement::statement, env::Env
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
                error.print();
                process::exit(1);
            };
        };
    }
}