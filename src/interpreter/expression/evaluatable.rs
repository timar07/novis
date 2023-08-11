use crate::interpreter::{
    runtime_exception::InterpreterException,
    value::Value, env::Env
};

pub trait Evaluatable {
    fn eval(&self, env: &mut Env) -> Result<Value, InterpreterException>;
}
