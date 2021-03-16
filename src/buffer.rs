use std::string::String;

pub struct Buffer {
    pub text: String
}

impl Buffer {
    pub fn new() -> Self {
        return Self {
            text: String::from("")
        };
    }
}
