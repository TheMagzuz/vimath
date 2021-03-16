extern crate pancurses;
mod buffer;
use pancurses::{initscr, endwin, noecho,Input};
use std::collections::HashMap;

type KeyBindFunction = fn();

let mut nmap: HashMap<char, KeyBindFunction> = HashMap::new();
let mut mode = Mode::NORMAL;

fn main() {
    let window = initscr();
    let curbuf = buffer::Buffer::new();

    noecho();
    loop {
        match window.getch() {
            Some(Input::Character(c)) => handle_character(c), 
            None => ()
        }
    }
    endwin();
}

fn handle_character(c: char) {

    match mode {
        Mode::NORMAL => { nmap.get(c)?(); }
    }

}

pub enum Mode {
    NORMAL, INSERT
}
