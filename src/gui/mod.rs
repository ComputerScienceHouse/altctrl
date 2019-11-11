extern crate ncurses;
extern crate gui_lib;


use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};
use ncurses::*;

use crate::shared::Event;
use crate::protocol::NewWindow;
use gui_lib::*;

#[derive(Clone, Debug)]
pub enum GuiEvent{
    CreateWindow(NewWindow),
    DestroyWindow(String),
    Log(String),
    Clear(),
}

pub fn launch(tx: Sender<Event>, rx: Receiver<GuiEvent>)
{
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Set up omniscient stuff
    // HashMap of active windows, so that we know what's bonkin'
    let mut windows: std::collections::HashMap<String, WINDOW> = HashMap::new();
    // Log buffer to use for keeping track of command output.
    let mut logbuffer: Vec<String> = Vec::new();
    for _i in 0..5 { logbuffer.push(" ".to_string()); }
    showlog(&logbuffer);

    refresh();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    for message in rx.iter() {
        showlog(&logbuffer);
        refresh();
        match message {
            GuiEvent::CreateWindow(new_window) => {
                open_win(new_window.x_pos, new_window.y_pos, new_window.width, new_window.height, &new_window.id, &new_window.content, &mut windows, &mut logbuffer);
            },
            GuiEvent::DestroyWindow(id) => {
                close_win(id, &mut windows, &mut logbuffer);
            },
            GuiEvent::Log(log_event) => {
                logbuffer.insert(0, log_event.to_string());
                // logbuffer.insert(0, "event received!".to_string());
                // showlog(&logbuffer);
                // dbg!("LOG EVENT RECEIVED!");
            },
            GuiEvent::Clear() => {
                clear();
                showlog(&logbuffer);
            },
        }
        showlog(&logbuffer);
        refresh();
        // let ch = getch();
        // if ch == KEY_F(1) {
        //     break;
        // }
    }
    endwin();
    std::process::exit(0);
}

