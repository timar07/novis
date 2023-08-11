use std::rc::Rc;
use crate::{
    file_stream::FileStream,
    errors::DebugInfo,
    lexer::token::{
        Token,
        TokenTag,
        Lexeme
    }
};
use super::lexical_error::{LexicalError, LexicalErrorTag::*};

pub struct Lexer {
    pub src: Rc<String>,
    pub curr: usize,
    pub start: usize,
    pub line: usize,
    pub col: usize,
    pub fname: String,
}

impl Lexer {
    pub fn from_file(path: &String) -> Self {
        Self {
            src: Rc::new(FileStream::new(path).as_str()),
            curr: 0,
            start: 0,
            line: 1,
            col: 0,
            fname: path.clone()
        }
    }

    #[allow(dead_code)]
    pub fn from_string(string: String) -> Self {
        Self {
            src: Rc::new(string),
            curr: 0,
            start: 0,
            line: 1,
            col: 0,
            fname: "unnamed".into()
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Vec<LexicalError>> {
        let mut tokens: Vec<Token> = vec![];
        let mut errors: Vec<LexicalError> = vec![];

        loop  {
            match self.lex_token() {
                Ok(token) => {
                    tokens.push(token.clone());

                    if token.tag == TokenTag::EndOfFile {
                        break;
                    }
                 }
                Err(error) => {
                    errors.push(error)
                }
            }
        }

        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(errors)
        }
    }

    fn lex_token(&mut self) -> Result<Token, LexicalError> {
        self.start = self.curr;

        if let Some(ch) = self.accept() {
            let token_tag = match ch {
                ' ' | '\n' => return self.lex_token(),
                '(' => TokenTag::LeftParen,
                ')' => TokenTag::RightParen,
                '{' => TokenTag::LeftCurly,
                '}' => TokenTag::RightCurly,
                '.' => TokenTag::Dot,
                ',' => TokenTag::Comma,
                ';' => TokenTag::Semicolon,
                '^' => TokenTag::Circ,
                '+' => {
                    if self.match_next('=') {
                        TokenTag::PlusEqual
                    } else {
                        TokenTag::Plus
                    }
                },
                '*' => {
                    if self.match_next('=') {
                        TokenTag::StarEqual
                    } else {
                        TokenTag::Star
                    }
                },
                '/' => {
                    if self.match_next('=') {
                        TokenTag::SlashEqual
                    } else {
                        TokenTag::Slash
                    }
                },
                '!' => {
                    if self.match_next('=') {
                        TokenTag::BangEqual
                    } else {
                        TokenTag::Bang
                    }
                },
                '-' => {
                    if self.match_next('>') {
                        TokenTag::ArrowRight
                    } else if self.match_next('=') {
                        TokenTag::MinusEqual
                    } else if self.match_next('-') {
                        self.skip_line();
                        return self.lex_token()
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
                '"' => self.lex_string(),
                '0'..='9' => self.lex_number(),

                // Identifiers and keywords
                'A'..='Z' | 'a'..='z' | '_' => self.lex_keyword(),
                _ => {
                    return Err(LexicalError {
                        token: self.create_token(TokenTag::Error),
                        tag: UnknownToken
                    })
                }
            };

            Ok(self.create_token(token_tag))
        } else {
            Ok(self.create_token(TokenTag::EndOfFile))
        }
    }

    fn create_token(&self, tag: TokenTag) -> Token {
        let len = self.curr-self.start;
        Token {
            tag: tag,
            lexeme: Lexeme {
                start: self.start,
                end: self.curr
            },
            info: DebugInfo {
                fname: self.fname.clone(),
                line: self.line,
                col: self.col-len+1,
                len: len,
                src: self.src.clone()
            }
        }
    }

    fn lex_keyword(&mut self) -> TokenTag {
        match self.prev().unwrap() {
            'e' => self.accept_keyword("lse", TokenTag::Else),
            'f' => {
                match self.current() {
                    Some('a') => self.accept_keyword("alse",TokenTag::False),
                    Some('u') => self.accept_keyword("unc",TokenTag::Func),
                    _ => self.lex_identifier()
                }
            }
            'p' => self.accept_keyword(
                "rint",
                TokenTag::Print
            ),
            'l' => {
                match self.current() {
                    Some('e') => self.accept_keyword("et",TokenTag::Let),
                    Some('o') => self.accept_keyword("oop",TokenTag::Loop),
                    _ => self.lex_identifier()
                }
            },
            't' => self.accept_keyword(
                "rue",
                TokenTag::True
            ),
            'i' => self.accept_keyword(
                "f",
                TokenTag::If
            ),
            'r' => {
                match self.current() {
                    Some('e') => match self.next() {
                        Some('t') => self.accept_keyword(
                            "eturn",
                            TokenTag::Return
                        ),
                        Some('p') => self.accept_keyword(
                            "epeat",
                            TokenTag::Repeat
                        ),
                        _ => self.lex_identifier()
                    },
                    _ => self.lex_identifier()
                }
            },
            _ => self.lex_identifier()
        }
    }

    fn accept_keyword(&mut self, word: &'static str, tag: TokenTag) -> TokenTag {
        if self.match_word(word) {
            return tag;
        }

        self.lex_identifier()
    }

    fn lex_identifier(&mut self) -> TokenTag {
        let mut name = String::new();
        name.push(self.prev().unwrap());

        while let Some(ch) = self.current() {
            if !self.is_next_identifier_char(ch) {
                break;
            }

            name.push(self.accept().unwrap());
        };

        TokenTag::Identifier(name)
    }

    fn lex_string(&mut self) -> TokenTag {
        while self.accept().unwrap() != '"' {
            if self.current().is_none() {
                panic!("Unterminated string");
            }
        };

        TokenTag::String(String::from(&self.src[self.start+1..self.curr-1]))
    }

    fn lex_number(&mut self) -> TokenTag {
        let ch = self.prev().unwrap();
        let mut mantissa: f64 = f64::from(ch.to_digit(10).unwrap());
        let mut exp = 0;

        while let Some(digit) = self.current() {
            if !digit.is_ascii_digit() {
                break;
            }

            mantissa *= 10.0;
            mantissa += self.lex_digit();
        };

        if self.current() == Some('.') {
            self.accept();

            while let Some(digit) = self.current() {
                if !digit.is_ascii_digit() {
                    break;
                }

                exp += 1;
                mantissa *= 10.0;
                mantissa += self.lex_digit();
            }
        }

        TokenTag::Number(
            f64::from(
                mantissa/10.0_f64.powi(exp)
            )
        )
    }

    fn lex_digit(&mut self) -> f64 {
        f64::from(self.accept().unwrap().to_digit(10).unwrap())
    }

    fn is_next_identifier_char(&self, ch: char) -> bool {
        self.is_identifier_char(ch) || ch.is_ascii_digit()
    }

    fn is_identifier_char(&self, ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }

    fn match_word(&mut self, word: &'static str) -> bool {
        if self.check_word(word) {
            self.curr += word.len();
            self.col += word.len();
            return true;
        }

        false
    }

    fn check_word(&mut self, word: &'static str) -> bool {
        &self.src[self.curr..self.curr+word.len()] == String::from(word)
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
            } else {
                self.col += 1;
            }
        }

        return ch;
    }

    fn skip_line(&mut self) {
        while self.current() != Some('\n') && self.current().is_some() {
            self.accept();
        }
    }

    #[allow(dead_code)]
    fn next(&self) -> Option<char> {
        self.src.chars().nth(self.curr+1)
    }

    fn prev(&self) -> Option<char> {
        self.src.chars().nth(self.curr-1)
    }

    fn current(&self) -> Option<char> {
        self.src.chars().nth(self.curr)
    }
}
