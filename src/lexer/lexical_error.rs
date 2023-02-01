use crate::{lexer::token::{
    Token
}, errors::{DescribableError}};

#[derive(Debug)]
pub enum LexicalError {
    UnknownToken{
        lexeme: char,
        token: Token
    },
}

impl DescribableError for LexicalError {
    fn kind(&self) -> String {
        "LexicalError".into()
    }

    fn message(&self) -> String {
        match self {
            LexicalError::UnknownToken {
                lexeme,
                token: _
            } => {
                format!(
                    "Unknown token `{}`",
                    lexeme
                )
            }
        }
    }
}
