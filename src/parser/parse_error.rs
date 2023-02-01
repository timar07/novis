use crate::{
    lexer::token::{
        Token, TokenTag
    },
    errors::{
        DescribableError
    }
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken{
        token: Token
    },
    ExpectedToken{
        expected: TokenTag,
        token: Token
    },
    ExpectedSemicolon {
        token: Token
    },
    ExpectedIdentifier{
        token: Token
    },
    ExpectedExpression{
        token: Token
    },
}

impl DescribableError for ParseError {
    fn kind(&self) -> String {
        "ParseError".into()
    }

    fn message(&self) -> String {
        match self {
            ParseError::UnexpectedToken{token} => {
                format!(
                    "unexpected token {:?}",
                    token.tag
                )
            },
            ParseError::ExpectedExpression{token} => {
                format!(
                    "expected expression, got {:?}",
                    token.tag
                )
            },
            ParseError::ExpectedToken { expected, token } => {
                format!(
                    "expected {:?}, got {:?}",
                    expected,
                    token.tag
                )
            },
            ParseError::ExpectedIdentifier { token } => {
                format!(
                    "expected identifier, got {:?}",
                    token.tag
                )
            },
            ParseError::ExpectedSemicolon { token } => {
                format!(
                    "expected semicolon, got {:?}",
                    token.tag
                )
            }
        }
    }
}

