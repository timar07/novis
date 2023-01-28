use std::{rc::Rc};

#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub fname: String,
    pub line: usize,
    pub col: usize,
    pub len: usize,
    pub src: Rc<String>
}

pub trait DescribableError {
    fn print(&self) -> ();
}
