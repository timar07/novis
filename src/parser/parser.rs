use crate::{lexer::token::{Token, TokenTag}, errors::DescribableError};
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

        while self.tokens.current().tag != TokenTag::EndOfFile {
            match statement(self.tokens.as_mut()) {
                Ok(statement) => statements.push(statement),
                Err(error) => {
                    self.sync();
                    errors.push(error);
                },
            }
        };

        if !errors.is_empty() {
            for error in errors { error.print() };
            return Err(())
        }

        Ok(statements)
    }

    pub fn sync(&mut self) {
        while self.tokens.current().tag != TokenTag::EndOfFile {
            if self.tokens.prev().tag == TokenTag::Semicolon {
                return ();
            }

            match self.tokens.next().tag {
                TokenTag::If
                | TokenTag::Print
                | TokenTag::Let
                | TokenTag::Func => return (),
                _ => {
                    self.tokens.accept();
                }
            }
        }
    }
}
