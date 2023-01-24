use std::fs;

pub struct FileStream {
    pub path: String
}

impl FileStream {
    pub fn new(path: &String) -> FileStream {
        FileStream {
            path: path.clone()
        }
    }

    pub fn as_str(&self) -> String {
        match fs::read_to_string(&self.path) {
            Ok(str) => str,
            Err(_) => panic!("Fatal error: failed to open file")
        }
    }
}
