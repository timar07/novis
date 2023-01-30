use crate::{
    lexer::token::{
        Token
    },
    errors::{
        DescribableError,
        print_error
    }
};

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub msg: String,
}

impl DescribableError for ParseError {
    fn print(&self) -> () {
        print_error(
            "Parse Error",
            self.msg.clone(),
            self.token.info.clone()
        );
    }
}
