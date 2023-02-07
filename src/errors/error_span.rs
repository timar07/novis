use std::fmt::Display;

use crate::lexer::token::Token;
use colored::*;

#[derive(Clone)]
pub struct Span {
    pub start: Token,
    pub end: Token,
}

impl Display for Span {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = self.clone().start.info;
        let len = (self.end.info.col+self.end.info.len) - self.start.info.col;

        let snippet_prefix = format!("    {} | ", info.line)
            .bright_black();

        if let Some(line) = info.src.lines().nth(info.line-1) {
            writeln!(
                f,
                "{}{}",
                snippet_prefix,
                line
            )?;

            write!(
                f,
                "{}{}{}",
                " ".repeat(snippet_prefix.len() + info.col-1),
                "~".repeat(len-1).red(),
                "^".red()
            )?;
        }

        write!(f, "")?;

        Ok(())
    }
}
