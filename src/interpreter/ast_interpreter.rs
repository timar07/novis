use std::process;

use crate::{
    parser::ast::expression::{
        Expression,
        BinaryNode,
        UnaryNode,
        PrimaryNode
    },
    lexer::token::TokenTag
};

use super::runtime_error::RuntimeError;

pub struct AstInterpreter {
    ast: Box<Expression>
}

impl AstInterpreter {
    pub fn new(ast: Box<Expression>) -> AstInterpreter {
        AstInterpreter { ast: ast }
    }

    pub fn interpret(&self) -> f64 {
        match self.expression(self.ast.as_ref()) {
            Ok(val) => val,
            Err(msg) => {
                msg.print();
                process::exit(1);
            }
        }
    }

    pub fn expression(&self, ast: &Expression) -> Result<f64, RuntimeError> {
        match ast {
            Expression::Binary(bin_expr) => self.binary(bin_expr),
            Expression::Unary(un_expr) => self.unary(un_expr),
            Expression::Primary(prim_expr) => self.primary(prim_expr),
        }
    }

    fn binary(&self, node: &BinaryNode) -> Result<f64, RuntimeError> {
        let left: f64 = self.expression(node.left.as_ref())?;
        let right: f64 = self.expression(node.right.as_ref())?;

        let val = match node.op.tag {
            TokenTag::Plus => left + right,
            TokenTag::Minus => left - right,
            TokenTag::Star => left * right,
            TokenTag::Circ => left.powf(right),
            TokenTag::Slash => {
                if right == 0.0 {
                    return Err(RuntimeError {
                        msg: String::from("Division by zero"),
                        info: node.op.info.clone()
                    })
                }

                left / right
            },
            _ => return Err(RuntimeError {
                msg: format!("Unknown binary operator {:?}", node.op.tag),
                info: node.op.info.clone()
            })
        };

        Ok(val)
    }

    fn unary(&self, node: &UnaryNode) -> Result<f64, RuntimeError> {
        let left: f64 = self.expression(node.left.as_ref())?;

        match node.op.tag {
            TokenTag::Minus => Ok(-left),
            _ => Err(RuntimeError {
                msg: format!("Invalid unary operator {:?}", node.op.tag),
                info: node.op.info.clone()
            })
        }
    }

    fn primary(&self, node: &PrimaryNode) -> Result<f64, RuntimeError> {
        let val = match node {
            PrimaryNode::Literal(literal) => *literal,
            PrimaryNode::Paren(expr) => self.expression(expr)?,
        };

        Ok(val)
    }
}