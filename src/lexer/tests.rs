#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, token::TokenTag::{self, *}};

    #[test]
    fn mixed_tokens() {
        let mut lexer = Lexer::from_string("
        func test_function(a, b) -> {
            return 1.432;
        }

        loop a > b {
            print \"some string\";
            a = a - 1;
        }
        ".into());

        let tokens: Vec<TokenTag> = lexer
            .lex()
            .iter()
            .map(|token| token.tag.clone())
            .collect();

        let expected = vec![
            Func, Identifier("test_function".into()), LeftParen,
            Identifier("a".into()), Comma, Identifier("b".into()),
            RightParen, ArrowRight, LeftCurly,
            Return, Number(1.432), Semicolon, RightCurly,
            Loop, Identifier("a".into()), Greater, Identifier("b".into()),
            LeftCurly, Print, String("some string".into()), Semicolon,
            Identifier("a".into()), Equal, Identifier("a".into()), Minus,
            Number(1.0), Semicolon, RightCurly, EndOfFile
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn nonterminal_tokens() {
        let mut lexer = Lexer::from_string("
        identifier \"string\" 3.14159265358979 1.41
        ".into());

        let tokens: Vec<TokenTag> = lexer
            .lex()
            .iter()
            .map(|token| token.tag.clone())
            .collect();

        let expected = vec![
            Identifier("identifier".into()),
            String("string".into()),
            Number(3.14159265358979),
            Number(1.41),
            EndOfFile
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn keyword_tokens() {
        let mut lexer = Lexer::from_string("
        else if true false func print let loop return
        ".into());

        let tokens: Vec<TokenTag> = lexer
            .lex()
            .iter()
            .map(|token| token.tag.clone())
            .collect();

        let expected = vec![
            Else, If, True, False, Func, Print, Let, Loop, Return,
            EndOfFile
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn two_character_tokens() {
        let mut lexer = Lexer::from_string("
        >= <= += -= *= /= != == -> <-
        ".into());

        let tokens: Vec<TokenTag> = lexer
            .lex()
            .iter()
            .map(|token| token.tag.clone())
            .collect();

        let expected = vec![
            GreaterEqual, LessEqual, PlusEqual, MinusEqual,
            StarEqual, SlashEqual, BangEqual, EqualEqual,
            ArrowRight, ArrowLeft, EndOfFile
        ];

        assert_eq!(tokens, expected);
    }

    #[test]
    fn one_character_tokens() {
        let mut lexer = Lexer::from_string("
        */+- = (){}.,
        ;^!<>
        ".into());

        let tokens: Vec<TokenTag> = lexer
            .lex()
            .iter()
            .map(|token| token.tag.clone())
            .collect();

        let expected = vec![
            Star, Slash, Plus, Minus, Equal,
            LeftParen, RightParen, LeftCurly, RightCurly,
            Dot, Comma, Semicolon, Circ, Bang, Less, Greater,
            EndOfFile
        ];

        assert_eq!(tokens, expected);
    }
}
