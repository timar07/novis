use crate::{
    parser::ast::expression::{ExpressionNode, Expression},
    interpreter::{
        runtime_error::{
            InterpreterException,
        },
        value::Value,
        env::Env,
    }
};

use super::eval_trait::Evaluatable;

impl Evaluatable for Expression {
    fn eval(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        self.eval(env)
    }
}

impl Evaluatable for ExpressionNode {
    fn eval(&self, env: &mut Env) -> Result<Value, InterpreterException> {
        match self {
            ExpressionNode::Primary(primary) => primary.eval(env),
            ExpressionNode::Unary(unary) => unary.eval(env),
            ExpressionNode::Binary(binary) => binary.eval(env),
        }
    }
}

