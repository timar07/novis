use crate::errors::DebugInfo;

#[derive(Debug, Clone)]
pub struct Token {
    pub tag: TokenTag,
    pub lexeme: Lexeme,
    pub info: DebugInfo
}

impl Token {
    pub fn get_lexeme(&self) -> String {
        self.info.src[self.lexeme.start..self.lexeme.end].to_string()
    }
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum TokenTag {
    // One character long
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    Dot,
    Comma,
    Circ,
    Semicolon,
    Less,
    Greater,
    Equal,
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    // Two character long
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
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
    Let,
    If,
    Else,
    Loop,
    Repeat,
    Return,
    // Other
    Number(f64),
    String(String),
    Identifier(String),
    Error,
    EndOfFile
}

#[derive(Debug, Clone, Copy)]
pub struct Lexeme {
    pub start: usize,
    pub end: usize,
}
