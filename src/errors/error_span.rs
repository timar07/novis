use std::{
    fmt::{
        Display,
        Formatter,
        Result
    },
    rc::Rc
};

use crate::lexer::token::Token;
use colored::*;

#[derive(Clone)]
pub struct Span {
    pub start: Token,
    pub end: Token,
}

impl From<Token> for Span {
    fn from(item: Token) -> Self {
        Self {
            start: item.clone(),
            end: item.clone()
        }
    }
}

impl Display for Span {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let info = self.clone().start.info;
        let snippet = if self.start.info.line != self.end.info.line {
            Highlighter::multiline(
                info.src,
                self.start.info.line,
                self.end.info.line
            )
        } else {
            let len = (self.end.info.col+self.end.info.len) - self.start.info.col;

            Highlighter::inline(
                info.src,
                info.line-1,
                info.col,
                if len == 0 { 1 } else { len }
            )
        };


        write!(
            f,
            "{}",
            snippet
        )?;

        Ok(())
    }
}

struct Highlighter;

impl Highlighter {
    pub fn inline(
        src: Rc<String>,
        line: usize,
        col: usize,
        len: usize
    ) -> String {
        dbg!(src.lines().nth(line).unwrap_or(" "));
        LineFormatter::new(
            line,
            src.lines().nth(line).unwrap_or(" ").into(),
            Some(Highlighter::underline('~', col, len))
        )
    }

    pub fn multiline(
        src: Rc<String>,
        start: usize,
        end: usize
    ) -> String {
        let mut snippet: String = String::from("");
        let mut width: usize = 0;

        for n in start-1..end {
            let line = src.lines().nth(n).unwrap();

            if line.len() > width {
                width = line.len();
            }

            snippet.push_str(&LineFormatter::new(
                n,
                line.into(),
                None
            ));
        }

        format!(
            "{}{}",
            snippet,
            Highlighter::underline('_', 8, width+1)
        )
    }

    fn underline(ch: char, col: usize, len: usize) -> String {
        format!(
            "{}{}{}",
            " ".repeat(col-1),
            ch.to_string().repeat(len-1).red(),
            "^".red()
        )
    }
}

struct LineFormatter;
impl LineFormatter {
    pub fn new(number: usize, line: String, extra: Option<String>) -> String {
        let snippet_prefix = format!(
            "    {} {} ",
            (number+1).to_string(),
            "|"
        );

        format!(
            "{}{}{}",
            snippet_prefix.bright_black(),
            line,
            if extra.is_some() {
                format!(
                    "\n{}{}",
                    " ".repeat(snippet_prefix.len()),
                    extra.unwrap()
                )
            } else {
                String::from("\n")
            }
        )
    }
}
