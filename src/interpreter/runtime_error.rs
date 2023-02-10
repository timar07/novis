use crate::{
    errors::{
        DescribableError
    },
    lexer::token::Token, parser::ast::expression::{
        BinaryNode,
        Node, UnaryNode
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
    IncompatibleOperand {
        expr: UnaryNode,
        op: Token
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
    NameRedefinition {
        name: String
    }
}

impl DescribableError for RuntimeError {
    fn kind(&self) -> String {
        "RuntimeError".into()
    }

    fn snippet(&self) -> String {
        match self {
            RuntimeError::DivisionByZero(expr) => {
                expr
            },
            RuntimeError::IncompatibleOperands {
                expr,
                op: _
            } => {
                expr
            }
            _ => todo!()
        }.get_span().to_string()
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
            RuntimeError::IncompatibleOperand { expr: _, op } => {
                format!("Cannot perform `{:?}` to the operand", op.tag)
            },
            RuntimeError::NameRedefinition { name } => {
                format!("Name `{}` is already defined", name)
            },
        }
    }
}
