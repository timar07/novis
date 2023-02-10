use std::{rc::Rc};

use colored::Colorize;

#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub fname: String,
    pub line: usize,
    pub col: usize,
    pub len: usize,
    pub src: Rc<String>
}

pub trait DescribableError {
    fn message(&self) -> String;
    fn kind(&self) -> String;
    fn snippet(&self) -> String;
    fn print(&self) {
        eprintln!(
            "{}: {}\n{}",
            self.kind().red(),
            self.message(),
            self.snippet()
        );
    }
}
