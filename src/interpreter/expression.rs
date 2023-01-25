use crate::{
    interpreter::env::Env,
    parser::ast::expression::{
        Expression,
        BinaryNode,
        UnaryNode,
        PrimaryNode
    },
    lexer::token::TokenTag
};

use super::runtime_error::RuntimeError;

pub fn expression(env: &mut Env, ast: &Expression) -> Result<f64, RuntimeError> {
    match ast {
        Expression::Binary(bin_expr) => binary(env, bin_expr),
        Expression::Unary(un_expr) => unary(env, un_expr),
        Expression::Primary(prim_expr) => primary(env, prim_expr),
    }
}

fn binary(env: &mut Env, node: &BinaryNode) -> Result<f64, RuntimeError> {
    let left: f64 = expression(env, node.left.as_ref())?;
    let right: f64 = expression(env, node.right.as_ref())?;

    let val = match node.op.tag {
        TokenTag::Plus => left + right,
        TokenTag::Minus => left - right,
        TokenTag::Star => left * right,
        TokenTag::Circ => left.powf(right),
        TokenTag::EqualEqual => if left == right { 1.0 } else { 0.0 },
        TokenTag::BangEqual => if left != right { 1.0 } else { 0.0 },
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

fn unary(env: &mut Env, node: &UnaryNode) -> Result<f64, RuntimeError> {
    let left: f64 = expression(env, node.left.as_ref())?;

    match node.op.tag {
        TokenTag::Minus => Ok(-left),
        _ => Err(RuntimeError {
            msg: format!("Invalid unary operator {:?}", node.op.tag),
            info: node.op.info.clone()
        })
    }
}

fn primary(env: &mut Env, node: &PrimaryNode) -> Result<f64, RuntimeError> {
    let val = match node {
        PrimaryNode::Literal(literal) => *literal,
        PrimaryNode::Paren(expr) => expression(env, expr)?,
        PrimaryNode::Identifier(token) => match &token.tag {
            TokenTag::Identifier(name) => match env.get(&name) {
                Some(val) => val,
                None => return Err(RuntimeError {
                    msg: format!("`{}` is not defined", *name),
                    info: token.info.clone()
                })
            },
            _ => panic!()
        }
    };

    Ok(val)
}
