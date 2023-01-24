use std::io;
use std::rc::Rc;
use colored::Colorize;
use crate::errors::{DescribableError, DebugInfo};
use crate::lexer::token::{
    Token,
    TokenTag,
    Lexeme
};

use super::lexical_error::LexicalError;

pub struct Lexer {
    pub src: Rc<String>,
    pub curr: usize,
    pub start: usize,
    pub col: usize,
    pub line: usize,
}

impl Lexer {
    pub fn new(src: &String) -> Result<Lexer, io::Error> {
        Ok(Lexer {
            src: Rc::new(src.clone()),
            line: 1,
            col: 0,
            curr: 0,
            start: 0,
        })
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        loop  {
            match self.lex_token() {
                Ok(token) => match token.tag {
                    TokenTag::EndOfFile => break,
                    _ => {
                        println!("{token}");
                        tokens.push(token)
                    }
                },
                Err(error) => error.print(),
            }
        }

        tokens
    }

    fn lex_token(&mut self) -> Result<Token, LexicalError> {
        self.start = self.curr;

        if let Some(ch) = self.accept() {
            let token_tag = match ch {
                ' ' | '\n' => return self.lex_token(),
                '(' => TokenTag::LeftParen,
                ')' => TokenTag::RightParen,
                '+' => TokenTag::Plus,
                '*' => TokenTag::Star,
                '/' => TokenTag::Slash,
                '.' => TokenTag::Dot,
                ',' => TokenTag::Comma,
                ';' => TokenTag::Semicolon,
                '^' => TokenTag::Circ,
                '-' => {
                    if self.match_next('>') {
                        TokenTag::ArrowRight
                    } else {
                        TokenTag::Minus
                    }
                },
                '<' => {
                    if self.match_next('-') {
                        TokenTag::ArrowLeft
                    } else if self.match_next('=') {
                        TokenTag::LessEqual
                    } else {
                        TokenTag::Less
                    }
                },
                '>' => {
                    if self.match_next('=') {
                        TokenTag::GreaterEqual
                    } else {
                        TokenTag::Greater
                    }
                },
                '=' => {
                    if self.match_next('=') {
                        TokenTag::EqualEqual
                    } else {
                        TokenTag::Equal
                    }
                },
                '"' => {
                    while self.accept().unwrap() != '"' {
                        if self.current().is_none() {
                            panic!("Unterminated string");
                        }
                    };

                    TokenTag::String(String::from(""))
                },
                '0'..='9' => {
                    let mut number: f64 = f64::from(ch.to_digit(10).unwrap());

                    while let Some(digit) = self.current() {
                        if !digit.is_ascii_digit() {
                            break;
                        }

                        number *= 10.0;
                        number += f64::from(self.accept().unwrap().to_digit(10).unwrap());
                    };

                    TokenTag::Number(number)
                },

                // Keywords
                'f' => {
                    if self.match_word("unc") {
                        TokenTag::Func
                    } else if self.match_word("alse") {
                        TokenTag::False
                    } else {
                        TokenTag::Identifier
                    }
                },

                't' => {
                    if self.match_word("rue") {
                        TokenTag::True
                    } else {
                        TokenTag::Identifier
                    }
                },

                'p' => {
                    if self.match_word("rint") {
                        TokenTag::Print
                    } else {
                        TokenTag::Identifier
                    }
                }

                // Identifiers
                'A'..='Z' | 'a'..='z' | '_' => {
                    while let Some(ch) = self.current() {
                        if !self.is_next_identifier_char(ch) {
                            break;
                        }

                        self.accept();
                    };

                    TokenTag::Identifier
                },
                _ => return Err(LexicalError {
                    token: self.create_token(TokenTag::Error),
                    msg: format!(
                        "Unexpected token '{}'",
                        String::from(ch).underline()
                    )
                })
            };

            Ok(self.create_token(token_tag))
        } else {
            Ok(self.create_token(TokenTag::EndOfFile))
        }
    }

    fn create_token(&self, tag: TokenTag) -> Token {
        Token {
            tag: tag,
            lexeme: Lexeme {
                start: self.start,
                end: self.curr
            },
            info: DebugInfo {
                line: self.line,
                col: self.col,
                len: self.curr-self.start,
                src: self.src.clone()
            }
        }
    }

    fn match_word(&mut self, word: &'static str) -> bool {
        if self.check_word(word) {
            self.curr += word.len();
            return true;
        }

        false
    }

    fn check_word(&mut self, word: &'static str) -> bool {
        &self.src[self.curr..=word.len()] == String::from(word)
    }

    fn match_next(&mut self, ch: char) -> bool {
        let next = self.current();
        if next.is_some() && next.unwrap() == ch {
            self.accept();
            return true;
        }

        false
    }

    fn accept(&mut self) -> Option<char> {
        let ch = self.current();

        if ch.is_some() {
            self.curr += 1;

            if ch.unwrap() == '\n' {
                self.line += 1;
                self.col = 0;
            }

            self.col += 1;
        }

        return ch;
    }

    fn current(&self) -> Option<char> {
        self.src.chars().nth(self.curr)
    }

    fn is_next_identifier_char(&self, ch: char) -> bool {
        self.is_identifier_char(ch) || ch.is_ascii_digit()
    }

    fn is_identifier_char(&self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }
}
