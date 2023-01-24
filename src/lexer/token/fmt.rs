use std::fmt::Display;
use super::Token;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?}:{}:{}>", self.tag, self.info.line, self.info.col)
    }
}