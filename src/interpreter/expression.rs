use crate::{
    interpreter::env::Env,
    parser::ast::expression::{
        Expression,
        BinaryNode,
        UnaryNode,
        PrimaryNode,
        ExpressionNode, Node
    },
    lexer::token::{TokenTag, Token}, errors::Span
};
use super::{
    runtime_error::InterpreterException::{
        self,
        *
    },
    runtime_error::{
        RuntimeError,
        RuntimeErrorTag::*
    },
    value::Value,
    statement::statement,
};

pub fn expression(env: &mut Env, ast: &Expression) -> Result<Value, InterpreterException> {
    match ast.get_node() {
        ExpressionNode::Binary(bin_expr) => binary(env, &bin_expr),
        ExpressionNode::Unary(un_expr) => unary(env, &un_expr),
        ExpressionNode::Primary(prim_expr) => primary(env, &prim_expr),
    }
}

/// Evaluate binary expression
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
                _ => return Err(Fatal(
                    RuntimeError {
                        span: node.get_span(),
                        tag: IncompatibleOperands {
                            expr: node.clone(),
                            op: node.op.clone()
                        }
                    }
                )),
            }
        },
        TokenTag::Minus => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Number(l - r)
                },
                _ => return Err(Fatal(
                    RuntimeError {
                        span: node.get_span(),
                        tag: IncompatibleOperands {
                            expr: node.clone(),
                            op: node.op.clone()
                        }
                    }
                )),
            }
        },
        TokenTag::Star => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Number(l * r)
                },
                _ => return Err(Fatal(
                    RuntimeError {
                        span: node.get_span(),
                        tag: IncompatibleOperands {
                            expr: node.clone(),
                            op: node.op.clone()
                        }
                    }
                )),
            }
        },
        TokenTag::Circ => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Number(l.powf(r))
                },
                _ => return Err(Fatal(
                    RuntimeError {
                        span: node.get_span(),
                        tag: IncompatibleOperands {
                            expr: node.clone(),
                            op: node.op.clone()
                        }
                    }
                )),
            }
        },
        TokenTag::Slash => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    if r != 0.0 {
                        Value::Number(l / r)
                    } else {
                        return Err(Fatal(
                            RuntimeError {
                                span: node.get_span(),
                                tag: DivisionByZero(node.clone())
                            }
                        ))
                    }
                },
                _ => return Err(Fatal(
                    RuntimeError {
                        span: node.get_span(),
                        tag: IncompatibleOperands {
                            expr: node.clone(),
                            op: node.op.clone()
                        }
                    }
                )),
            }
        },
        TokenTag::EqualEqual => Value::Boolean(left == right),
        TokenTag::BangEqual => Value::Boolean(left != right),
        TokenTag::Less => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Boolean(l < r)
                },
                _ => return Err(Fatal(
                    RuntimeError {
                        span: node.get_span(),
                        tag: IncompatibleOperands {
                            expr: node.clone(),
                            op: node.op.clone()
                        }
                    }
                )),
            }
        },
        TokenTag::Greater => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Boolean(l > r)
                },
                _ => return Err(Fatal(
                    RuntimeError {
                        span: node.get_span(),
                        tag: IncompatibleOperands {
                            expr: node.clone(),
                            op: node.op.clone()
                        }
                    }
                )),
            }
        },
        TokenTag::LessEqual => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Boolean(l <= r)
                },
                _ => return Err(Fatal(
                    RuntimeError {
                        span: node.get_span(),
                        tag: IncompatibleOperands {
                            expr: node.clone(),
                            op: node.op.clone()
                        }
                    }
                )),
            }
        },
        TokenTag::GreaterEqual => {
            match (left, right) {
                (Value::Number(l), Value::Number(r)) => {
                    Value::Boolean(l >= r)
                },
                _ => return Err(Fatal(RuntimeError {
                    span: node.get_span(),
                    tag: IncompatibleOperands {
                        expr: node.clone(),
                        op: node.op.clone()
                    }
                })),
            }
        },
        _ => unreachable!()
    };

    Ok(val)
}

/// Evaluate unary expression
fn unary(env: &mut Env, node: &UnaryNode) -> Result<Value, InterpreterException> {
    let left = expression(env, node.left.as_ref())?;

    match node.op.tag {
        TokenTag::Minus => {
            match left {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(Fatal(
                    RuntimeError {
                        span: node.get_span(),
                        tag: IncompatibleOperand {
                            expr: node.clone(),
                            op: node.op.clone()
                        }
                    }
                ))
            }
        },
        _ => unreachable!()
    }
}

/// Evaluate primary expression
fn primary(env: &mut Env, node: &PrimaryNode) -> Result<Value, InterpreterException> {
    match node {
        PrimaryNode::Literal(token) => literal(token),
        PrimaryNode::Paren {
            lparen: _,
            rparen: _,
            expr
        } => paren(env, expr),
        PrimaryNode::Identifier(token) => identifier(env, token),
        PrimaryNode::Call {
            name,
            args,
            rparen: _
        } => call(name, env, args),
    }
}

/// Evaluate function call
fn call(name: &Token, env: &mut Env, args: &Vec<Box<Expression>>) -> Result<Value, InterpreterException> {
    match &name.tag {
        TokenTag::Identifier(s) => match env.get(&s) {
            Some(Value::Function {
                params,
                name:_,
                body
            }) => {
                let mut global_env = env.to_owned();
                let closure = &mut global_env.enter();

                for i in 0..params.len() {
                    match &params[i].tag {
                        TokenTag::Identifier(name) => {
                            let definition_result = closure.define(
                                &name,
                                expression(&mut env.to_owned(), &args[i])?
                            );

                            match definition_result {
                                Err(err) => {
                                    return Err(InterpreterException::Fatal(err))
                                },
                                _ => ()
                            }
                        },
                        _ => unreachable!()
                    }
                }

                match statement(closure, body.as_ref()) {
                    Err(InterpreterException::Return(value)) => {
                        closure.leave();
                        *env = global_env;
                        return Ok(value);
                    }
                    _ => { Ok(Value::Null) }
                }
            }
            Some(_) => return Err(Fatal(
                RuntimeError {
                    span: Span::from(name.clone()),
                    tag: ObjectIsNotCallable
                }
            )),
            None => return Err(Fatal(
                RuntimeError {
                    span: Span::from(name.clone()),
                    tag: FunctionNotDefined { name: name.get_lexeme() }
                }
            ))
        }
        _ => unreachable!()
    }
}

/// Evaluate identifier
fn identifier(env: &mut Env, token: &Token) -> Result<Value, InterpreterException> {
    match &token.tag {
        TokenTag::Identifier(name) => match env.get(&name) {
            Some(val) => return Ok(val.clone()),
            None => return Err(Fatal(
                RuntimeError {
                    span: Span::from(token.clone()),
                    tag: NameNotDefined {
                        name: name.clone()
                    }
                }
            ))
        },
        _ => unreachable!()
    }
}

/// Evaluate literal value
fn literal(token: &Token) -> Result<Value, InterpreterException> {
    let value = match token.tag.clone() {
        TokenTag::Number(n) => Value::Number(n),
        TokenTag::String(s) => Value::String(Box::new(s.into())),
        _ => unreachable!()
    };

    Ok(value)
}

/// Evaluate parenthesized expression
fn paren(env: &mut Env, expr: &Box<Expression>) -> Result<Value, InterpreterException> {
    Ok(expression(env, expr)?)
}
