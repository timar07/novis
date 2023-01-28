use std::rc::Rc;

use crate::{
    parser::ast::expression::Expression,
    lexer::token::Token
};

#[derive(Debug)]
pub enum Statement {
    Expression {
        expr: Box<Expression>,
    },
    Print {
        expr: Box<Expression>,
    },
    Let {
        name: Token,
        expr: Box<Expression>
    },
    Assignment {
        name: Token,
        expr: Box<Expression>
    },
    Group(Vec<Statement>),
    Cond {
        condition: Box<Expression>,
        if_block: Box<Statement>,
        else_block: Option<Box<Statement>>
    },
    Loop {
        condition: Box<Expression>,
        body: Box<Statement>
    },
    Func {
        name: Token,
        params: Vec<Token>,
        body: Rc<Statement>
    }
}
