use crate::{
    lexer::token::{
        Token
    }
};

#[derive(Debug)]
pub struct Expression {
    node: ExpressionNode
}

impl Expression {
    pub fn create(node: ExpressionNode) -> Box<Self> {
        Box::new(Self {
            node: node
        })
    }

    pub fn get_node(&self) -> &ExpressionNode {
        &self.node
    }
}

#[derive(Debug)]
pub enum ExpressionNode {
    Primary(PrimaryNode),
    Unary(UnaryNode),
    Binary(BinaryNode)
}

#[derive(Debug)]
pub struct BinaryNode {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: Token,
}

#[derive(Debug)]
pub struct UnaryNode {
    pub left: Box<Expression>,
    pub op: Token,
}

#[derive(Debug)]
pub enum PrimaryNode {
    Literal(LiteralValue),
    Paren(Box<Expression>),
    Identifier(Token),
    Call {
        name: Token,
        args: Vec<Box<Expression>>
    }
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    String(String),
    Number(f64),
}
