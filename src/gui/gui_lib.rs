use ncurses::*;
use std::collections::HashMap;
use crate::protocol::{WindowContent, WindowStyle};

pub fn close_win(window: String, windows: &mut HashMap<String,WINDOW>, logbuffer: &mut Vec<String>) {
    match window.as_ref() {
        "mainwindow" => {
            logbuffer.insert(0, "You idiot! Don't delete the main window!".to_string());
            showlog(&logbuffer);
        },
        _ => {
            match windows.get(&window) {
                Some(&win) => {
                    let ch = ' ' as chtype;
                    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
                    wrefresh(win);
                    delwin(win);
                    windows.remove(&window);
                    logbuffer.insert(0, format!("Window \"{}\" destroyed.", window).to_string());
                    showlog(&logbuffer);
                },
                _ => {
                    mvprintw(2, 0, "Invalid window name!");
                },
            }
        }
    }
}

pub fn open_win(x_loc: i32,
                y_loc: i32,
                x_dim: i32,
                y_dim: i32,
                name: &str,
                content: WindowContent,
                style: WindowStyle,
                windows: &mut HashMap<String,WINDOW>,
                logbuffer: &mut Vec<String>) {
    if !windows.contains_key(name){
    let mut max_x = 0;
    let mut max_y = 0;
    let start_x;
    let start_y;
    match x_loc+y_loc {
        -2 => {
            /* Get the screen bounds. */
            getmaxyx(stdscr(), &mut max_y, &mut max_x);
            start_y = max_y / 2;
            start_x = max_x / 2;
        },
        _ => {
            max_x = x_loc;
            max_y = y_loc;
            start_y = max_y;
            start_x = max_x;
        },
    }
    let win = newwin((y_dim)+2, (x_dim)+2, start_y, start_x);
    windows.insert(name.to_string(), win);
    if let Some(message) = content.text {
        if message.len() > (x_dim as usize) {
            let real_x_dim = x_dim as usize;
            for i in 0..message.len(){
                let i_i32 = i as i32;
                if i == 0 {
                    mvprintw(start_y+1+i_i32, start_x+1, &message[0..real_x_dim]);
                } else if real_x_dim*(i+1) >= message.len() {
                    mvprintw(start_y+1+i_i32, start_x+1, &message[real_x_dim*(i)..]);
                    break;
                } else {
                    mvprintw(start_y+1+i_i32, start_x+1, &message[real_x_dim*(i)..real_x_dim*(i+1)]);
                }
            }
        } else {
            mvprintw(start_y+1, start_x+1, &message);
        }
    }
    
    box_(win, 0, 0);
    wrefresh(win);
    attron(A_BOLD());
    let title = format!("|{}|", name);
    mvprintw(start_y, start_x+1, &title);
    attroff(A_BOLD());
    } else {
        logbuffer.insert(0, "This name is already taken!".to_string());
        showlog(&logbuffer);
    }
}

pub fn showlog(logbuffer: &Vec<String>) {
    let mut max_x = 0;
    let mut max_y = 0;
    /* Get the screen bounds. */
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    mv(5, 0);
    for _i in 0..max_x {
        addstr("-");
    }
    attron(A_BOLD());
    mvprintw(5, COLS() - 8, &"Console");
    attroff(A_BOLD());
    mv(0,0);

    //Update log window...
    for i in 0..5 {
        mv(i,0);
        clrtoeol();
    }
    mv(0,0);
    for i in (0..5).rev() {
        mv(4-(i as i32), 0);
        // addstr(&i.to_string());
        addstr(logbuffer.get(i).unwrap());
        // addstr("\n");
    }
}