use crate::{
    interpreter::{
        runtime_error::{InterpreterException},
        value::Value,
        env::Env,
    },
};

pub trait Executable {
    fn run(&self, env: &mut Env) -> Result<Value, InterpreterException>;
}
