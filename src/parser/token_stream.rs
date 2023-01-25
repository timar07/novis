use crate::lexer::token::{Token, TokenTag};

#[derive(Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
    curr: usize
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> TokenStream {
        // dbg!(&tokens);
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
        if tokens.contains(&self.current().tag) {
            self.accept();
            return true
        }

        false
    }

    // Check if the current token matches 'token'
    #[allow(dead_code)]
    fn check(&mut self, token: TokenTag) -> bool {
        self.current().tag == token
    }

    // Consume token
    pub fn accept(&mut self) -> &Token {
        self.advance();
        self.prev()
    }

    pub fn advance(&mut self) {
        self.curr += 1;
    }

    pub fn prev(&self) -> &Token {
        self.nth(self.curr - 1)
    }

    #[allow(dead_code)]
    pub fn next(&self) -> &Token {
        self.nth(self.curr + 1)
    }

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
