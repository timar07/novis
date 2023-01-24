use crate::lexer::token::{Token, TokenTag};

#[derive(Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
    curr: usize
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> TokenStream {
        TokenStream { tokens: tokens, curr: 0 }
    }

    pub fn require(&mut self, tokens: &'static [TokenTag]) -> Result<(), ()> {
        if self.match_next(tokens) {
            return Ok(());
        }

        Err(())
    }

    // All the same as previous, but consumes the token
    // if it matches
    pub fn match_next(&mut self, tokens: &'static [TokenTag]) -> bool {
        if let Some(current) = self.current() {
            if tokens.contains(&current.tag) {
                self.accept();
                return true
            }
        }

        false
    }

    // Check if the current token matches 'token'
    #[allow(dead_code)]
    fn check(&mut self, token: TokenTag) -> bool {
        if self.current().is_none() {
            return false;
        }

        self.current().unwrap().tag == token
    }

    // Consume token
    pub fn accept(&mut self) -> Option<Token> {
        let token = self.current();
        self.curr += 1;
        token
    }

    pub fn prev(&self) -> Option<Token> {
        if self.is_at_end() { return None; }

        Some(self.tokens[self.curr - 1].clone())
    }

    #[allow(dead_code)]
    pub fn next(&self) -> Option<Token> {
        if self.is_at_end() { return None; }

        Some(self.tokens[self.curr + 1].clone())
    }

    pub fn current(&self) -> Option<Token> {
        if self.is_at_end() { return None; }

        Some(self.tokens[self.curr].clone())
    }

    pub fn is_at_end(&self) -> bool {
        self.curr >= self.tokens.len()
    }
}
