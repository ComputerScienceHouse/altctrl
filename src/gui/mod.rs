extern crate ncurses;
extern crate gui_lib;

use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use ncurses::*;
use crate::types::GuiMsg;
use gui_lib::*;

pub fn launch(_gui_rx: Receiver<GuiMsg>)
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
    mvprintw(0, 0, "Use the arrow keys to move");
    put_pos(0, 0);
    //mvprintw(LINES() - 2, 0, &INSTRUCTIONS);
    refresh();

    let mut windows: std::collections::HashMap<String, WINDOW> = HashMap::new();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);


    let mut window_height: i32 = 3;
    let mut window_width: i32 = 4;

    /* Start in the center. */
    let mut start_y = (max_y - window_height) / 2;
    let mut start_x = (max_x - window_width) / 2;
    let mut win = create_win("mainwindow".to_string(), start_y, start_x, window_width, window_height, &mut windows);

    let mut ch = getch();
    while ch != KEY_F(1) {
        match ch {
            KEY_LEFT => {
                start_x -= 1;
                destroy_win(win);
                win = create_win("mainwindow".to_string(), start_y, start_x, window_width, window_height, &mut windows);
            },
            KEY_RIGHT => {
                start_x += 1;
                destroy_win(win);
                win = create_win("mainwindow".to_string(), start_y, start_x, window_width, window_height, &mut windows);            },
            KEY_UP => {
                start_y -= 1;
                destroy_win(win);
                win = create_win("mainwindow".to_string(), start_y, start_x, window_width, window_height, &mut windows);            },
            KEY_DOWN => {
                start_y += 1;
                destroy_win(win);
                win = create_win("mainwindow".to_string(), start_y, start_x, window_width, window_height, &mut windows);
            },
            99 => { // 'c' -> Clear
                clear();
            },
            101 => { // 'e' -> Eliminate (window)
                mv(1,0);
                addstr("Enter window name: ");
                let mut s = String::new();
                ch = getch();
                while ch != 10 {
                    if ch == 263 {
                        //Delete character
                        s.pop();
                        mv(1,0);
                        clrtoeol();
                        addstr("Enter window name: ");
                        addstr(&s);
                    } else {
                        s.push(ch as u8 as char);
                        addstr(&(ch as u8 as char).to_string());        
                    }                    
                    ch = getch();
                }
                close_win(s, &mut windows);
                mv(1,0);
                clrtoeol();
            },
            103 => { // 'g' -> Move window
                mv(1, 0);
                clrtoeol();
                mv(2, 0);
                clrtoeol();
                addstr("Enter x: ");
                let mut x = String::new();

                ch = getch();
                while ch != 10 {
                    x.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                match x.parse::<i32>() {
                    Ok(n) => start_x = n,
                    Err(_e) => {
                        start_x = start_x;
                        addstr("Invalid position.");
                    },
                }

                addstr(" | Enter y: ");
                let mut y = String::new();
                ch = getch();
                while ch != 10 {
                    y.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                match y.parse::<i32>() {
                    Ok(n) => start_y = n,
                    Err(_e) => {
                        start_y = start_y;
                        addstr("Invalid position.");
                    },
                }
                mv(1, 0);
                clrtoeol();

                mv(2, 0);
                clrtoeol();

            },
            108 => { // 'l' -> List all windows.
                mv(1,0);
                for windowname in windows.keys() {
                    addstr(windowname);
                    addstr("\n");
                }
            },
            109 => { // 'm' -> Display message window
                mv(2,0);
                clrtoeol();
                mv(3,0);
                clrtoeol();
                mv(1,0);
                clrtoeol();
                addstr("Enter window name: ");
                let mut name = String::new();
                ch = getch();
                while ch != 10 {
                    if ch == 263 {
                        //Delete character
                        name.pop();
                        mv(1,0);
                        clrtoeol();
                        addstr("Enter window name: ");
                        addstr(&name);
                    } else {
                        name.push(ch as u8 as char);
                        addstr(&(ch as u8 as char).to_string());        
                    }                    
                    ch = getch();
                }

                mv(2,0);
                addstr("Enter message: ");
                let mut s = String::new();
                ch = getch();
                while ch != 10 {
                    if ch == 263 {
                        //Delete character
                        s.pop();
                        mv(1,0);
                        clrtoeol();
                        addstr("Enter message: ");
                        addstr(&s);
                    } else {
                        s.push(ch as u8 as char);
                        addstr(&(ch as u8 as char).to_string());        
                    }                    
                    ch = getch();
                }

                // DIMENSION CODE
                mv(3, 0);
                clrtoeol();
                addstr("Enter x dimension: ");
                let mut x = String::new();
                ch = getch();
                while ch != 10 {
                    x.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                let x_i32;
                match x.parse::<i32>() {
                    Ok(n) => x_i32 = n,
                    Err(_e) => {
                        x_i32 = 0;
                        // mv(3,0);
                        addstr(" Invalid dimension entered. ");
                        // mv(4,0);

                    },
                }

                addstr(" | Enter y dimension: ");
                let mut y = String::new();
                ch = getch();
                while ch != 10 {
                    y.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                let y_i32;
                match y.parse::<i32>() {
                    Ok(n) => y_i32 = n,
                    Err(_e) => {
                        y_i32 = 0;
                        addstr(" Invalid dimension entered. ");
                        // mv(4,0);
                    },
                }

                //POSITION CODE
                // mv(4, 0);
                clrtoeol();
                addstr(" -> Enter x position: ");
                let mut x = String::new();
                ch = getch();
                while ch != 10 {
                    x.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                let x_i32_pos;
                match x.parse::<i32>() {
                    Ok(n) => x_i32_pos = n,
                    Err(_e) => {
                        x_i32_pos = 0;
                        mv(3,0);
                        addstr(" Invalid position entered. ");
                        mv(4,0);
                    },
                }

                addstr(" | Enter y position: ");
                let mut y = String::new();
                ch = getch();
                while ch != 10 {
                    y.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                let y_i32_pos;
                match y.parse::<i32>() {
                    Ok(n) => y_i32_pos = n,
                    Err(_e) => {
                        y_i32_pos = 0;
                        addstr(" Invalid position entered. ");
                        mv(5,0);
                    },
                }

                put_alert(x_i32_pos, y_i32_pos, x_i32, y_i32, &name, &s, &mut windows);

                mv(1,0);
                clrtoeol();
                mv(2,0);
                clrtoeol();
                mv(3,0);
                clrtoeol();
                mv(4,0);
                clrtoeol();

            },
            114 => { // 'r' -> Resize main window
                mv(1, 0);
                clrtoeol();
                mv(2, 0);
                clrtoeol();
                addstr("Enter x: ");
                let mut x = String::new();

                ch = getch();
                while ch != 10 {
                    x.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                match x.parse::<i32>() {
                    Ok(n) => window_width = n,
                    Err(_e) => {
                        window_width = window_width;
                        addstr("Invalid position. ");
                    },
                }

                addstr(" | Enter y: ");
                let mut y = String::new();
                ch = getch();
                while ch != 10 {
                    y.push(ch as u8 as char);
                    addstr(&(ch as u8 as char).to_string());
                    ch = getch();
                }
                match y.parse::<i32>() {
                    Ok(n) => window_height = n,
                    Err(_e) => {
                        window_height = window_height;
                        addstr("Invalid position. ");
                    },
                }
                mv(1, 0);
                clrtoeol();
                mv(2, 0);
                clrtoeol();
            },
            _ => { }
        }

        mvprintw(0, 0, "Use the arrow keys to move");
        put_pos(start_x, start_y);
        ch = getch();

        if start_x == 0 { start_x = max_x-2; }
        if start_x == max_x-1 { start_x = 1; }
        if start_y == 0 { start_y = max_y-2; }
        if start_y == max_y-1 { start_y = 1; }

        /*if start_x == 1 && start_y == 1 {
            put_alert(-1, -1, 30, 10, "The quick brown fox jumps over the lazy dog. and actually, I believe you'll find that it's pronounced whomstved... What is ligma? How did I get this disease? What are my options?");
        }*/
    }

    endwin();
}

