use crate::{
    errors::{
        DescribableError
    },
    lexer::token::Token
};

#[derive(Debug)]
pub enum RuntimeError {
    IncompatibleOperands {
        op: Token,
    },
    DivisionByZero,

    FunctionNotDefined {
        name: String
    },
    NameNotDefined {
        name: String
    },
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

    fn message(&self) -> String {
        match self {
            RuntimeError::DivisionByZero => {
                format!("Division by zero")
            },
            RuntimeError::FunctionNotDefined { name } => {
                format!("Function `{}` not defined", name)
            },
            RuntimeError::NameNotDefined { name } => {
                format!("Name `{}` not defined", name)
            },
            RuntimeError::IncompatibleOperands { op } => {
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
