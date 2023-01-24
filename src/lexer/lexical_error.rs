use crate::{lexer::token::{
    Token
}, errors::{DescribableError, print_error}};

pub struct LexicalError {
    pub token: Token,
    pub msg: String,
}

impl DescribableError for LexicalError {
    fn print(&self) -> () {
        print_error(
            "Lexical Error",
            self.msg.clone(),
            self.token.info.clone()
        );
    }
}
