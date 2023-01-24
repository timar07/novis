use std::process;

use crate::{
    parser::ast::{expression::{
        Expression,
        BinaryNode,
        UnaryNode,
        PrimaryNode
    }, statement::Statement},
    lexer::token::{TokenTag, Token}
};

use super::runtime_error::RuntimeError;

pub struct Interpreter {
    statements: Vec<Statement>
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Interpreter {
        Interpreter { statements: statements }
    }

    pub fn interpret(&self) -> () {
        for statement in &self.statements {
            // match statement {
            //     Statement::Print {value, keyword: _} => self.print(value)
            // };
        };
    }

    fn print(&self, val: &Token) {
        match val.tag {
            TokenTag::Number(n) => println!("{n}"),
            _ => panic!("Cannot print")
        }
    }
}