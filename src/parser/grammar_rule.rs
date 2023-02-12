use super::token_stream::TokenStream;

pub trait GrammarRule<T> {
    fn parse(&self, tokens: &mut TokenStream) -> T;
}
