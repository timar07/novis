use crate::{lexer::token::{
    Token
}, errors::{DescribableError, Span}};

pub struct LexicalError {
    pub token: Token,
    pub tag: LexicalErrorTag
}

#[derive(Debug)]
pub enum LexicalErrorTag {
    UnknownToken,
}

impl DescribableError for LexicalError {
    fn kind(&self) -> String {
        "LexicalError".into()
    }

    fn message(&self) -> String {
        match self.tag {
            LexicalErrorTag::UnknownToken => {
                format!("Unknown token")
            }
        }
    }

    fn print_snippet(&self) {
        eprintln!("{}", Span::from(self.token.clone()));
    }
}
