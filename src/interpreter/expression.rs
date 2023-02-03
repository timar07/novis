use crate::{
    interpreter::env::Env,
    parser::ast::expression::{
        Expression,
        BinaryNode,
        UnaryNode,
        PrimaryNode,
        LiteralValue
    },
    lexer::token::TokenTag
};
use super::{
    runtime_error::RuntimeError::{
        self,
        *
    },
    value::Value,
    statement::statement
};

type ExpressionValue = Result<Value, RuntimeError>;

pub fn expression(env: &mut Env, ast: &Expression) -> ExpressionValue {
    match ast {
        Expression::Binary(bin_expr) => binary(env, bin_expr),
        Expression::Unary(un_expr) => unary(env, un_expr),
        Expression::Primary(prim_expr) => primary(env, prim_expr),
    }
}

fn binary(env: &mut Env, node: &BinaryNode) -> Result<Value, RuntimeError> {
    let left = expression(env, node.left.as_ref())?;
    let right = expression(env, node.right.as_ref())?;

    let val = match node.op.tag {
        TokenTag::Plus => {
            match (left, right) {
                (Value::Number(r), Value::Number(l)) => {
                    Value::Number(l + r)
                },
                (Value::String(r), Value::String(l)) => {
                    return Ok(
                        Value::String(
                            Box::new(l.as_ref().clone() + r.as_ref())
                        )
                    );
                },
                (Value::Number(l), Value::String(r)) => {
                    Value::String(
                        Box::new(l.to_string() + r.as_ref())
                    )
                },
                (Value::String(l), Value::Number(r)) => {
                    Value::String(
                        Box::new(l.as_ref().clone() + &r.to_string())
                    )
                },
                _ => return Err(IncompatibleOperands {
                    op: node.op.clone()
                }),
            }
        },
        TokenTag::Minus => {
            match (left, right) {
                (Value::Number(r), Value::Number(l)) => {
                    Value::Number(l + r)
                },
                _ => return Err(IncompatibleOperands {
                    op: node.op.clone()
                }),
            }
        },
        TokenTag::Star => {
            match (left, right) {
                (Value::Number(r), Value::Number(l)) => {
                    Value::Number(l + r)
                },
                _ => return Err(IncompatibleOperands {
                    op: node.op.clone()
                }),
            }
        },
        TokenTag::Circ => {
            match (left, right) {
                (Value::Number(r), Value::Number(l)) => {
                    Value::Number(l + r)
                },
                _ => return Err(IncompatibleOperands {
                    op: node.op.clone()
                }),
            }
        },
        TokenTag::Slash => {
            match (left, right) {
                (Value::Number(r), Value::Number(l)) => {
                    if r != 0.0 {
                        Value::Number(l + r)
                    } else {
                        return Err(DivisionByZero)
                    }
                },
                _ => return Err(IncompatibleOperands {
                    op: node.op.clone()
                }),
            }
        },
        TokenTag::EqualEqual => Value::Boolean(left == right),
        TokenTag::BangEqual => Value::Boolean(left != right),
        TokenTag::Less => {
            match (left, right) {
                (Value::Number(r), Value::Number(l)) => {
                    Value::Boolean(l < r)
                },
                _ => panic!()
            }
        },
        TokenTag::Greater => {
            match (left, right) {
                (Value::Number(r), Value::Number(l)) => {
                    Value::Boolean(l > r)
                },
                _ => panic!()
            }
        },
        _ => unreachable!()
    };

    Ok(val)
}

fn unary(env: &mut Env, node: &UnaryNode) -> ExpressionValue {
    let left = expression(env, node.left.as_ref())?;

    match node.op.tag {
        TokenTag::Minus => {
            match left {
                Value::Number(n) => Ok(Value::Number(n)),
                _ => return Err(IncompatibleOperands {
                    op: node.op.clone()
                }),
            }
        },
        _ => unreachable!()
    }
}

fn primary(env: &mut Env, node: &PrimaryNode) -> ExpressionValue {
    match node {
        PrimaryNode::Literal(literal) => {
            let value = match literal {
                LiteralValue::Number(n) => Value::Number(*n),
                LiteralValue::String(str) => Value::String(Box::new(str.into())),
            };

            Ok(value)
        },
        PrimaryNode::Paren(expr) => return Ok(expression(env, expr)?),
        PrimaryNode::Identifier(token) => match &token.tag {
            TokenTag::Identifier(name) => match env.get(&name) {
                Some(val) => return Ok(val.clone()),
                None => return Err(NameNotDefined {
                    name: name.clone()
                })
            },
            _ => unreachable!()
        },
        PrimaryNode::Call {
            name,
            args
        } => match &name.tag {
            TokenTag::Identifier(s) => match env.get(&s) {
                Some(val) => {
                    match val {
                        Value::Function {
                            params: _,
                            name: _,
                            body
                        } => {
                            let mut closure = Env::local(
                                Box::new(env.to_owned())
                            );

                            for _ in 0..args.len() {
                                // closure.define(params[i].tag, expression(env, &args[i])?)
                            }

                            statement(&mut closure, body.as_ref())?;
                            *env = closure.enclosing.unwrap().as_mut().clone();
                        },
                        _ => unreachable!()
                    }
                    return Ok(Value::Null);
                }
                None => return Err(FunctionNotDefined {
                    name: name.to_string()
                })
            }
            _ => unreachable!()
        }
    }
}
