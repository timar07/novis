use std::{rc::Rc};
use crate::{
    parser::ast::statement::Statement,
    lexer::token::Token
};
use super::{
    runtime_error::{
        InterpreterException,
        RuntimeError,
        RuntimeErrorTag::*
    }
};

#[derive(Clone, Debug)]
pub enum Value {
    String(Box<String>),
    Number(f64),
    Boolean(bool),
    Function {
        params: Vec<Token>,
        name: Token,
        body: Rc<Statement>,
    },
    Null,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            _ => false
        }
    }
}

impl Value {
    pub fn to_boolean(&self) -> Result<Value, RuntimeError> {
        match self {
            Value::String(str) => {
                Ok(Value::Boolean(*str.as_ref() != ""))
            },
            Value::Number(n) => {
                Ok(Value::Boolean(*n != 0.0))
            },
            Value::Boolean(_) => Ok(self.clone()),
            _ => todo!()
        }
    }

    pub fn to_string(&self) -> Result<String, InterpreterException> {
        match self {
            Value::String(str) => Ok(format!("{str}")),
            Value::Number(n) => Ok(format!("{n}")),
            Value::Boolean(boolean) => Ok(format!("{boolean}")),
            _ => {
                Err(InterpreterException::Fatal(
                    RuntimeError {
                        tag: ConversionError {
                            from: format!("{:?}", self),
                            to: "String".into(),
                        }
                    }
                ))
            }
        }
    }
}
