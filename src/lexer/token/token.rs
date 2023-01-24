use crate::errors::DebugInfo;

#[derive(Debug, Clone)]

pub struct Token {
    pub tag: TokenTag,
    pub lexeme: Lexeme,
    pub info: DebugInfo
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenTag {
    // One character long
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    Comma,
    Circ,
    Semicolon,
    Less,
    Greater,
    Equal,
    LeftParen,
    RightParen,
    // Two character long
    ArrowLeft,
    ArrowRight,
    EqualEqual,
    BangEqual,
    LessEqual,
    GreaterEqual,
    // Keywords
    False,
    True,
    Func,
    Print,
    // Other
    Number(f64),
    String(String),
    Identifier,
    Error,
    EndOfFile
}

#[derive(Debug, Clone, Copy)]
pub struct Lexeme {
    pub start: usize,
    pub end: usize,
}
