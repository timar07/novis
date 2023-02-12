use crate::{
    parser::ast::expression::{UnaryNode, Node},
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

impl Evaluatable for UnaryNode {
    fn eval(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        let left = self.left.eval(env)?;

        match self.op.tag {
            TokenTag::Minus => {
                match left {
                    Value::Number(n) => Ok(Value::Number(-n)),
                    _ => Err(Fatal(
                        RuntimeError {
                            span: self.get_span(),
                            tag: IncompatibleOperand {
                                expr: self.clone(),
                                op: self.op.clone()
                            }
                        }
                    ))
                }
            },
            _ => unreachable!()
        }
    }
}
