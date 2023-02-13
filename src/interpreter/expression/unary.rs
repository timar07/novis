use crate::{
    parser::ast::expression::{UnaryNode},
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
    }, lexer::token::TokenTag, errors::Span
};

use super::evaluatable::Evaluatable;

impl Evaluatable for UnaryNode {
    fn eval(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        let left = self.left.eval(env)?;

        let val =match self.op.tag {
            TokenTag::Minus => {
               -left
            },
            _ => unreachable!()
        };

        match val {
            Err(_) => Err(Fatal(
                RuntimeError {
                    span: Span::from(self.clone()),
                    tag: IncompatibleOperand {
                        expr: self.clone(),
                        op: self.op.clone()
                    }
                }
            )),
            Ok(val) => Ok(val)
        }
    }
}
