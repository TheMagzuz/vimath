use std::string::String;

pub struct Buffer {
    pub lines: Vec<String>, 
    line_count: i32, 
}

impl Buffer {
    pub fn new() -> Self {
        return Self {
            lines: vec![String::new(); 20], 
            line_count: 1, 
        };
    }


    pub fn write_char(&mut self, c: char, x: i32, y: i32) {
        self.lines[y as usize].insert(x as usize, c);
    }

    pub fn newline_at(&mut self, line: i32) {
        assert!(line < self.get_line_count()+1);

        self.lines.insert(line as usize, "".to_string());
        self.line_count += 1;

    }

    pub fn remove_line(&mut self, line: i32) -> String {
        assert!(line < self.get_line_count()+1);

        self.line_count -= 1;
        return self.lines.remove(line as usize);
    }

    pub fn get_line(&self, line: i32) -> &String {
        return &self.lines[line as usize];
    }

    pub fn get_line_buffer_count(&self) -> i32 {
        return self.lines.len() as i32;
    }

    pub fn get_line_count(&self) -> i32 {
        return self.line_count;
    }

    pub fn get_last_line(&self) -> i32 {
        let mut last = 0;
        for i in 0..self.lines.len() {
            if !self.lines[i].is_empty() {
                last = i;
            }
        }
        return last as i32;
    }

    pub fn get_line_length(&self, line: i32) -> i32 {
        return self.lines[line as usize].chars().count() as i32;
    }

}
