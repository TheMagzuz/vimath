use std::string::String;

pub struct Buffer {
    pub lines: Vec<String>
}

impl Buffer {
    pub fn new() -> Self {
        return Self {
            lines: vec![String::new(); 20], 
        };
    }

    pub fn write_char(&mut self, c: char, x: i32, y: i32) {
        self.lines[y as usize].insert(x as usize, c);
    }

    pub fn get_line_count(&self) -> i32 {
        return self.lines.len() as i32;
    }

    pub fn get_line_length(&self, line: i32) -> i32 {
        return self.lines[line as usize].chars().count() as i32;
    }

}
