#[cfg(test)]
mod tests {
    use crate::{lexer::{Lexer, token::{TokenTag::{self, *}, Token}}, parser::{Parser, ast::{statement::Statement, expression::{Expression, BinaryNode}}}};

    #[test]
    fn expressions() {
        let mut lexer = Lexer::from_string("2+2*3/4;".into());
        let tokens = lexer.lex();

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
    }
}