use crate::{lexer::token::Token, errors::DescribableError};
use super::{
    token_stream::TokenStream,
    ast::statement::Statement,
    statement::statement
};


pub struct Parser {
    pub tokens: Box<TokenStream>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: Box::new(TokenStream::new(tokens)),
        }
    }
    pub fn parse(&mut self) -> Result<Vec<Statement>, ()> {
        let mut statements = vec![];
        let mut errors = vec![];

        while !self.tokens.is_at_end() {
            match statement(self.tokens.as_mut()) {
                Ok(statement) => statements.push(statement),
                Err(error) => errors.push(error),
            }
        };

        if errors.is_empty() {
            Ok(statements)
        } else {
            for error in errors { error.print() };
            Err(())
        }
    }
}
