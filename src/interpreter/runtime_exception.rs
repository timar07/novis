use crate::{
    errors::{
        DescribableError, Span
    },
    lexer::token::Token, parser::ast::expression::{
        BinaryNode,
        UnaryNode
    }
};

use super::value::Value;

#[derive(Debug)]
pub enum InterpreterException {
    Fatal(RuntimeError),
    Return(ReturnValue)
}

#[derive(Debug)]
pub struct ReturnValue {
    pub val: Value,
    pub span: Span
}

#[derive(Debug)]
pub struct RuntimeError {
    pub span: Span,
    pub tag: RuntimeErrorTag
}

#[derive(Debug)]
pub enum RuntimeErrorTag {
    IncompatibleOperands {
        expr: BinaryNode,
        op: Token,
    },
    IncompatibleOperand {
        expr: UnaryNode,
        op: Token
    },
    DivisionByZero,
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

impl RuntimeErrorTag {
    pub fn to_human_readable(&self) -> String {
        match self {
            Self::DivisionByZero => {
                format!("Division by zero")
            },
            Self::ObjectIsNotCallable => {
                format!("Object is not callable")
            }
            Self::ReturnOutOfFunction => {
                format!("Cannot return value outside of the function")
            },
            Self::ConversionError {
                from,
                to
            } => {
                format!("Cannot convert value of type `{from}` to `{to}`")
            },
            Self::FunctionNotDefined { name } => {
                format!("Function `{}` not defined", name)
            },
            Self::NameNotDefined { name } => {
                format!("Name `{}` not defined", name)
            },
            Self::IncompatibleOperands { expr: _, op } => {
                format!("Cannot perform `{:?}` between operands", op.tag)
            },
            Self::IncompatibleOperand { expr: _, op } => {
                format!("Cannot perform `{:?}` to the operand", op.tag)
            },
            Self::NameRedefinition { name } => {
                format!("Name `{}` is already defined", name)
            },
        }
    }
}

impl DescribableError for RuntimeError {
    fn kind(&self) -> String {
        "RuntimeError".into()
    }

    fn snippet(&self) -> String {
        self.span.to_string()
    }

    fn message(&self) -> String {
        self.tag.to_human_readable()
    }
}
