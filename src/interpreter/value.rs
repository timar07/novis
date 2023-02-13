use std::{rc::Rc, ops};
use crate::{
    parser::ast::statement::Group,
    lexer::token::Token,
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
        body: Rc<Group>,
    },
    Null,
}

impl ops::Add<Value> for Value {
    type Output = Result<Value, ()>;

    fn add(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => {
                Ok(Value::Number(l + r))
            },
            (Value::String(l), Value::String(r)) => {
                Ok(Value::String(
                    Box::new(l.as_ref().clone() + r.as_ref())
                ))
            },
            (Value::Number(l), Value::String(r)) => {
                Ok(Value::String(
                    Box::new(l.to_string() + r.as_ref())
                ))
            },
            (Value::String(l), Value::Number(r)) => {
                Ok(Value::String(
                    Box::new(l.as_ref().clone() + &r.to_string())
                ))
            },
            _ => Err(())
        }
    }
}

impl ops::Sub for Value {
    type Output = Result<Value, ()>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => {
                Ok(Value::Number(l - r))
            },
            _ => Err(()),
        }
    }
}

impl ops::Neg for Value {
    type Output = Result<Value, ()>;

    fn neg(self) -> Self::Output {
        match self {
            Value::Number(n) => Ok(Value::Number(-n)),
            _ => Err(())
        }
    }
}

impl ops::Div for Value {
    type Output =  Result<Value, ()>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => {
                Ok(Value::Number(l / r))
            },
            _ => Err(())
        }
    }
}

impl ops::Mul for Value {
    type Output = Result<Value, ()>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(l), Value::Number(r)) => {
                Ok(Value::Number(l * r))
            },
            _ => Err(()),
        }
    }
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
                        span: todo!(),
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
