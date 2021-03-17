extern crate pancurses;
mod buffer;
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
}

impl State {
    pub fn new() -> Self{
        Self {
            mode: Mode::NORMAL, 
            nmap: HashMap::new(),
            current_buffer: buffer::Buffer::new(),
            window: initscr(), 
        }
    }
}

fn main() {
    let mut state = State::new();

    noecho();
    loop {
        match state.window.getch() {
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
    state.current_buffer.text.push(c);
    state.window.addch(c);
}

pub enum Mode {
    NORMAL, INSERT
}
