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
pub enum Node {
    Primary(PrimaryNode),
    Unary(UnaryNode),
    Binary(BinaryNode)
}


impl Node {
    pub fn create(
        node: Node,
    ) -> Box<Self> {
        Box::new(node)
    }
}

impl From<Node> for Span {
    fn from(node: Node) -> Span {
        match node {
            Node::Primary(node) => node.into(),
            Node::Unary(node) => node.into(),
            Node::Binary(node) => node.into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryNode {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub op: Token,
}

impl From<BinaryNode> for Span {
    fn from(node: BinaryNode) -> Self {
        let left: Span = node.left.as_ref().clone().into();
        let right: Span = node.right.as_ref().clone().into();

        Span {
            start: left.start,
            end: right.end
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryNode {
    pub left: Box<Node>,
    pub op: Token,
}

impl From<UnaryNode> for Span {
    fn from(node: UnaryNode) -> Span {
        let left: Span = node.left.as_ref().clone().into();
        Span {
            start: node.op.clone(),
            end: left.end
        }
    }
}

#[derive(Debug, Clone)]
pub enum PrimaryNode {
    Literal(Token),
    Paren {
        lparen: Token,
        rparen: Token,
        expr: Box<Node>
    },
    Identifier(Token),
    Call {
        name: Token,
        args: Vec<Box<Node>>,
        rparen: Token
    }
}

impl From<PrimaryNode> for Span {
    fn from(node: PrimaryNode) -> Span {
        match node {
            PrimaryNode::Identifier(token) => Span {
                start: token.clone(),
                end: token.clone()
            },
            PrimaryNode::Literal(token) => Span {
                start: token.clone(),
                end: token.clone()
            },
            PrimaryNode::Paren {
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
