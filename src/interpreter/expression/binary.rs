use std::ops::DerefMut;

use crate::{
    parser::ast::expression::{BinaryNode, Node},
    interpreter::{
        runtime_error::{
            InterpreterException::{
                self,
                *
            },
            RuntimeError,
            RuntimeErrorTag::*
        },
        value::Value, env::Env
    }, lexer::token::TokenTag
};

use super::eval_trait::Evaluatable;

impl Evaluatable for BinaryNode {
    fn eval(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        let left = self.left.eval(env)?;
        let right = self.right.eval(env)?;

        let val = match self.op.tag {
            TokenTag::Plus => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        Value::Number(l + r)
                    },
                    (Value::String(l), Value::String(r)) => {
                        Value::String(
                            Box::new(l.as_ref().clone() + r.as_ref())
                        )
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
                            span: self.get_span(),
                            tag: IncompatibleOperands {
                                expr: self.clone(),
                                op: self.op.clone()
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
                            span: self.get_span(),
                            tag: IncompatibleOperands {
                                expr: self.clone(),
                                op: self.op.clone()
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
                            span: self.get_span(),
                            tag: IncompatibleOperands {
                                expr: self.clone(),
                                op: self.op.clone()
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
                            span: self.get_span(),
                            tag: IncompatibleOperands {
                                expr: self.clone(),
                                op: self.op.clone()
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
                                    span: self.get_span(),
                                    tag: DivisionByZero(self.clone())
                                }
                            ))
                        }
                    },
                    _ => return Err(Fatal(
                        RuntimeError {
                            span: self.get_span(),
                            tag: IncompatibleOperands {
                                expr: self.clone(),
                                op: self.op.clone()
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
                            span: self.get_span(),
                            tag: IncompatibleOperands {
                                expr: self.clone(),
                                op: self.op.clone()
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
                            span: self.get_span(),
                            tag: IncompatibleOperands {
                                expr: self.clone(),
                                op: self.op.clone()
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
                            span: self.get_span(),
                            tag: IncompatibleOperands {
                                expr: self.clone(),
                                op: self.op.clone()
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
                        span: self.get_span(),
                        tag: IncompatibleOperands {
                            expr: self.clone(),
                            op: self.op.clone()
                        }
                    })),
                }
            },
            _ => unreachable!()
        };

        Ok(val)
    }
}
