extern crate pancurses;
use crate::buffer;
use crate::util;
use pancurses::{Window, initscr};

pub struct ViMathWindow {
    pub window: Window, 
    buffer: buffer::Buffer, 
    cursor_x: i32, 
    cursor_y: i32, 
}

impl ViMathWindow {
    pub fn new() -> Self{
        let mut s = Self {
            window: initscr(), 
            buffer: buffer::Buffer::new(), 
            cursor_x: 0, 
            cursor_y: 0, 
        };
        s.draw_line_numbers();
        s.mv(0, 0);
        return s;
    }

    pub fn draw_line_numbers(&self) {
        for i in 0..self.buffer.get_line_count() {
            self.window.mvaddstr(i, 0, i.to_string());
        }
        self.window.vline('|', -1);
    }

    pub fn gutter_size(&self) -> i32 {
        return (self.buffer.get_line_count().to_string().len()+1) as i32;
    }

    pub fn mv(&mut self, y: i32, x: i32) {
        self.cursor_y = y;
        self.cursor_x = x;
        self.mv_no_mut(y, x);
    }

    fn mv_no_mut(&mut self, y: i32, x: i32) {
        self.window.mv(y, x+self.gutter_size());
    }

    pub fn mv_relative(&mut self, y: i32, x: i32) {
        self.cursor_y = util::clamp(self.cursor_y+y, 0, self.buffer.get_line_count()-1);
        self.cursor_x = util::clamp(self.cursor_x+x, 0, self.buffer.get_line_length(self.cursor_y));
        self.mv(self.cursor_y, self.cursor_x);
    }

    pub fn insert_character(&mut self, c: char) {
        if c == '\n' {
            self.buffer.newline_at(self.cursor_y+1);
            self.draw_line_numbers();
            self.mv_relative(1, 0);
            return;
        }
        self.buffer.write_char(c, self.cursor_x, self.cursor_y);
        self.redraw_line();
    }

    pub fn backspace(&mut self) {
        // If we are on the first character of the first line, just do nothing
        if self.cursor_x == 0 && self.cursor_y == 0 {
            return;
        }

        // If we are on the first character of the line, join it with the line before
        if self.cursor_x == 0 {
            let removed = self.buffer.lines.remove(self.cursor_y as usize);
            let old_len = self.buffer.lines[(self.cursor_y-1) as usize].len();
            self.buffer.lines[(self.cursor_y-1) as usize].push_str(&removed);
            self.mv(self.cursor_y-1, (old_len-1) as i32);
            self.redraw_line();
            self.redraw_end();
            return
        }

        // Otherwise, just delete the character
        self.mv_relative(0, -1);
        self.window.delch();
        self.buffer.lines[self.cursor_y as usize].remove(self.cursor_x as usize);
    }

    fn redraw_line(&mut self) {
        self.mv_no_mut(self.cursor_y, 0);
        self.window.clrtoeol();
        self.window.addstr(self.buffer.get_line(self.cursor_y));
        self.cursor_x += 1;
        self.mv_no_mut(self.cursor_y, self.cursor_x);
    }

    fn redraw_end(&mut self) {
        self.window.clrtobot();
        for i in self.cursor_y..self.buffer.get_line_count()-1 {
            self.mv_no_mut(i, 0);
            self.window.printw(self.buffer.get_line(i));
        }
    }

}
