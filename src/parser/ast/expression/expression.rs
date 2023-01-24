use crate::{lexer::token::{
    Token
}, errors::DebugInfo};

#[derive(Debug)]
pub enum PrimaryNode {
    Literal(f64),
    Paren(Box<Expression>),
}

pub struct UnaryNode {
    pub left: Box<Expression>,
    pub op: Token,
}

pub struct BinaryNode {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: Token,
}

#[derive(Debug)]
pub enum Expression {
    Primary(PrimaryNode),
    Unary(UnaryNode),
    Binary(BinaryNode)
}
