use crate::{
    lexer::token::{
        Token
    }
};

#[derive(Debug, Clone)]
pub enum LiteralValue {
    String(String),
    Number(f64),
}

#[derive(Debug)]
pub enum PrimaryNode {
    Literal(LiteralValue),
    Paren(Box<Expression>),
    Identifier(Token),
    Call(Token)
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
