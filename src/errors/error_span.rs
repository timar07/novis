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

#[derive(Debug, Clone)]
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
                self.start.info.line-1,
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
    /// Highlight inline span, for example:
    ///     1 | print foo(2+2)/0;
    ///         ~~~~~~~~~~~~~~~~^
    pub fn inline(
        src: Rc<String>,
        line: usize,
        col: usize,
        len: usize
    ) -> String {
        LineFormatter::new(
            line,
            &src.lines().nth(line).unwrap_or(" ").to_string(),
            Some(Highlighter::underline('~', col, len))
        )
    }

    /// Highlight multiline span, for example:
    ///     1 | print foo(123 /
    ///     2 |           (2+3-5)
    ///     3 |       );
    ///         ________________^
    pub fn multiline(
        src: Rc<String>,
        start: usize,
        end: usize
    ) -> String {
        let mut snippet: String = String::from("");
        let mut width: usize = 0;
        let lines = src
            .lines()
            .map(|line| { line.to_string() })
            .collect::<Vec<String>>();

        let excerpt = lines[start..end]
            .iter()
            .enumerate();

        for (n, line) in excerpt {
            if line.len() > width {
                width = line.len();
            }

            // Last line ends with highlight
            if n == end-start-1 {
                snippet.push_str(&LineFormatter::new(
                    n,
                    line,
                    Some(Highlighter::underline('_', 1, width))
                ));
            } else {
                snippet.push_str(&LineFormatter::new(
                    n,
                    line,
                    None
                ));
            }
        }

        snippet
    }

    /// Util function for creating underlines.
    ///
    /// # Examples
    /// ```
    /// let foo = Highlighter::underline('~', 1, 10);
    /// assert_eq!(foo, "~~~~~~~~~^".to_string().red());
    /// ```
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
    /// Line formatting util.
    /// # Examples
    /// ```
    /// let line = String::from("foo(\"some code\");")
    /// let fmt_line = LineFormatter::new(1, &line, None);
    /// println!(fmt_line); // 1 | foo("some code");
    /// ```
    pub fn new(number: usize, line: &String, extra: Option<String>) -> String {
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
