extern crate pancurses;
mod buffer;
mod util;
use pancurses::{initscr, endwin, noecho,Input,Window};
use std::collections::HashMap;

// TODO: Move this somewhere else
fn kbf_nop(_state: &mut State) {}

type KeyBindFunction = fn(state: &mut State);

pub struct State {
    mode: Mode,
    nmap: HashMap<char, KeyBindFunction>,
    current_buffer: buffer::Buffer, 
    window: Window,
    cursor_x: i32, 
    cursor_y: i32, 
}

impl State {
    pub fn new() -> Self {
        let mut s = Self {
            mode: Mode::NORMAL, 
            nmap: HashMap::new(),
            current_buffer: buffer::Buffer::new(),
            window: initscr(), 
            cursor_x: 0, 
            cursor_y: 0, 
        };

        s.nmap.insert('i', |state: &mut State| state.mode = Mode::INSERT);
        s.nmap.insert('h', |state: &mut State| state.move_relative(-1, 0));
        s.nmap.insert('j', |state: &mut State| state.move_relative(0, 1));
        s.nmap.insert('k', |state: &mut State| state.move_relative(0, -1));
        s.nmap.insert('l', |state: &mut State| state.move_relative(1, 0));

        return s;
    }

    pub fn move_relative(&mut self, x: i32, y: i32) {
        self.cursor_y = util::clamp(self.cursor_y+y, 0, self.current_buffer.get_line_count());
        self.cursor_x = util::clamp(self.cursor_x+x, 0, self.current_buffer.get_line_length(self.cursor_y));
        self.window.mv(self.cursor_y, self.cursor_x);
    }
}

fn main() {
    let mut state = State::new();

    noecho();
    loop {
        match state.window.getch() {
            //Some(Input::Character('\x1B')) => { state.window.printw("Esc"); },
            Some(Input::Character(c)) => handle_character(c, &mut state), 
            _ => (), 
        }
    }
    endwin();
}

fn handle_character(c: char, mut state: &mut State) {
    let knop: KeyBindFunction = kbf_nop;

    match state.mode {
        Mode::NORMAL => { state.nmap.get(&c).unwrap_or(&knop)(&mut state); }
        Mode::INSERT => { insert_character(c, &mut state ); }
    }
}

fn insert_character(c: char, state: &mut State) {
    state.current_buffer.write_char(c, state.cursor_x, state.cursor_y);
    state.window.insch(c);
}

pub enum Mode {
    NORMAL, INSERT
}
