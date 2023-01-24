use crate::{lexer::token::Token, parser::ast::expression::Expression};

pub enum Statement {
    Print {
        value: Expression,
        keyword: Token
    }
}