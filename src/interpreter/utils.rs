use crate::parser::ast::expression::Expression;
use super::{
    env::Env,
    value::Value,
    expression::Evaluatable,
    runtime_error::{
        InterpreterException
    }
};


pub fn check_condition(
    env: &mut Env,
    condition: &Box<Expression>
) -> Result<bool, InterpreterException> {
    Ok(condition.eval(env)?.to_boolean().unwrap() == Value::Boolean(true))
}
