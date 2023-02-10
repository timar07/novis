use crate::{
    lexer::token::{
        Token, TokenTag
    },
    errors::{
        DescribableError, Span
    }
};

#[derive(Debug)]
pub struct ParseError {
    pub token: Token,
    pub tag: ParseErrorTag
}

#[derive(Debug, Clone)]
pub enum ParseErrorTag {
    UnexpectedToken,
    ExpectedToken(TokenTag),
    ExpectedSemicolon,
    ExpectedIdentifier,
    ExpectedExpression
}

impl Into<String> for ParseErrorTag {
    fn into(self) -> String {
        match self {
            Self::UnexpectedToken => {
                format!(
                    "unexpected token",
                )
            },
            Self::ExpectedExpression => {
                format!(
                    "expected expression",
                )
            },
            Self::ExpectedToken(token) => {
                format!(
                    "expected {:?}",
                    token,
                )
            },
            Self::ExpectedIdentifier => {
                format!(
                    "expected identifier",
                )
            },
            Self::ExpectedSemicolon => {
                format!(
                    "expected semicolon",
                )
            }
        }
    }
}

impl DescribableError for ParseError {
    fn kind(&self) -> String {
        "ParseError".into()
    }

    fn print_snippet(&self) {
        eprintln!("{}", Span::from(self.token.clone()))
    }

    fn message(&self) -> String {
        self.tag.clone().into()
    }
}

