use crate::{
    parser::ast::expression::{BinaryNode},
    interpreter::{
        runtime_exception::{
            InterpreterException::{
                self,
                *
            },
            RuntimeError,
            RuntimeErrorTag::*
        },
        value::Value, env::Env
    }, lexer::token::TokenTag, errors::Span
};

use super::evaluatable::Evaluatable;

impl Evaluatable for BinaryNode {
    fn eval(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        let left = self.left.eval(env)?;
        let right = self.right.eval(env)?;

        let val = match self.op.tag {
            TokenTag::Plus => {
                left + right
            },
            TokenTag::Minus => {
                left - right
            },
            TokenTag::Star => {
                left * right
            },
            TokenTag::Circ => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Number(l.powf(r)))
                    },
                    _ => Err(())
                }
            },
            TokenTag::Slash => {
                if right == Value::Number(0.0) {
                    return Err(Fatal(
                        RuntimeError {
                            span: Span::from(self.clone()),
                            tag: DivisionByZero
                        }
                    ))
                }

                left / right
            },
            TokenTag::EqualEqual => Ok(Value::Boolean(left == right)),
            TokenTag::BangEqual => Ok(Value::Boolean(left != right)),
            TokenTag::Less => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l < r))
                    },
                    _ => Err(())
                }
            },
            TokenTag::Greater => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l > r))
                    },
                    _ => Err(())
                }
            },
            TokenTag::LessEqual => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l <= r))
                    },
                    _ => Err(()),
                }
            },
            TokenTag::GreaterEqual => {
                match (left, right) {
                    (Value::Number(l), Value::Number(r)) => {
                        Ok(Value::Boolean(l >= r))
                    },
                    _ => Err(())
                }
            },
            _ => unreachable!()
        };

        match val {
            Err(_) => Err(Fatal(
                RuntimeError {
                    span: Span::from(self.clone()),
                    tag: IncompatibleOperands {
                        expr: self.clone(),
                        op: self.op.clone()
                    }
                }
            )),
            Ok(val) => Ok(val)
        }
    }
}
