use std::rc::Rc;

use crate::{
    parser::ast::expression::{Expression, Node},
    lexer::token::Token, errors::Span
};

#[derive(Debug)]
pub enum Statement {
    Expression {
        expr: Box<Expression>,
    },
    Print {
        expr: Box<Expression>,
    },
    Return {
        keyword: Token,
        expr: Box<Expression>,
    },
    Let {
        name: Token,
        expr: Box<Expression>
    },
    Assignment {
        operator: Token,
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
    // Repeat {
    //     times: Box<Expression>,
    //     body: Box<Statement>
    // },
    Func {
        keyword: Token,
        name: Token,
        params: Vec<Token>,
        body: Rc<Statement>
    }
}

impl Statement {
    pub fn get_span(&self) -> Span {
        match self {
            Self::Expression { expr } => {
                expr.get_node().get_span()
            },
            Self::Print { expr } => {
                expr.get_node().get_span()
            }
            Self::Return { keyword, expr } => {
                Span {
                    start: keyword.clone(),
                    end: expr.get_node().get_span().end
                }
            },
            Self::Func {
                keyword,
                name: _,
                params: _,
                body
            } => {
                Span {
                    start: Span::from(keyword.clone()).start,
                    end: body.get_span().end
                }
            }
            _ => todo!()
        }
    }
}
