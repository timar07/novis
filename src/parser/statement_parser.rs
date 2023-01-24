use crate::{
    lexer::token::{Token, TokenTag},
    errors::DebugInfo
};

use crate::parser::{
    ast::statement::Statement,
    token_stream::TokenStream,
    parse_error::ParseError
};

pub struct StatementParser {
    tokens: &mut TokenStream,
}

impl StatementParser {
    pub fn new(tokens: TokenStream) -> StatementParser {
        StatementParser {
            tokens: tokens
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Vec<ParseError>> {
        let mut statements: Vec<Statement> = vec![];
        let mut errors: Vec<ParseError> = vec![];

        while self.tokens.current().is_some() {
            match self.statement() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => errors.push(error)
            };
        };

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.tokens.accept().unwrap();

        match token.tag {
            TokenTag::Print => self.print(),
            _ => return Err(ParseError {
                token: token,
                msg: "Expected statement".to_string()
            })
        }
    }

    fn print(&mut self) -> Result<Statement, ParseError> {
        Ok(Statement::Print {
            value: match self.tokens.current() {
                Some(val) => val,
                None => panic!("Unexpected EOF")
            },
            keyword: self.tokens.prev().unwrap()
        })
    }
}
