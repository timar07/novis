use super::token_stream::TokenStream;
use crate::lexer::token::TokenTag;


/// Synchronization routine
pub fn sync(tokens: &mut TokenStream) {
    while tokens.current().tag != TokenTag::EndOfFile {
        if tokens.prev().tag == TokenTag::Semicolon {
            return ();
        }

        match tokens.next().tag {
            TokenTag::If
            | TokenTag::Print
            | TokenTag::Let
            | TokenTag::Func => return (),
            _ => {
                tokens.accept();
                dbg!(tokens.current());
            }
        }
    }
}
