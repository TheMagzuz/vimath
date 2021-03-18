extern crate pancurses;
mod buffer;
mod util;
mod vimathwindow;
use pancurses::{endwin, noecho,Input,curs_set};
use std::collections::HashMap;

// TODO: Move this somewhere else
fn kbf_nop(_state: &mut State) {}

type KeyBindFunction = fn(state: &mut State);

pub struct State {
    mode: Mode,
    nmap: HashMap<char, KeyBindFunction>,
    window: vimathwindow::ViMathWindow,
}

impl State {
    pub fn new() -> Self {
        let mut s = Self {
            mode: Mode::NORMAL, 
            nmap: HashMap::new(),
            window: vimathwindow::ViMathWindow::new(), 
        };

        s.nmap.insert('i', |state: &mut State| state.set_mode(Mode::INSERT));
        s.nmap.insert('h', |state: &mut State| state.window.mv_relative(0, -1));
        s.nmap.insert('j', |state: &mut State| state.window.mv_relative(1, 0));
        s.nmap.insert('k', |state: &mut State| state.window.mv_relative(-1, 0));
        s.nmap.insert('l', |state: &mut State| state.window.mv_relative(0, 1));

        return s;
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        match self.mode {
            Mode::INSERT => { curs_set(1); }, 
            Mode::NORMAL => { curs_set(2); }
        }
    }

}

fn main() {
    let mut state = State::new();
    curs_set(2);

    noecho();
    loop {
        match state.window.window.getch() {
            Some(Input::Character('\x1B')) => { 
                match state.mode {
                    Mode::INSERT => { state.set_mode(Mode::NORMAL) }, 
                    _ => (), 
                }
            },
            Some(Input::Character('\x7f')) => {
                match state.mode {
                    Mode::INSERT => state.window.backspace(), 
                    _ => (), 
                }
            },
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
        Mode::INSERT => { state.window.insert_character(c); }
    }
}

pub enum Mode {
    NORMAL, INSERT
}
