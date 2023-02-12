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

impl From<ExpressionNode> for Span {
    fn from(node: ExpressionNode) -> Span {
        match node {
            ExpressionNode::Primary(primary) => Span::from(primary),
            ExpressionNode::Unary(node) => Span::from(node),
            ExpressionNode::Binary(node) => Span::from(node)
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryNode {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: Token,
}

impl From<BinaryNode> for Span {
    fn from(node: BinaryNode) -> Span {
        Span {
            start: Span::from(*node.left.get_node()).start,
            end: Span::from(*node.right.get_node()).end
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryNode {
    pub left: Box<Expression>,
    pub op: Token,
}

impl From<UnaryNode> for Span {
    fn from(node: UnaryNode) -> Span {
        Span {
            start: node.op.clone(),
            end: Span::from(*node.left.get_node()).end
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
