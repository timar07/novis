use crate::{errors::{
    print_error,
    DebugInfo
}};

pub struct RuntimeError {
    pub msg: String,
    pub info: DebugInfo
}

impl RuntimeError {
    pub fn print(&self) {
        print_error(
            "Runtime Error",
            self.msg.clone(),
            self.info.clone(),
        )
    }
}
