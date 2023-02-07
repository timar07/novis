use crate::{
    errors::{
        DescribableError
    },
    lexer::token::Token, parser::ast::expression::{
        BinaryNode,
        Node
    }
};

use super::value::Value;

#[derive(Debug)]
pub enum InterpreterException {
    Fatal(RuntimeError),
    Return(Value)
}

#[derive(Debug)]
pub enum RuntimeError {
    IncompatibleOperands {
        expr: BinaryNode,
        op: Token,
    },
    DivisionByZero(BinaryNode),
    ReturnOutOfFunction,
    ObjectIsNotCallable,
    ConversionError {
        from: String,
        to: String
    },
    FunctionNotDefined {
        name: String
    },
    NameNotDefined {
        name: String
    },
    #[allow(dead_code)]
    FunctionRedefinition {
        name: String
    },
    NameRedefinition {
        name: String
    }
}

impl DescribableError for RuntimeError {
    fn kind(&self) -> String {
        "RuntimeError".into()
    }

    fn print_snippet(&self) {
        let span = match self {
            RuntimeError::DivisionByZero(expr) => {
                expr.get_span()
            },
            RuntimeError::IncompatibleOperands {
                expr,
                op
            } => {
                expr.get_span()
            }
            _ => todo!()
        };

        eprintln!("{}", span);
    }

    fn message(&self) -> String {
        match self {
            RuntimeError::DivisionByZero(_) => {
                format!("Division by zero")
            },
            RuntimeError::ObjectIsNotCallable => {
                format!("Object is not callable")
            }
            RuntimeError::ReturnOutOfFunction => {
                format!("Cannot return value outside of the function")
            },
            RuntimeError::ConversionError {
                from,
                to
            } => {
                format!("Cannot convert value of type `{from}` to `{to}`")
            },
            RuntimeError::FunctionNotDefined { name } => {
                format!("Function `{}` not defined", name)
            },
            RuntimeError::NameNotDefined { name } => {
                format!("Name `{}` not defined", name)
            },
            RuntimeError::IncompatibleOperands { expr: _, op } => {
                format!("Cannot perform `{:?}` between operands", op.tag)
            },
            RuntimeError::FunctionRedefinition { name } => {
                format!("Function `{}` is already defined", name)
            },
            RuntimeError::NameRedefinition { name } => {
                format!("Name `{}` is already defined", name)
            },
        }
    }
}
