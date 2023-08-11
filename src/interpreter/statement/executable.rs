use crate::interpreter::{
    runtime_exception::InterpreterException,
    value::Value,
    env::Env,
};

pub trait Executable {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException>;
}
