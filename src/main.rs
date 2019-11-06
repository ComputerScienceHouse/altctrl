/*
    Copyright © 2013 Free Software Foundation, Inc
    See licensing in LICENSE file

    File: examples/ex_4.rs
    Author: Jesse 'Jeaye' Wilkerson
    Description:
      Window creation and input example.
      Use the cursor keys to move the window
      around the screen.
*/

extern crate ncurses;

use ncurses::*;

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 4;

fn main()
{
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Status/help info. */
    addstr("Use the arrow keys to move");
    mvprintw(LINES() - 1, 0, "Press F1 to exit");
    refresh();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    /* Start in the center. */
    let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
    let mut start_x = (max_x - WINDOW_WIDTH) / 2;
    let mut win = create_win(start_y, start_x);

    let mut ch = getch();
    while ch != KEY_F(1)
    {
        match ch
        {
          KEY_LEFT =>
          {
            start_x -= 1;
            destroy_win(win);
            win = create_win(start_y, start_x);
          },
          KEY_RIGHT =>
          {
            start_x += 1;
            destroy_win(win);
            win = create_win(start_y, start_x);
          },
          KEY_UP =>
          {
            start_y -= 1;
            destroy_win(win);
            win = create_win(start_y, start_x);
          },
          KEY_DOWN =>
          {
            start_y += 1;
            destroy_win(win);
            win = create_win(start_y, start_x);
          },
          103 =>
          {
              mvprintw( 0, 30, "Enter x");
              let mut s = String::new();
              getstr(&mut s);
              start_x = s.parse().unwrap();
              //dbg!(s);

              mvprintw(0, 38, "Enter y");
              let mut s = String::new();
              getstr(&mut s);
              start_y = s.parse().unwrap();
              //dbg!(s);

              mvprintw(0, 30, "                                                   ");
          },
          _ => { }
        }

        put_pos(start_y, start_x);
        mvprintw(LINES() - 1, 0, "Press F1 to exit");

        ch = getch();

        if start_x == 0 { start_x = max_x-2; }
        if start_x == max_x-1 { start_x = 1; }
        if start_y == 0 { start_y = max_y-2; }
        if start_y == max_y-1 { start_y = 1; }

        if start_x == 1 && start_y == 1 {
            put_alert(30, 30, "The quick brown fox jumps over the lazy dog. and actually, I believe you'll find that it's pronounced whomstved...");
        }
    }

    endwin();
}

fn create_win(start_y: i32, start_x: i32) -> WINDOW
{
  let win = newwin(WINDOW_HEIGHT, WINDOW_WIDTH, start_y, start_x);
  box_(win, 0, 0);
  wrefresh(win);
  win
}

fn destroy_win(win: WINDOW)
{
  let ch = ' ' as chtype;
        wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
        wrefresh(win);
        delwin(win);
}

fn put_pos(start_y: i32, start_x: i32) {
    attron(A_BOLD());
    mvprintw(LINES() -2, 0, "                    ");
    mvprintw(LINES() - 2, 0, format!("X: {} Y: {}", start_y, start_x).as_str());
    attroff(A_BOLD());
}

fn put_alert(x_dim: i32, y_dim: i32, message: &str) {
    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    let start_y = (max_y - y_dim) / 2;
    let start_x = (max_x - x_dim) / 2;
    let win = newwin((x_dim)+2, (y_dim)+2, start_y, start_x);
    //mvprintw(start_y + 1, start_x + 1, message);
    if message.len() > (x_dim as usize)
    {
        let real_x_dim = x_dim as usize;
        for i in 0..message.len(){
            let i_i32 = i as i32;
            let start = real_x_dim*i;
            if i == 0 {
                mvprintw(start_y+1+i_i32, start_x+1, &message[0..real_x_dim]);
            } else if real_x_dim*(i+1) >= message.len() {
                mvprintw(start_y+1+i_i32, start_x+1, &message[real_x_dim*(i)..]);
                break;
            } else {
                mvprintw(start_y+1+i_i32, start_x+1, &message[real_x_dim*(i)..real_x_dim*(i+1)]);
            }
        }
        //mvprintw(start_y+1, start_x+1, &message[..(x_dim as usize)*2-2]);
        //mvprintw(start_y+2, start_x+1, &message[(x_dim as usize)*2-2..]);
    } else {
        mvprintw(start_y+1, start_x+1, &message);
    }
    box_(win, 0, 0);
    wrefresh(win);
}
