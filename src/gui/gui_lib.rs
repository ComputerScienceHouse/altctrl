use ncurses::*;
use std::collections::HashMap;
use crate::protocol::WindowData;

pub fn close_win(window: String, windows: &mut HashMap<String,(WINDOW, WindowData)>, logbuffer: &mut Vec<String>) {
    match window.as_ref() {
        "mainwindow" => {
            logbuffer.insert(0, "You idiot! Don't delete the main window!".to_string());
            showlog(&logbuffer);
        },
        _ => {
            match windows.get(&window) {
                Some(win) => {
                    let ch = ' ' as chtype;
                    wborder(win.0, ch, ch, ch, ch, ch, ch, ch, ch);
                    wrefresh(win.0);
                    delwin(win.0);
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

pub fn clear_windows(windows: &mut HashMap<String, (WINDOW, WindowData)>, logbuffer: &mut Vec<String>) {
    // let mut WINDOW;
    for (title, win) in &*windows {
        match title.as_ref() {
            "mainwindow" => { },
            _ => {
                let ch = ' ' as chtype;
                wborder(win.0, ch, ch, ch, ch, ch, ch, ch, ch);
                wrefresh(win.0);
                delwin(win.0);
                logbuffer.insert(0, format!("Window \"{}\" destroyed.", title).to_string());
                showlog(&logbuffer);
            }
        }
    }
    windows.clear();
    logbuffer.insert(0, "Cleared all windows.".to_string());
    showlog(&logbuffer);
}

pub fn draw_win(new_window: &WindowData, win: WINDOW) {
    let x_loc = new_window.x_pos;
    let y_loc = new_window.y_pos;
    let x_dim = new_window.width;
    let y_dim = new_window.height;
    let name = &new_window.id;
    let content = &new_window.content;
    let message = &new_window.message;
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
    
    // Match content, then use that to figure out the data.
    match content.as_str() {
        "Text" | "T" => { // Display whatever text you need in a normal, window wrapping fashion.
            if message.len() > (x_dim as usize) {
                let real_x_dim = x_dim as usize;
                for i in 0..message.len(){
                    if i == 0 {
                        mvprintw(start_y+1+(i as i32), start_x+1, &message[0..real_x_dim]);
                    } else if real_x_dim*(i+1) >= message.len() {
                        mvprintw(start_y+1+(i as i32), start_x+1, &message[real_x_dim*(i)..]);
                        break;
                    } else {
                        mvprintw(start_y+1+(i as i32), start_x+1, &message[real_x_dim*(i)..real_x_dim*(i+1)]);
                    }
                }
            } else {
                mvprintw(start_y+1, start_x+1, &message);
            }
        },
        "List" | "L" => { // Display a list of items or options
            let list_data = message.split('|').collect::<Vec<&str>>();
            attron(A_UNDERLINE());
            for i in 0..list_data.len() {
                for j in 0..x_dim {
                    mvprintw(start_y+1+(i as i32), start_x+1+(j as i32), " ");
                }
                mvprintw(start_y+1+(i as i32), start_x+1, &list_data[i]);
            }
            attroff(A_UNDERLINE());
        },
        "Scoreboard" | "S" | "SB" | "Score" => { // Like a list, but you can pair numbers with it. Unsorted.
            let list_data = message.split('|').collect::<Vec<&str>>();
            attron(A_UNDERLINE());
            for i in 0..list_data.len() {
                for j in 0..x_dim {
                    mvprintw(start_y+1+(i as i32), start_x+1+(j as i32), " ");
                }
                let item_metric = &list_data[i].split('+').collect::<Vec<&str>>();
                if item_metric.len() >= 1 { // I guess I can display a name with no score on the scoreboard.
                    mvprintw(start_y+1+(i as i32), start_x+1, item_metric[0]);
                }
                attron(A_BOLD());
                if item_metric.len() == 2 { // The damn thing should be at most two values
                    mvprintw(start_y+1+(i as i32), start_x+x_dim-3, item_metric[1]);
                }
                attroff(A_BOLD());
            }
            attroff(A_UNDERLINE());
        },
        "ProgressBar" | "PB" | "ProgBar" => { // Display a bar of some sort in a window.
                                              // (Window heights of 1 work best).
            let metrics = message.split('|').collect::<Vec<&str>>();
            let lower = metrics[0].parse::<f32>().unwrap();
            let upper = metrics[1].parse::<f32>().unwrap();
            let absolute_progress = ((lower/upper)*(x_dim as f32)) as i32; // How far across the window the bar is
            attron(A_STANDOUT()); // Solid bar style. I guess TODO: Make more styles?
            for i in 0..absolute_progress {
                mvprintw(start_y+1, start_x+1+(i as i32), " ");
            }
            attroff(A_STANDOUT());
            // Print the value
            attron(A_BOLD());
            let progress_string = format!("|{}/{}|", lower, upper);
            mvprintw(start_y+y_dim+1, start_x+1, &progress_string);
            attroff(A_BOLD());
        },
        _ => { dbg!("Dawg something totally whack happened I guess. o7 to your debugging.");},
    }
    box_(win, 0, 0);
    wrefresh(win);
    attron(A_BOLD());
    let title = format!("|{}|", name);
    mvprintw(start_y, start_x+1, &title);
    attroff(A_BOLD());
}

// Opens a new window and keeps track of it in the window HashMap.
pub fn open_win(new_window: WindowData,
                windows: &mut HashMap<String, (WINDOW, WindowData)>,
                logbuffer: &mut Vec<String>) {
    let x_loc = new_window.x_pos;
    let y_loc = new_window.y_pos;
    let x_dim = new_window.width;
    let y_dim = new_window.height;
    let name = &new_window.id;
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
        draw_win(&new_window, win);
        windows.insert(name.to_string(), (win, new_window));
    } else {
        logbuffer.insert(0, "This window name is already taken!".to_string());
        showlog(&logbuffer);
    }
}

// Redraws the windows open on screen when anything changes that could expose hidden content such as a window closing. Shouldn't be used by the user.
pub fn redraw(windows: &mut HashMap<String, (WINDOW, WindowData)>) {
    for (_window,data) in windows {
        draw_win(&data.1, data.0);
    }
}

// Redraws the log.
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
        addstr(logbuffer.get(i).unwrap());
    }
}