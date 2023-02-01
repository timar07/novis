use crate::lexer::token::{Token, TokenTag};

use super::parse_error::ParseError;

#[derive(Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
    curr: usize
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> TokenStream {
        TokenStream { tokens: tokens, curr: 0 }
    }

    /// Check if the next token matches one of the `tokens`,
    /// if it's not, returns `ParseError`
    pub fn require(
        &mut self,
        tokens: &'static [TokenTag],
    ) -> Result<&Token, ParseError> {
        if self.match_next(tokens) {
            return Ok(self.prev());
        }

        Err(ParseError::ExpectedToken {
            expected: tokens[0].clone(),
            token: self.current().clone(),
        })
    }

    /// All the same as previous, but consumes the token
    /// if it matches
    pub fn match_next(&mut self, tokens: &'static [TokenTag]) -> bool {
        if tokens.contains(&self.current().tag) {
            self.accept();
            return true
        }

        false
    }

    /// Check if the current token matches 'token'
    #[allow(dead_code)]
    fn check(&mut self, token: TokenTag) -> bool {
        self.current().tag == token
    }

    /// Consume token
    pub fn accept(&mut self) -> &Token {
        self.skip();
        self.prev()
    }

    /// Skip current token
    pub fn skip(&mut self) {
        self.curr += 1;
    }

    /// Push token back
    pub fn discard(&mut self) {
        self.curr -= 1;
    }

    /// Get previous token
    pub fn prev(&self) -> &Token {
        self.nth(self.curr - 1)
    }

    /// Get nth token after current
    pub fn lookahead(&self, n: usize) -> &Token {
        self.nth(self.curr + n)
    }

    /// Get current token
    pub fn current(&self) -> &Token {
        self.nth(self.curr)
    }

    fn nth(&self, n: usize) -> &Token {
        let mut iter = self.tokens.iter();

        match iter.nth(n) {
            Some(token) => token,
            None => {
                self.prev()
            },
        }
    }
}
