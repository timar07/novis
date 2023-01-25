use crate::{
    parser::ast::expression::Expression,
    lexer::token::Token
};

#[derive(Debug)]
pub enum Statement {
    Print {
        expr: Box<Expression>,
    },
    Let {
        name: Token,
        expr: Box<Expression>
    },
    Group(Vec<Statement>),
    Cond {
        condition: Box<Expression>,
        if_block: Box<Statement>
    },
}
