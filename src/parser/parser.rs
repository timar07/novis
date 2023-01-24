use crate::lexer::token::Token;

use super::{
    statement_parser::StatementParser,
    token_stream::TokenStream,
    ast::statement::Statement,
    parse_error::ParseError
};

pub struct Parser {
    tokens: TokenStream
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: TokenStream::new(tokens),
        }
    }
    pub fn parse(&mut self) -> Result<Vec<Statement>, Vec<ParseError>> {
        StatementParser::new(self.tokens.clone())
            .parse()
    }
}
