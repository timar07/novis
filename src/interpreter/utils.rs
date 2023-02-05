use crate::parser::ast::expression::Expression;
use super::{
    env::Env,
    value::Value,
    expression::expression,
    runtime_error::{RuntimeError, InterpreterException}
};



pub fn check_condition(
    env: &mut Env,
    condition: &Box<Expression>
) -> Result<bool, InterpreterException> {
    Ok(expression(env, condition)?.to_boolean().unwrap() == Value::Boolean(true))
}