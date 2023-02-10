use std::fmt::Debug;

use crate::{
    lexer::token::{
        Token
    },
    errors::{
        Span
    }
};

#[derive(Debug, Clone)]
pub struct Expression {
    node: ExpressionNode,
}

impl Expression {
    pub fn create(
        node: ExpressionNode,
    ) -> Box<Self> {
        Box::new(Self {
            node: node,
        })
    }

    pub fn get_node(&self) -> &ExpressionNode {
        &self.node
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Primary(PrimaryNode),
    Unary(UnaryNode),
    Binary(BinaryNode)
}

impl Node for ExpressionNode {
    fn get_span(&self) -> Span {
        match self {
            Self::Primary(node) => node.get_span(),
            Self::Unary(node) => node.get_span(),
            Self::Binary(node) => node.get_span()
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryNode {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: Token,
}

impl Node for BinaryNode {
    fn get_span(&self) -> Span {
        Span {
            start: self.left.get_node().get_span().start,
            end: self.right.get_node().get_span().end
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryNode {
    pub left: Box<Expression>,
    pub op: Token,
}

impl Node for UnaryNode {
    fn get_span(&self) -> Span {
        Span {
            start: self.op.clone(),
            end: self.left.get_node().get_span().end
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrimaryNode {
    Literal(Token),
    Paren {
        lparen: Token,
        rparen: Token,
        expr: Box<Expression>
    },
    Identifier(Token),
    Call {
        name: Token,
        args: Vec<Box<Expression>>,
        rparen: Token
    }
}

impl Node for PrimaryNode {
    fn get_span(&self) -> Span {
        match &self {
            Self::Identifier(token) => Span {
                start: token.clone(),
                end: token.clone()
            },
            Self::Literal(token) => Span {
                start: token.clone(),
                end: token.clone()
            },
            Self::Paren {
                lparen,
                rparen,
                expr: _
            }  => Span {
                start: lparen.clone(),
                end: rparen.clone()
            },
            PrimaryNode::Call {
                name,
                args: _,
                rparen
            } => Span {
                start: name.clone(),
                end: rparen.clone()
            },
        }
    }
}

pub trait Node {
    fn get_span(&self) -> Span;
}
