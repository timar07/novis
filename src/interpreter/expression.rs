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
    runtime_error::InterpreterException::{
        self,
        *
    },
    runtime_error::RuntimeError::{
        self,
        *
    },
    value::Value,
    statement::statement
};

type ExpressionValue = Result<Value, InterpreterException>;

pub fn expression(env: &mut Env, ast: &Expression) -> ExpressionValue {
    match ast {
        Expression::Binary(bin_expr) => binary(env, bin_expr),
        Expression::Unary(un_expr) => unary(env, un_expr),
        Expression::Primary(prim_expr) => primary(env, prim_expr),
    }
}

fn binary(env: &mut Env, node: &BinaryNode) -> Result<Value, InterpreterException> {
    let left = expression(env, node.left.as_ref())?;
    let right = expression(env, node.right.as_ref())?;

    let val = match node.op.tag {
        TokenTag::Plus => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Number(l + r)
                },
                (Value::String(l), Value::String(r)) => {
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
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
            }
        },
        TokenTag::Minus => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Number(l - r)
                },
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
            }
        },
        TokenTag::Star => {
            match (left, right) {
                (Value::Number(r), Value::Number(l)) => {
                    Value::Number(l + r)
                },
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
            }
        },
        TokenTag::Circ => {
            match (left, right) {
                (Value::Number(r), Value::Number(l)) => {
                    Value::Number(l + r)
                },
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
            }
        },
        TokenTag::Slash => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    if r != 0.0 {
                        Value::Number(l + r)
                    } else {
                        return Err(Fatal(DivisionByZero))
                    }
                },
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
            }
        },
        TokenTag::EqualEqual => Value::Boolean(left == right),
        TokenTag::BangEqual => Value::Boolean(left != right),
        TokenTag::Less => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Boolean(l < r)
                },
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
            }
        },
        TokenTag::Greater => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Boolean(l > r)
                },
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
            }
        },
        TokenTag::LessEqual => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Boolean(l <= r)
                },
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
            }
        },
        TokenTag::GreaterEqual => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Boolean(l >= r)
                },
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
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
                _ => return Err(Fatal(IncompatibleOperands {
                    op: node.op.clone()
                })),
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
                None => return Err(Fatal(NameNotDefined {
                    name: name.clone()
                }))
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
                            params,
                            name: _,
                            body
                        } => {
                            let mut global_env = env.to_owned();
                            let closure = &mut global_env.enter();

                            for i in 0..params.len() {
                                match &params[i].tag {
                                    TokenTag::Identifier(name) => {
                                        closure.define(
                                            &name,
                                            expression(&mut env.to_owned(), &args[i])?
                                        );
                                    },
                                    _ => unreachable!()
                                }
                            }

                            match statement(closure, body.as_ref()) {
                                Err(exception) => {
                                    match exception {
                                        InterpreterException::Return(value) => {
                                            closure.leave();
                                            *env = global_env;
                                            return Ok(value);
                                        },
                                        _ => panic!()
                                    }
                                }
                                Ok(_) => todo!(),
                            }
                        },
                        _ => unreachable!()
                    }
                    return Ok(Value::Null);
                }
                None => return Err(Fatal(FunctionNotDefined {
                    name: name.to_string()
                }))
            }
            _ => unreachable!()
        }
    }
}
