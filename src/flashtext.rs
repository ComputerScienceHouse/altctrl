/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file
    File: examples/ex_1.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Simple "Hello, world" example.
*/

extern crate ncurses;

use ncurses::*;

fn main() {
    flashtext();
}



fn flashtext()
{
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();
    /* Prompt for a character. */
    addstr("Enter characters: \n");
    loop {
        /* Wait for input. */
        let ch = getch();
        if ch == KEY_F1
        {
            /* Enable attributes and output message. */
            attron(A_BOLD() | A_BLINK());
            addstr("\nF1");
            attroff(A_BOLD() | A_BLINK());
            addstr(" pressed");
        }
        else if ch == 10
        {
            show_char(ch);
            addstr("\nPress a key to exit.");
            break;
        }
        else
        {
            show_char(ch);
        }

        /* Refresh, showing the previous message. */
        refresh();
    }
    /* Wait for one more character before exiting. */
    getch();
    endwin();
}

fn show_char(ch: i32)
{
    /* Enable attributes and output message. */
    //addstr("\nKey pressed: ");
    attron(A_BOLD() | A_BLINK());
    addstr(format!("{}", std::char::from_u32(ch as u32).expect("Invalid char")).as_ref());
    attroff(A_BOLD() | A_BLINK());

}
